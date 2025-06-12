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

    fn fancy_name(name: &str) -> String {
        match name {
            "alpha" => "\\alpha".to_string(),
            "beta" => "\\beta".to_string(),
            "gamma" => "\\gamma".to_string(),
            "delta" => "\\delta".to_string(),
            "epsilon" => "\\epsilon".to_string(),
            "zeta" => "\\zeta".to_string(),
            "eta" => "\\eta".to_string(),
            "theta" => "\\theta".to_string(),
            "iota" => "\\iota".to_string(),
            "kappa" => "\\kappa".to_string(),
            "lambda" => "\\lambda".to_string(),
            "mu" => "\\mu".to_string(),
            "nu" => "\\nu".to_string(),
            "xi" => "\\xi".to_string(),
            "omicron" => "\\omicron".to_string(),
            "pi" => "\\pi".to_string(),
            "rho" => "\\rho".to_string(),
            "sigma" => "\\sigma".to_string(),
            "tau" => "\\tau".to_string(),
            "upsilon" => "\\upsilon".to_string(),
            "phi" => "\\phi".to_string(),
            "chi" => "\\chi".to_string(),
            "psi" => "\\psi".to_string(),
            "omega" => "\\omega".to_string(),
            _ => name.to_string(),
        }
    }

    fn operator_precedence(self: &AstNode) -> Option<u32> {
        match self {
            AstNode::Negate(_) => Some(3),
            AstNode::Add(_, _) => Some(1),
            AstNode::Sub(_, _) => Some(1),
            AstNode::Mul(_, _) => Some(2),
            AstNode::Div(_, _) => Some(2),
            AstNode::Pow(_, _) => Some(4),
            _ => None,
        }
    }

    fn wrap_with_parentheses(
        sub_tree_str: String,
        precedence: Option<u32>,
        parent_precedence: Option<u32>,
    ) -> String {
        if parent_precedence > precedence {
            format!("\\left({}\\right)", sub_tree_str)
        } else {
            sub_tree_str
        }
    }

    fn ast_to_latex(ast: &AstNode, parent_precedence: Option<u32>) -> String {
        let precedence = ast.operator_precedence();

        match ast {
            AstNode::Constant(value) => value.to_string(),
            AstNode::NamedValue(name) => Self::fancy_name(name),
            AstNode::Negate(node) => {
                format!("-{}", Self::ast_to_latex(node, precedence))
            }
            AstNode::Add(lhs, rhs) => Self::wrap_with_parentheses(
                format!(
                    "{} + {}",
                    Self::ast_to_latex(lhs, precedence),
                    Self::ast_to_latex(rhs, precedence)
                ),
                precedence,
                parent_precedence,
            ),
            AstNode::Sub(lhs, rhs) => Self::wrap_with_parentheses(
                format!(
                    "{} - {}",
                    Self::ast_to_latex(lhs, precedence),
                    Self::ast_to_latex(rhs, precedence)
                ),
                precedence,
                parent_precedence,
            ),
            AstNode::Mul(lhs, rhs) => {
                let lhs_str = Self::ast_to_latex(lhs, precedence);
                let rhs_str = Self::ast_to_latex(rhs, precedence);

                let sub_tree_disp = if lhs.is_numeric() && rhs.is_numeric() {
                    format!("{} \\cdot {}", lhs_str, rhs_str)
                } else {
                    format!("{} {}", lhs_str, rhs_str)
                };

                Self::wrap_with_parentheses(sub_tree_disp, precedence, parent_precedence)
            }
            AstNode::Div(lhs, rhs) => {
                let lhs_str = Self::ast_to_latex(lhs, precedence);
                let rhs_str = Self::ast_to_latex(rhs, precedence);

                let sub_tree_disp = format!("\\frac{{{}}}{{{}}}", lhs_str, rhs_str);
                Self::wrap_with_parentheses(sub_tree_disp, precedence, parent_precedence)
            }
            AstNode::Pow(lhs, rhs) => {
                let lhs_str = Self::ast_to_latex(lhs, precedence);
                let rhs_str = Self::ast_to_latex(rhs, precedence);

                let sub_tree_disp = format!("{}^{{{}}}", lhs_str, rhs_str);
                Self::wrap_with_parentheses(sub_tree_disp, precedence, parent_precedence)
            }
            AstNode::Sin(node) => {
                let node_str = Self::ast_to_latex(node, precedence);
                format!("\\sin\\left({}\\right)", node_str)
            }
            AstNode::Cos(node) => {
                let node_str = Self::ast_to_latex(node, precedence);
                format!("\\cos\\left({}\\right)", node_str)
            }
            AstNode::Tan(node) => {
                let node_str = Self::ast_to_latex(node, precedence);
                format!("\\tan\\left({}\\right)", node_str)
            }
            AstNode::Sqrt(node) => {
                let node_str = Self::ast_to_latex(node, precedence);
                format!("\\sqrt{{{}}}", node_str)
            }
            AstNode::FunctionCall { name, args } => {
                let mut args_disp = Vec::new();

                for arg in args {
                    args_disp.push(Self::ast_to_latex(arg, None));
                }

                let mut lbracket = "\\left(".to_string();
                let mut rbracket = "\\right)".to_string();

                let name = match name.as_str() {
                    "sin" => "\\sin".to_string(),
                    "cos" => "\\cos".to_string(),
                    "tan" => "\\tan".to_string(),
                    "sqrt" => {
                        lbracket = "{".to_string();
                        rbracket = "}".to_string();
                        "\\sqrt".to_string()
                    }
                    _ => format!("\\operatorname{{{}}}", name),
                };

                format!("{name}{lbracket}{}{rbracket}", args_disp.join(", "))
            }
            AstNode::Block(nodes) => {
                let mut block_str = String::new();
                for node in nodes {
                    if !block_str.is_empty() {
                        block_str.push_str(" \\\\\n");
                    }
                    block_str.push_str(&Self::ast_to_latex(node, parent_precedence));
                }
                block_str
            }
        }
    }

    pub fn to_latex(&self) -> String {
        Self::ast_to_latex(self, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_ast_to_latex() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(ast.to_latex(), "2 + 3");
    }

    #[test]
    fn test_ast_to_latex_with_parenthesis() {
        let ast = parse("(2 + 3) * 6").unwrap();
        assert_eq!(ast.to_latex(), "\\left(2 + 3\\right) 6");
    }

    #[test]
    fn test_ast_to_latex_multiple_adds() {
        let ast = parse("1+2+3+4").unwrap();
        assert_eq!(ast.to_latex(), "1 + 2 + 3 + 4");
    }

    #[test]
    fn test_ast_to_latex_with_unary_op() {
        let ast = parse("-2 + 3").unwrap();
        assert_eq!(ast.to_latex(), "-2 + 3");
    }

    #[test]
    fn test_ast_to_latex_with_function_call() {
        let ast = parse("5*pi^2/4*cos[pi*x/2]*sin[pi*y/2]").unwrap();
        assert_eq!(
            ast.to_latex(),
            "\\frac{5 \\pi^{2}}{4} \\cos\\left(\\frac{\\pi x}{2}\\right) \\sin\\left(\\frac{\\pi y}{2}\\right)"
        );
    }
}
