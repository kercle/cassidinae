use std::{iter::Peekable, str::CharIndices};

use numbers::rational::Rational;

use crate::ast::{AstNode, BinaryOp, UnaryOp};

#[derive(Debug)]
struct ParserState<'a> {
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> ParserState<'a> {
    fn new(slice: &'a str) -> Self {
        Self {
            iter: slice.char_indices().peekable(),
        }
    }

    fn advance(&mut self) {
        self.iter.next();
    }

    fn consume_pattern<F>(&mut self, matcher: F, skip_whitespaces: bool) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        if skip_whitespaces {
            self.consume_whitespaces();
        }

        if self.is_at_end() {
            return None;
        }

        if let Some((_, c)) = self.peek() {
            if matcher(c) {
                self.advance();
                return Some(c);
            }
        }

        None
    }

    fn consume_char(&mut self, c: char, skip_whitespaces: bool) -> Option<char> {
        self.consume_pattern(|ch| ch == c, skip_whitespaces)
    }

    fn consume_digit(&mut self, skip_whitespaces: bool) -> Option<char> {
        self.consume_pattern(|c| c.is_ascii_digit(), skip_whitespaces)
    }

    fn consume_letter_or_underscore(&mut self, skip_whitespaces: bool) -> Option<char> {
        self.consume_pattern(|c| c.is_alphabetic() || c == '_', skip_whitespaces)
    }

    fn consume_alphanumeric_or_underscore(&mut self, skip_whitespaces: bool) -> Option<char> {
        self.consume_pattern(|c| c.is_alphanumeric() || c == '_', skip_whitespaces)
    }

    fn consume_whitespaces(&mut self) {
        while self.consume_pattern(char::is_whitespace, false).is_some() {}
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.iter.peek().copied()
    }

    fn is_at_end(&mut self) -> bool {
        self.iter.peek().is_none()
    }

    fn cursor_pos(&mut self) -> Option<usize> {
        self.peek().map(|(pos, _)| pos)
    }
}

fn parse_number(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <digit>+ ("." <digit>*)? ( "e" ["+"|"-"] <digit>+ )?

    let mut matched_string = String::new();

    state.consume_whitespaces();
    while let Some(c) = state.consume_digit(false) {
        matched_string.push(c);
    }

    if matched_string.is_empty() {
        return Err("Expected a digit".to_string());
    }

    if state.consume_char('.', false).is_some() {
        matched_string.push('.');
        while let Some(c) = state.consume_digit(false) {
            matched_string.push(c);
        }
    }

    if state
        .consume_pattern(|c| c == 'e' || c == 'E', false)
        .is_some()
    {
        matched_string.push('e');

        if let Some(c) = state.consume_pattern(|c| c == '+' || c == '-', false) {
            matched_string.push(c);
        }

        if let Some(c) = state.consume_digit(false) {
            matched_string.push(c);
        } else {
            return Err("Expected digits after exponent".to_string());
        }

        while let Some(c) = state.consume_digit(false) {
            matched_string.push(c);
        }
    }

    let parsed_number = Rational::from_decimal_str(&matched_string)
        .map_err(|_| "Failed to parse number".to_string())?;
    Ok(AstNode::Number(numbers::RealScalar::Rational(parsed_number)))
}

fn parse_identifier(state: &mut ParserState<'_>) -> Result<String, String> {
    // <identifier> ::= <letter> { <letter> | <digit> }*

    let mut matched_string = String::new();

    if let Some(c) = state.consume_letter_or_underscore(true) {
        matched_string.push(c);
    } else {
        return Err("Expected a letter to start an identifier".to_string());
    }

    while let Some(c) = state.consume_alphanumeric_or_underscore(false) {
        matched_string.push(c);
    }

    Ok(matched_string)
}

