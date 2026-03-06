use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
};

pub fn trigonometric_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Pythagorean identity ===============
        (
            norm_expr!(Cos[Pattern[a, Blank[]]]^2 + Sin[Pattern[a, Blank[]]]^2 + Pattern[rest, BlankNullSeq[]]),
            expr!(1 + rest),
        ),
        (
            norm_expr!(Sqrt[1 - Cos[Pattern[x, Blank[]]] ^ 2]),
            expr!(Sin[x]),
        ),
        (
            norm_expr!(Sqrt[1 - Sin[Pattern[x, Blank[]]] ^ 2]),
            expr!(Cos[x]),
        ),
    ]
}
