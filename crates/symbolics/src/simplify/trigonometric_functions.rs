use crate::expr::{Expr, NormalizedExpr};
use crate::{expr, norm_expr};

pub(super) fn trigonometric_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Pythagorean identity ===============
        (norm_expr!(Cos[a_] ^ 2 + Sin[a_] ^ 2 + r___), expr!(1 + r)),
        (norm_expr!(Sqrt[1 - Cos[x_] ^ 2]), expr!(Sin[x])),
        (norm_expr!(Sqrt[1 - Sin[x_] ^ 2]), expr!(Cos[x])),
    ]
}
