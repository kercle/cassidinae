use crate::expr::RawExpr;

mod input;
mod latex;

pub trait MathDisplay {
    fn to_latex_form(&self) -> String;
    fn to_input_form(&self) -> String;
}

impl MathDisplay for RawExpr {
    fn to_latex_form(&self) -> String {
        latex::render(self)
    }

    fn to_input_form(&self) -> String {
        input::render(self)
    }
}
