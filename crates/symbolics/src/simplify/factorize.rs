use crate::{expr::NormExpr, norm_expr};

pub(super) fn factorization_rules() -> Vec<(NormExpr, NormExpr)> {
    vec![
        (
            norm_expr!(a_^2 + 2*a_*b_ + b_^2 + r___),
            norm_expr!((a + b)^2 + Add[r]),
        ),
        (
            norm_expr!(a_^2 - 2*a_*b_ + b_^2 + r___),
            norm_expr!((a - b)^2 + Add[r]),
        ),
        (
            norm_expr!(a_ * b__ + a_ * c__ + r___),
            norm_expr!(a * (Mul[b] + Mul[c]) + Add[r]),
        ),
        (
            norm_expr!(a_ + a_ * b__ + r___),
            norm_expr!(a * (1 + Mul[b]) + Add[r]),
        ),
        (
            norm_expr!(a_^2 - b_^2 + r___),
            norm_expr!((a + b)*(a - b) + Add[r]),
        ),
    ]
}
