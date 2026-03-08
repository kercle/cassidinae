use crate::{expr::NormExpr, norm_expr};

pub(super) fn factorization_rules() -> Vec<(NormExpr, NormExpr)> {
    vec![
        (
            norm_expr!(a_ * b__ + a_ * c__ + r___),
            norm_expr!(a * (Mul[b] + Mul[c]) + Add[r]),
        ),
        (
            norm_expr!(a_ + a_ * b__ + r___),
            norm_expr!(a * (1 + Mul[b]) + Add[r]),
        ),
    ]
}
