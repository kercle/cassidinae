use crate::expr::{Expr, NormalizedExpr};
use crate::{expr, norm_expr};

pub(super) fn factorization_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        (
            norm_expr!(
                Pattern[r, BlankNullSeq[]]
                    + Pattern[a, Blank[]] * Pattern[b, BlankSeq[]]
                    + Pattern[a, Blank[]] * Pattern[c, BlankSeq[]]
            ),
            expr!(a * (Mul[b] + Mul[c]) + Add[r]),
        ),
        (
            norm_expr!(
                Pattern[r, BlankNullSeq[]]
                    + Pattern[a, Blank[]]
                    + Pattern[a, Blank[]] * Pattern[b, BlankSeq[]]
            ),
            expr!(a * (1 + Mul[b]) + Add[r]),
        ),
    ]
}
