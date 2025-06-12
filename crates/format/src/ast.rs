use crate::LatexDisplay;
use symbolics::parser::ast::AstNode;

impl LatexDisplay for AstNode {
    fn to_latex(&self) -> String {
        ast_to_latex(self, None)
    }
}

fn greek_letter(name: &str) -> String {
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

fn operator_precedence(ast: &AstNode) -> Option<u32> {
    match ast {
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
    let precedence = operator_precedence(ast);

    use AstNode::*;
    match ast {
        Constant(value) => value.to_string(),
        NamedValue(name) => greek_letter(name),
        Negate(node) => {
            format!("-{}", ast_to_latex(node, precedence))
        }
        Add(lhs, rhs) => wrap_with_parentheses(
            format!(
                "{} + {}",
                ast_to_latex(lhs, precedence),
                ast_to_latex(rhs, precedence)
            ),
            precedence,
            parent_precedence,
        ),
        AddSeq(nodes) => {
            let mut add_str = nodes
                .iter()
                .map(|node| ast_to_latex(node, precedence))
                .collect::<Vec<_>>()
                .join(" + ");
            wrap_with_parentheses(add_str, precedence, parent_precedence)
        }
        Sub(lhs, rhs) => wrap_with_parentheses(
            format!(
                "{} - {}",
                ast_to_latex(lhs, precedence),
                ast_to_latex(rhs, precedence)
            ),
            precedence,
            parent_precedence,
        ),
        Mul(lhs, rhs) => {
            let lhs_str = ast_to_latex(lhs, precedence);
            let rhs_str = ast_to_latex(rhs, precedence);
            let mul_str = if lhs.is_constant() && rhs.is_constant() {
                format!("{} \\cdot {}", lhs_str, rhs_str)
            } else {
                format!("{} {}", lhs_str, rhs_str)
            };

            wrap_with_parentheses(mul_str, precedence, parent_precedence)
        }
        MulSeq(nodes) => {
            let mul_str = nodes
                .iter()
                .map(|node| ast_to_latex(node, precedence))
                .collect::<Vec<_>>()
                .join(" \\cdot ");
            wrap_with_parentheses(mul_str, precedence, parent_precedence)
        }
        Div(lhs, rhs) => {
            let frac_str = format!(
                "\\frac{{{}}}{{{}}}",
                ast_to_latex(lhs, None),
                ast_to_latex(rhs, None)
            );

            wrap_with_parentheses(frac_str, precedence, parent_precedence)
        }
        Pow(lhs, rhs) => {
            let pow_str = format!(
                "{}^{{{}}}",
                ast_to_latex(lhs, precedence),
                ast_to_latex(rhs, precedence)
            );

            wrap_with_parentheses(pow_str, precedence, parent_precedence)
        }
        Sin(node) => {
            format!("\\sin\\left({}\\right)", ast_to_latex(node, precedence))
        }
        Cos(node) => {
            format!("\\cos\\left({}\\right)", ast_to_latex(node, precedence))
        }
        Tan(node) => {
            format!("\\tan\\left({}\\right)", ast_to_latex(node, precedence))
        }
        Sqrt(node) => {
            format!("\\sqrt{{{}}}", ast_to_latex(node, precedence))
        }
        FunctionCall { name, args } => {
            let args_str = args
                .iter()
                .map(|arg| ast_to_latex(arg, None))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{name}\\left[{args_str}\\right]")
        }
        Block(nodes) => {
            let mut block_str = String::new();
            for node in nodes {
                if !block_str.is_empty() {
                    block_str.push_str(" \\\\\n");
                }
                block_str.push_str(&ast_to_latex(node, parent_precedence));
            }
            block_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LatexDisplay;
    use symbolics::parser::parse;

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
