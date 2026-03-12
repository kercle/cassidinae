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
    fn category(&self) -> &'static str;

    fn title(&self) -> &'static str;

    fn head_symbol(&self) -> &'static str;

    fn summary(&self) -> &'static str;

    fn pattern_doc(&self) -> Vec<PatternDoc>;

    fn examples(&self) -> Vec<(&'static str, &'static str)>;

    fn related(&self) -> Vec<&'static str>;

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr
    }
}
