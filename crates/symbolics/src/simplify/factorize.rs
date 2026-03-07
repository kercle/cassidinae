use crate::expr::{Expr, NormalizedExpr};
use crate::{expr, norm_expr};

pub(super) fn factorization_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        (
            norm_expr!(a_ * b__ + a_ * c__ + r___),
            expr!(a * (Mul[b] + Mul[c]) + Add[r]),
        ),
        (
            norm_expr!(a_ + a_ * b__ + r___),
            expr!(a * (1 + Mul[b]) + Add[r]),
        ),
    ]
}
