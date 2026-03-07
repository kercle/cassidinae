use crate::expr::{Expr, NormalizedExpr};
use crate::{expr, norm_expr};

pub(crate) fn indefinite_integrals_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Linearity ===============
        (
            norm_expr!(
            Integrate[
                f_ + r__,
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(
            Integrate[f, x] + Integrate[Add[r],x]
            ),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[c_, IsNumberQ] * r__,
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(
            c * Integrate[Mul[r],x]
            ),
        ),
        // =============== Basic ===============
        (
            norm_expr!(
            Integrate[
                PatternTest[c_, IsNumberQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                x_,
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(x ^ 2 / 2),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[c_, IsSymbolQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[a_, IsSymbolQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(a * x),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            Integrate[
                1 / x_,
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Log[Abs[x]]),
        ),
        (
            norm_expr!(
            Integrate[
                x_ ^ PatternTest[k_, IsNumberQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(x ^ (k + 1) / (k + 1)),
        ),
        // =============== Exponentials ===============
        (
            norm_expr!(
            Integrate[
                Exp[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Exp[x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            Integrate[
                Log[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(x * Log[x] - x),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            Integrate[
                Sin[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(-Cos[x]),
        ),
        (
            norm_expr!(
            Integrate[
                Cos[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            expr!(Sin[x]),
        ),
    ]
}
