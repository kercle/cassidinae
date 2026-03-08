use crate::expr::RawExpr;

mod latex;

pub trait MathDisplay {
    fn to_latex(&self) -> String;
}

impl MathDisplay for RawExpr {
    fn to_latex(&self) -> String {
        latex::expr_to_latex(self, None)
    }
}
