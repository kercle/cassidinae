pub mod ast;

pub trait LatexDisplay {
    fn to_latex(&self) -> String;
}
