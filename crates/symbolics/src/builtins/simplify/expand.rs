use crate::{
    builtins::traits::{BuiltIn, PatternDoc},
    expr::{ExprKind, NormExpr, RawExpr},
    hold_expr, norm_expr,
    pattern::environment::Environment,
    rewrite::Rewriter,
};

const EXPAND_HEAD_SYMBOL: &'static str = "Expand";

pub struct Expand {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Expand {
    pub fn new() -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new("Expand[t_]", "Expands the given term $t$.")],
            rewriter: build_rewriter(),
        }
    }
}

impl Default for Expand {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltIn for Expand {
    fn title(&self) -> String {
        "Term expansion".to_string()
    }

    fn head_symbol(&self) -> &'static str {
        "Expand"
    }

    fn summary(&self) -> &'static str {
        "Expand factors."
    }

    fn pattern_doc(&self) -> Vec<PatternDoc> {
        self.pattern_doc.clone()
    }

    fn apply_all(&self, mut expr: NormExpr) -> NormExpr {
        let last_fingerprint = expr.fingerprint();

        expr.rewrite_all(&self.rewriter, 20)
    }
}

pub(super) fn build_rewriter() -> Rewriter {
    let rules = vec![
        (
            norm_expr!(Expand[a_ + b__]),
            hold_expr!(Expand[a] + Expand[Add[b]]),
        ),
        (
            norm_expr!(Expand[(a_ + b__) ^ 2]),
            hold_expr!(Expand[a ^ 2 + 2 * a * Add[b] + Add[b] ^ 2]),
        ),
        (
            norm_expr!(Expand[(a_ + b__) ^ 3]),
            hold_expr!(Expand[a ^ 3 + 3 * a ^ 2 * Add[b] + 3 * a * Add[b] ^ 2 + Add[b] ^ 3]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__)]),
            hold_expr!(Expand[Mul[a] * b] + Expand[Mul[a] * Add[c]]),
        ),
        (
            norm_expr!(Expand[a_ * b__]),
            hold_expr!(Expand[a] * Expand[Mul[b]]),
        ),
        (norm_expr!(Expand[a_]), hold_expr!(a)),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }))
}
