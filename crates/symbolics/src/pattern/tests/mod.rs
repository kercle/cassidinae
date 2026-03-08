mod literals;
mod mixed;
mod multiset;
mod other;
mod predicate;
mod program;
mod sequence;
mod utils;
mod wildcards;

use crate::norm_expr;

use super::*;

#[test]
fn test_built_pattern_from_expr() {
    let expr = norm_expr! {
        PatternTest[Blank[], IsSymbolQ]
    };
    let pattern = Pattern::from_expr(&expr);
    assert_eq!(
        format!("{pattern:?}"),
        r#"Blank{None, None, Some(IsSymbolQ)}"#
    );

    let expr = norm_expr! {
        PatternTest[Pattern[x, Blank[]], IsSymbolQ]
    };
    let pattern = Pattern::from_expr(&expr);

    assert_eq!(
        format!("{pattern:?}"),
        r#"Blank{Some("x"), None, Some(IsSymbolQ)}"#
    );
}
