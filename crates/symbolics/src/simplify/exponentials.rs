use crate::{expr::NormExpr, hold_expr, norm_expr};

pub(super) fn exponentials_rules() -> Vec<(NormExpr, NormExpr)> {
    vec![
        (norm_expr!(Pow[Pow[a_, b_], c_]), hold_expr!(a ^ (b * c))),
        (norm_expr!(a_ ^ m_ * b_ ^ m_), hold_expr!((a * b) ^ m)),
    ]
}
