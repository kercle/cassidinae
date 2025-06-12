use numbers::RealScalar;

pub trait Operator {
    fn precedence(&self) -> u8;
    fn is_left_associative(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
}

impl Operator for UnaryOp {
    fn precedence(&self) -> u8 {
        3
    }

    fn is_left_associative(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator for BinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Sub => 1,
            BinaryOp::Mul | BinaryOp::Div => 2,
            BinaryOp::Pow => 4,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => true,
            BinaryOp::Pow => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Constant(RealScalar),
    NamedValue(String),
    Negate(Box<AstNode>),
    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Pow(Box<AstNode>, Box<AstNode>),
    Sin(Box<AstNode>),
    Cos(Box<AstNode>),
    Tan(Box<AstNode>),
    Sqrt(Box<AstNode>),
    FunctionCall { name: String, args: Vec<AstNode> },
    Block(Vec<AstNode>),
}

impl AstNode {
    pub fn from_function_call(name: String, mut args: Vec<AstNode>) -> Result<Self, String> {
        let initial_args_len = args.len();

        let result = match name.as_str() {
            "sin" => Ok(AstNode::Sin(Box::new(
                args.pop().ok_or("sin requires one argument")?,
            ))),
            "cos" => Ok(AstNode::Cos(Box::new(
                args.pop().ok_or("cos requires one argument")?,
            ))),
            "tan" => Ok(AstNode::Tan(Box::new(
                args.pop().ok_or("tan requires one argument")?,
            ))),
            "sqrt" => Ok(AstNode::Sqrt(Box::new(
                args.pop().ok_or("sqrt requires one argument")?,
            ))),
            _ => {
                return Ok(AstNode::FunctionCall {
                    name: name.clone(),
                    args: args,
                });
            }
        };

        if !args.is_empty() {
            let expected_arg_count = initial_args_len - args.len();

            let arguments = if expected_arg_count == 1 {
                "argument"
            } else {
                "arguments"
            };

            return Err(format!(
                "Function {} takes {} {arguments} but {} given.",
                name,
                initial_args_len - args.len(),
                initial_args_len
            ));
        }

        result
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, AstNode::Constant(_))
    }
}
