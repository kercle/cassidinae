use crate::expr::{Expr, NormalizedExpr};
use crate::{expr, norm_expr};

pub(super) fn known_function_values_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Sin exact values ===============
        (norm_expr!(Sin[0]), expr!(0)),
        (norm_expr!(Sin[pi / 12]), expr!(Sqrt[2] * (Sqrt[3] - 1) / 4)),
        (norm_expr!(Sin[pi / 10]), expr!((Sqrt[5] - 1) / 4)),
        (norm_expr!(Sin[pi / 8]), expr!(Sqrt[2 - Sqrt[2]] / 2)),
        (norm_expr!(Sin[pi / 6]), expr!(1 / 2)),
        (
            norm_expr!(Sin[pi / 5]),
            expr!(Sqrt[2] * Sqrt[5 - Sqrt[5]] / 4),
        ),
        (norm_expr!(Sin[pi / 4]), expr!(1 / Sqrt[2])),
        // =============== Cos exact values ===============
        (norm_expr!(Cos[0]), expr!(1)),
        (norm_expr!(Cos[pi / 12]), expr!(Sqrt[2] * (Sqrt[3] + 1) / 4)),
        (
            norm_expr!(Cos[pi / 10]),
            expr!(Sqrt[2] * Sqrt[5 + Sqrt[5]] / 4),
        ),
        (norm_expr!(Cos[pi / 8]), expr!(Sqrt[2 + Sqrt[2]] / 2)),
        (norm_expr!(Cos[pi / 6]), expr!(Sqrt[3] / 2)),
        (norm_expr!(Cos[pi / 5]), expr!((Sqrt[5] + 1) / 4)),
        (norm_expr!(Cos[pi / 4]), expr!(1 / Sqrt[2])),
        // =============== Tan exact values ===============
        (norm_expr!(Tan[0]), expr!(0)),
        // =============== Exp exact values ===============
        (norm_expr!(Exp[0]), expr!(1)),
        // =============== Log exact values ===============
        (norm_expr!(Log[0]), expr! { -Infinity }),
        (norm_expr!(Log[1]), expr!(0)),
    ]
}
