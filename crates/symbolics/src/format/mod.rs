use crate::expr::Expr;

mod latex;

pub trait MathDisplay {
    fn to_latex(&self) -> String;
}

impl MathDisplay for Expr {
    fn to_latex(&self) -> String {
        latex::expr_to_latex(self, None)
    }
}
