use crate::expr::NormExpr;

#[derive(Clone, Debug)]
pub struct PatternDoc {
    pub pattern: String,
    pub summary: String,
}

impl PatternDoc {
    pub fn new<S: ToString, T: ToString>(pattern: S, summary: T) -> Self {
        Self {
            pattern: pattern.to_string(),
            summary: summary.to_string(),
        }
    }
}

pub trait BuiltIn {
    fn title(&self) -> String;

    fn head_symbol(&self) -> &'static str;

    fn summary(&self) -> &'static str;

    fn pattern_doc(&self) -> Vec<PatternDoc>;

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr
    }
}
