use crate::expr::{Expr, NormalizedExpr};
use crate::{expr, norm_expr};

pub(crate) fn derivative_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Linearity ===============
        (
            norm_expr!( D[f_ + r__, PatternTest[x_, IsSymbolQ]] ),
            expr!( D[f, x] + D[Add[r],x] ),
        ),
        (
            norm_expr!(
            D[
                PatternTest[c_, IsNumberQ] * r__,
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(
            c * D[Mul[r],x]
            ),
        ),
        // =============== Basic ===============
        (norm_expr!( D[x_, PatternTest[x_, IsSymbolQ]] ), expr!(1)),
        (
            norm_expr!(
            D[
                PatternTest[c_, IsNumberQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(0),
        ),
        (
            norm_expr!(
            D[
                PatternTest[a_, IsSymbolQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(0),
        ),
        (
            norm_expr!( D[f_ * g_, PatternTest[x_, IsSymbolQ]] ),
            expr!(D[f, x] * g + f * D[g, x]),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            D[
                f_ ^ g_,
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((f ^ g) *((g / f) * D[f, x] + Log[f] * D[g, x])),
        ),
        // =============== Exponential ===============
        (
            norm_expr!(
            D[
                Exp[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Exp[f] * D[f, x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            D[
                Log[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / f) * D[f, x]),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            D[
                Sin[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Cos[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Cos[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(-Sin[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Tan[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / (Cos[f] ^ 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Cot[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(-(1 / (Sin[f] ^ 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Sec[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Sec[f] * Tan[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Csc[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(-Csc[f] * Cot[f] * D[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!(
            D[
                ArcSin[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcCos[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcTan[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / (1 + f ^ 2)) * D[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!(
            D[
                ArcSin[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcCos[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcTan[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / (1 + f ^ 2)) * D[f, x]),
        ),
        // =============== Hyperbolic functions ===============
        (
            norm_expr!(
            D[
                Sinh[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Cosh[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Cosh[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Sinh[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Tanh[f_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!((1 / (Cosh[f] ^ 2)) * D[f, x]),
        ),
    ]
}
