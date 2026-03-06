use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
};

pub fn indefinite_integrals_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Linearity ===============
        (
            norm_expr!(
            Integrate[
                Pattern[f, Blank[]] + Pattern[r, BlankSeq[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(
            Integrate[f, x] + Integrate[Add[r],x]
            ),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[Pattern[c, Blank[]], IsNumberQ] * Pattern[r, BlankSeq[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(
            c * Integrate[Mul[r],x]
            ),
        ),
        // =============== Basic ===============
        (
            norm_expr!(
            Integrate[
                PatternTest[Pattern[c, Blank[]], IsNumberQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                Pattern[x, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(x ^ 2 / 2),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[Pattern[c, Blank[]], IsSymbolQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[Pattern[a, Blank[]], IsSymbolQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(a * x),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            Integrate[
                1 / Pattern[x, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Log[Abs[x]]),
        ),
        (
            norm_expr!(
            Integrate[
                Pattern[x, Blank[]] ^ PatternTest[Pattern[k, Blank[]], IsNumberQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(x ^ (k + 1) / (k + 1)),
        ),
        // =============== Exponentials ===============
        (
            norm_expr!(
            Integrate[
                Exp[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Exp[x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            Integrate[
                Log[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(x * Log[x] - x),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            Integrate[
                Sin[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-Cos[x]),
        ),
        (
            norm_expr!(
            Integrate[
                Cos[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Sin[x]),
        ),
    ]
}
