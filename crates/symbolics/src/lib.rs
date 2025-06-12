pub mod parser;
pub mod simplify;

pub trait LatexDisplay {
    fn to_latex(&self) -> String;
}
