use crate::{expr::NormExpr, norm_expr};

pub(crate) fn indefinite_integrals_rules() -> Vec<(NormExpr, NormExpr)> {
    vec![
        // =============== Linearity ===============
        (
            norm_expr!(
            Integrate[
                f_ + r__,
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(
            Integrate[f, x] + Integrate[Add[r],x]
            ),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[c_, IsNumberQ] * r__,
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(
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
            norm_expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                x_,
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(x ^ 2 / 2),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[c_, IsSymbolQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[a_, IsSymbolQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(a * x),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            Integrate[
                1 / x_,
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(Log[Abs[x]]),
        ),
        (
            norm_expr!(
            Integrate[
                x_ ^ PatternTest[k_, IsNumberQ],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(x ^ (k + 1) / (k + 1)),
        ),
        // =============== Exponentials ===============
        (
            norm_expr!(
            Integrate[
                Exp[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(Exp[x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            Integrate[
                Log[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(x * Log[x] - x),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            Integrate[
                Sin[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(-Cos[x]),
        ),
        (
            norm_expr!(
            Integrate[
                Cos[x_],
                PatternTest[x_, IsSymbolQ]
            ]),
            norm_expr!(Sin[x]),
        ),
    ]
}
