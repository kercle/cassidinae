pub mod ast;
pub mod error;
pub mod lex;

use numbers::RealScalar;

use crate::parser::{
    ast::AstNode,
    error::{BoxedError, ParseError},
    lex::{Token, TokenStream},
};

fn parse_identifier_or_call(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <named_value_or_function_call> ::= <identifier>
    //    | <identifier> "(" ")"
    //    | <identifier> "(" <block> { "," <block> }* ")"

    let start_token = stream.peek().cloned();
    let identifier = stream.next_if_identifier();

    if identifier.is_none() {
        return Err(ParseError {
            message: "Expected an identifier".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    let identifier = identifier.unwrap().to_owned();

    if stream.next_if_matches_token(&Token::LeftBracket).is_none() {
        return Ok(AstNode::NamedValue(identifier.to_string()));
    }

    if stream.next_if_matches_token(&Token::RightBracket).is_some() {
        return AstNode::from_function_call(identifier, vec![]).map_err(|message| ParseError {
            message,
            at_token: start_token,
        });
    }

    let mut args = vec![parse_block(stream)?];
    loop {
        if stream.next_if_matches_token(&Token::Comma).is_none() {
            break; // No more arguments
        }

        args.push(parse_block(stream)?);
    }

    if stream.next_if_matches_token(&Token::RightBracket).is_none() {
        return Err(ParseError {
            message: "Expected closing parenthesis ')' after function arguments".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    AstNode::from_function_call(identifier, args).map_err(|message| ParseError {
        message,
        at_token: start_token,
    })
}

fn parse_atom(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <atom> ::= <number>
    //    | "(" <block> ")"
    //    | <named_value_or_function_call>

    if stream.next_if_matches_token(&Token::LeftParen).is_some() {
        let expr = parse_block(stream)?;

        if stream.next_if_matches_token(&Token::RightParen).is_some() {
            return Ok(expr);
        } else {
            return Err(ParseError {
                message: "Expected closing parenthesis ')'".to_string(),
                at_token: stream.peek().cloned(),
            });
        }
    } else if let Some(value) = stream.next_if_number() {
        let scalar = RealScalar::from_str(value).map_err(|e| ParseError {
            message: e,
            at_token: stream.peek().cloned(),
        })?;
        Ok(AstNode::Constant(scalar))
    } else {
        parse_identifier_or_call(stream)
    }
}

fn parse_power(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <power> ::= <atom> { "^" <power> }

    let mut result = parse_atom(stream)?;

    if stream.next_if_matches_token(&Token::Caret).is_some() {
        result = AstNode::Pow(Box::new(result), Box::new(parse_power(stream)?));
    }

    Ok(result)
}

fn parse_signed_power(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <signed_power> ::= { "+" | "-" }* power

    let mut negate_count = 0;

    while let Some(sign) =
        stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus))
    {
        if *sign == Token::Minus {
            negate_count += 1;
        }
    }

    let ast = parse_power(stream)?;

    if negate_count % 2 == 1 {
        Ok(AstNode::Negate(Box::new(ast)))
    } else {
        Ok(ast)
    }
}

fn parse_product(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <product> ::= <match_signed_power> { ("*"|"/") <match_signed_power> }*

    let mut result = parse_signed_power(stream)?;

    loop {
        let c = stream.next_if_matches(|token| matches!(token, Token::Asterisk | Token::Slash));

        if c.is_none() {
            break; // No more operators
        }

        result = match c {
            Some(Token::Asterisk) => {
                AstNode::Mul(Box::new(result), Box::new(parse_signed_power(stream)?))
            }
            Some(Token::Slash) => {
                AstNode::Div(Box::new(result), Box::new(parse_signed_power(stream)?))
            }
            _ => unreachable!(),
        };
    }

    Ok(result)
}

fn parse_sum(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <sum> ::= <product> { ("+"|"-") <product> }*

    let mut result = parse_product(stream)?;

    loop {
        let op = stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus));
        if op.is_none() {
            break; // No more operators
        }

        result = match op {
            Some(Token::Plus) => AstNode::Add(Box::new(result), Box::new(parse_product(stream)?)),
            Some(Token::Minus) => AstNode::Sub(Box::new(result), Box::new(parse_product(stream)?)),
            _ => unreachable!(),
        };
    }

    Ok(result)
}

fn parse_expression(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <expression> ::= <sum>

    parse_sum(stream)
}

fn parse_block(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <block> ::= <expression> { ";" <expression> }*
    //    | "{" <block> "}"

    let mut nodes = Vec::new();

    if stream.next_if_matches_token(&Token::LeftBrace).is_some() {
        let block = parse_block(stream)?;
        if stream.next_if_matches_token(&Token::RightBrace).is_none() {
            return Err(ParseError {
                message: "Expected closing brace '}'".to_string(),
                at_token: stream.peek().cloned(),
            });
        }

        Ok(block)
    } else {
        loop {
            let expr = parse_expression(stream)?;
            nodes.push(expr);

            if stream.next_if_matches_token(&Token::Semicolon).is_none() {
                break; // No more expressions
            }
        }

        if nodes.is_empty() {
            Err(ParseError {
                message: "Expected at least one expression in the block".to_string(),
                at_token: stream.peek().cloned(),
            })
        } else if nodes.len() == 1 {
            Ok(nodes.pop().unwrap())
        } else {
            Ok(AstNode::Block(nodes))
        }
    }
}

pub fn parse(input: &str) -> Result<AstNode, BoxedError> {
    let mut stream = TokenStream::from_str(input)?;

    let ast = parse_block(&mut stream);

    if ast.is_ok() && !stream.end_of_stream() {
        return Err(ParseError {
            message: "Unexpected tokens at the end of input".to_string(),
            at_token: stream.peek().cloned(),
        }
        .into());
    }

    match ast {
        Ok(node) => Ok(node),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::ast::AstNode::*;
    use super::*;

    #[test]
    fn test_parse_expression_long() {
        let input = "3 + 4 * 2 - (1 + 5) ^ 2 ^ 3";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Sub(
                Add(
                    Constant(RealScalar::from_str("3").unwrap()).into(),
                    Mul(
                        Constant(RealScalar::from_str("4").unwrap()).into(),
                        Constant(RealScalar::from_str("2").unwrap()).into()
                    )
                    .into()
                )
                .into(),
                Pow(
                    Add(
                        Constant(RealScalar::from_str("1").unwrap()).into(),
                        Constant(RealScalar::from_str("5").unwrap()).into()
                    )
                    .into(),
                    Pow(
                        Constant(RealScalar::from_str("2").unwrap()).into(),
                        Constant(RealScalar::from_str("3").unwrap()).into()
                    )
                    .into()
                )
                .into()
            )
        );
    }

    #[test]
    fn test_parse_expression_short() {
        let input = "3+ 5 -  2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Sub(
                Add(
                    Constant(RealScalar::from_str("3").unwrap()).into(),
                    Constant(RealScalar::from_str("5").unwrap()).into()
                )
                .into(),
                Constant(RealScalar::from_str("2").unwrap()).into()
            )
        );
    }

    #[test]
    fn test_parse_expression_with_float() {
        let input = "3.14 + 2.71";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Add(
                Constant(RealScalar::from_str("3.14").unwrap()).into(),
                Constant(RealScalar::from_str("2.71").unwrap()).into()
            )
        );
    }

    #[test]
    fn test_parse_expression_with_invalid_input() {
        let input = "3 + * 5";
        let ast = parse(input);

        assert!(ast.is_err(), "Expected parse to fail with invalid input");
    }

    #[test]
    fn test_parse_expression_with_products() {
        let input = "3 + 5 * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Add(
                Constant(RealScalar::from_str("3").unwrap()).into(),
                Mul(
                    Constant(RealScalar::from_str("5").unwrap()).into(),
                    Constant(RealScalar::from_str("2").unwrap()).into()
                )
                .into()
            )
        );
    }

    #[test]
    fn test_parse_expression_with_powers() {
        let input = "7 * 2^3 + 4";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Add(
                Mul(
                    Constant(RealScalar::from_str("7").unwrap()).into(),
                    Pow(
                        Constant(RealScalar::from_str("2").unwrap()).into(),
                        Constant(RealScalar::from_str("3").unwrap()).into()
                    )
                    .into()
                )
                .into(),
                Constant(RealScalar::from_str("4").unwrap()).into()
            )
        );
    }

    #[test]
    fn test_parse_expression_with_parentheses() {
        let input = "(3 + 5) * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Mul(
                Add(
                    Constant(RealScalar::from_str("3").unwrap()).into(),
                    Constant(RealScalar::from_str("5").unwrap()).into()
                )
                .into(),
                Constant(RealScalar::from_str("2").unwrap()).into()
            )
        );
    }

    #[test]
    fn test_parse_expression_with_nested_parentheses() {
        let input = "3 + (2 * (5 - 1))^2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Add(
                Constant(RealScalar::from_str("3").unwrap()).into(),
                Pow(
                    Mul(
                        Constant(RealScalar::from_str("2").unwrap()).into(),
                        Sub(
                            Constant(RealScalar::from_str("5").unwrap()).into(),
                            Constant(RealScalar::from_str("1").unwrap()).into()
                        )
                        .into()
                    )
                    .into(),
                    Constant(RealScalar::from_str("2").unwrap()).into()
                )
                .into()
            )
        );
    }

    #[test]
    fn test_parse_expression_with_function_call() {
        let input = "5*pi^2/4*cos[pi*x/2]*sin[pi*y/2]";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Mul(
                Mul(
                    Div(
                        Mul(
                            Constant(RealScalar::from_str("5").unwrap()).into(),
                            Pow(
                                NamedValue("pi".to_string()).into(),
                                Constant(RealScalar::from_str("2").unwrap()).into()
                            )
                            .into()
                        )
                        .into(),
                        Constant(RealScalar::from_str("4").unwrap()).into()
                    )
                    .into(),
                    Cos(Div(
                        Mul(
                            NamedValue("pi".to_string()).into(),
                            NamedValue("x".to_string()).into()
                        )
                        .into(),
                        Constant(RealScalar::from_str("2").unwrap()).into()
                    )
                    .into())
                    .into()
                )
                .into(),
                Sin(Div(
                    Mul(
                        NamedValue("pi".to_string()).into(),
                        NamedValue("y".to_string()).into(),
                    )
                    .into(),
                    Constant(RealScalar::from_str("2").unwrap()).into()
                )
                .into())
                .into()
            )
        );
    }

    #[test]
    fn test_parsing_block_with_nested_expressions() {
        let input = "{ 3 + 4; 5 * 6 }";
        let ast = parse(input).expect("Failed to parse block with nested expressions");

        assert_eq!(
            ast,
            Block(vec![
                Add(
                    Constant(RealScalar::from_str("3").unwrap()).into(),
                    Constant(RealScalar::from_str("4").unwrap()).into()
                ),
                Mul(
                    Constant(RealScalar::from_str("5").unwrap()).into(),
                    Constant(RealScalar::from_str("6").unwrap()).into()
                )
            ])
        );
    }
}