fn parse_named_value_or_function_call(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <named_value_or_function_call> ::= <identifier>
    //    | <identifier> "(" ")"
    //    | <identifier> "(" <expression> { "," <expression> }* ")"

    let identifier = parse_identifier(state)?;

    if state.consume_char('(', true).is_none() {
        return Ok(AstNode::NamedValue(identifier));
    }

    if state.consume_char(')', true).is_some() {
        return Ok(AstNode::FunctionCall {
            name: identifier,
            args: Vec::new(),
        });
    }

    let mut args = vec![parse_expression(state)?];
    loop {
        if state.consume_char(',', true).is_none() {
            break; // No more arguments
        }

        args.push(parse_expression(state)?);
    }

    if state.consume_char(')', true).is_none() {
        return Err("Expected closing parenthesis ')'".to_string());
    }

    Ok(AstNode::FunctionCall {
        name: identifier,
        args,
    })
}

fn parse_atom(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <atom> ::= <number>
    //    | "(" <expression> ")"
    //    | <named_value_or_function_call>

    if state.consume_char('(', true).is_some() {
        let expr = parse_expression(state)?;

        if state.consume_char(')', true).is_some() {
            return Ok(expr);
        } else {
            return Err("Expected closing parenthesis ')'".to_string());
        }
    }

    let Some((_, c)) = state.peek() else {
        return Err("Unexpected end of input".to_string());
    };

    if c.is_alphabetic() {
        parse_named_value_or_function_call(state)
    } else {
        parse_number(state)
    }
}

fn parse_power(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <power> ::= <atom> { "^" <power> }

    let mut result = parse_atom(state)?;

    if state.consume_char('^', true).is_some() {
        result = AstNode::BinaryOp {
            op: BinaryOp::Pow,
            lhs: Box::new(result),
            rhs: Box::new(parse_power(state)?),
        };
    }

    Ok(result)
}

fn parse_signed_power(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <signed_power> ::= { "+" | "-" }* power

    let mut negate_count = 0;

    while let Some(sign) = state.consume_pattern(|c| matches!(c, '+' | '-'), true) {
        if sign == '-' {
            negate_count += 1; // Toggle sign
        }
    }

    let ast = parse_power(state)?;

    if negate_count % 2 == 1 {
        Ok(AstNode::UnaryOp {
            op: UnaryOp::Negate,
            expr: Box::new(ast),
        })
    } else {
        Ok(ast)
    }
}

fn parse_product(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <product> ::= <match_signed_power> { ("*"|"/") <match_signed_power> }*

    let mut result = parse_signed_power(state)?;

    loop {
        let c = state.consume_pattern(|c| matches!(c, '*' | '/'), true);
        if c.is_none() {
            break; // No more operators
        }

        result = AstNode::BinaryOp {
            op: match c {
                Some('*') => BinaryOp::Mul,
                Some('/') => BinaryOp::Div,
                _ => unreachable!(),
            },
            lhs: Box::new(result),
            rhs: Box::new(parse_signed_power(state)?),
        };
    }

    Ok(result)
}

fn parse_sum(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <sum> ::= <product> { ("+"|"-") <product> }*

    let mut result = parse_product(state)?;

    loop {
        let op = state.consume_pattern(|c| c == '+' || c == '-', true);
        if op.is_none() {
            break; // No more operators
        }

        result = AstNode::BinaryOp {
            op: match op {
                Some('+') => BinaryOp::Add,
                Some('-') => BinaryOp::Sub,
                _ => unreachable!(),
            },
            lhs: Box::new(result),
            rhs: Box::new(parse_product(state)?),
        };
    }

    Ok(result)
}

fn parse_expression(state: &mut ParserState<'_>) -> Result<AstNode, String> {
    // <expression> ::= <sum>

    parse_sum(state)
}

pub fn parse(input: &str) -> Result<AstNode, String> {
    let mut state = ParserState::new(input);
    let ast = parse_expression(&mut state);

    if ast.is_ok() && !state.is_at_end() {
        return Err(format!(
            "Error at position {}: Unexpected characters after expression",
            state.cursor_pos().unwrap_or(input.len())
        ));
    }

    match ast {
        Ok(ast) => Ok(ast),
        Err(err) => Err(format!(
            "Error at position {}: {}",
            state.cursor_pos().unwrap_or(input.len()),
            err
        )),
    }
}
