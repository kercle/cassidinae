use crate::builtins::traits::{BuiltIn, PatternDoc};

#[derive(Default)]
pub struct Help;

impl BuiltIn for Help {
    fn category(&self) -> &'static str {
        "System"
    }

    fn title(&self) -> &'static str {
        "Help"
    }

    fn head_symbol(&self) -> &'static str {
        "Help"
    }

    fn summary(&self) -> &'static str {
        "Documentation for builtin functionality."
    }

    fn examples(&self) -> Vec<(&'static str, &'static str)> {
        vec![]
    }

    fn related(&self) -> Vec<&'static str> {
        vec![]
    }

    fn pattern_doc(&self) -> Vec<PatternDoc> {
        vec![
            PatternDoc::new(
                "Help[]",
                "Print table of contents with all built-in symbols.",
            ),
            PatternDoc::new(
                "Help[s_?IsSymbol]",
                "Specific documentation of the given symbol.",
            ),
        ]
    }
}
