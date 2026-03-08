use crate::{expr::NormExpr, norm_expr};

pub(super) fn trigonometric_rules() -> Vec<(NormExpr, NormExpr)> {
    vec![
        // =============== Pythagorean identity ===============
        (
            norm_expr!(Cos[a_] ^ 2 + Sin[a_] ^ 2 + r___),
            norm_expr!(1 + r),
        ),
        (norm_expr!(Sqrt[1 - Cos[x_] ^ 2]), norm_expr!(Sin[x])),
        (norm_expr!(Sqrt[1 - Sin[x_] ^ 2]), norm_expr!(Cos[x])),
    ]
}
