// ---------------------------------------------------------
// WILDCARD TESTS
// ---------------------------------------------------------
//
//  Pattern                | Test Expr   | Expected Matches
//  -----------------------|-------------|------------------
//  Blank[]                | 5           | 1
//  Blank[]                | foo         | 1
//  Blank[]                | f[1, 2]     | 1
//  f[Blank[]]             | f[1]        | 1
//  f[Blank[]]             | f[f[1, 2]]  | 1
//  f[Blank[]]             | f[1, 2]     | 0
//  f[Blank[], Blank[]]    | f[1, 2]     | 1
//  f[Blank[], Blank[]]    | f[1, 2, 3]  | 0
//  f[Blank[], 2, Blank[]] | f[1, 2, 3]  | 1
//  f[Blank[], 2, Blank[]] | f[1, 3, 3]  | 0
//  f[g[Blank[]], 2]       | f[g[99], 2] | 1
//  f[g[Blank[]], 2]       | f[g[99], 3] | 0
//  f[g[Blank[]]]          | f[h[1]]     | 0

// ----------------------------------------------------------
// HEAD PATTERN (WILDCARD WITH HEAD CONSTRAINT) TESTS
// ----------------------------------------------------------
//
//  Pattern               | Test Expr     | Expected Matches
//  ----------------------|---------------|------------------
//  Blank[f]              | f[1, 2]       | 1
//  Blank[f]              | g[1, 2]       | 0
//  Blank[f]              | 5             | 0
//  f[Blank[g]]           | f[g[1]]       | 1
//  f[Blank[g]]           | f[h[1]]       | 0
//  f[Blank[g], Blank[g]] | f[g[1], g[2]] | 1
//  f[Blank[g], Blank[g]] | f[g[1], h[2]] | 0
//  f[Blank[g], Blank[h]] | f[g[1], h[2]] | 1
//  f[Blank[f]]           | f[f[f[]]]     | 1

// -----------------------------------------------------------------------------------------------------
// NAMED VARIABLE / PATTERN BINDING TESTS
// -----------------------------------------------------------------------------------------------------
//
//  Pattern                                                          | Test Expr     | Expected Matches
//  -----------------------------------------------------------------|---------------|------------------
//  Pattern[x, Blank[]]                                              | 5             | 1
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]]]                      | f[1, 1]       | 1
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]]]                      | f[1, 2]       | 0
//  f[Pattern[x, Blank[]], Pattern[y, Blank[]]]                      | f[1, 2]       | 1
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]]]]                   | f[1, g[1]]    | 1
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]]]]                   | f[1, g[2]]    | 0
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[x, Blank[]]] | f[3, 3, 3]    | 1
//  f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[x, Blank[]]] | f[3, 3, 4]    | 0
//  f[Pattern[x, Blank[f]], Pattern[x, Blank[f]]]                    | f[f[1], f[1]] | 1
//  f[Pattern[x, Blank[f]], Pattern[x, Blank[f]]]                    | f[f[1], f[2]] | 0

use crate::expr;
use crate::pattern::tests::utils::count_matches;

// ---- Wildcard Tests ----

#[test]
fn test_wildcard_1() {
    let pattern = expr! { Blank[] };
    let subject = expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_2() {
    let pattern = expr! { Blank[] };
    let subject = expr! { foo };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_3() {
    let pattern = expr! { Blank[] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_4() {
    let pattern = expr! { f[Blank[]] };
    let subject = expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_5() {
    let pattern = expr! { f[Blank[]] };
    let subject = expr! { f[f[1, 2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_6() {
    let pattern = expr! { f[Blank[]] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_7() {
    let pattern = expr! { f[Blank[], Blank[]] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_8() {
    let pattern = expr! { f[Blank[], Blank[]] };
    let subject = expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_9() {
    let pattern = expr! { f[Blank[], 2, Blank[]] };
    let subject = expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_10() {
    let pattern = expr! { f[Blank[], 2, Blank[]] };
    let subject = expr! { f[1, 3, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_11() {
    let pattern = expr! { f[g[Blank[]], 2] };
    let subject = expr! { f[g[99], 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_12() {
    let pattern = expr! { f[g[Blank[]], 2] };
    let subject = expr! { f[g[99], 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_wildcard_13() {
    let pattern = expr! { f[g[Blank[]]] };
    let subject = expr! { f[h[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

// ---- Head Pattern (Wildcard with Head Constraint) Tests ----

#[test]
fn test_head_pattern_1() {
    let pattern = expr! { Blank[f] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_2() {
    let pattern = expr! { Blank[f] };
    let subject = expr! { g[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_3() {
    let pattern = expr! { Blank[f] };
    let subject = expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_4() {
    let pattern = expr! { f[Blank[g]] };
    let subject = expr! { f[g[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_5() {
    let pattern = expr! { f[Blank[g]] };
    let subject = expr! { f[h[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_6() {
    let pattern = expr! { f[Blank[g], Blank[g]] };
    let subject = expr! { f[g[1], g[2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_7() {
    let pattern = expr! { f[Blank[g], Blank[g]] };
    let subject = expr! { f[g[1], h[2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_8() {
    let pattern = expr! { f[Blank[g], Blank[h]] };
    let subject = expr! { f[g[1], h[2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_head_pattern_9() {
    let pattern = expr! { f[Blank[f]] };
    let subject = expr! { f[f[f[]]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

// ---- Named Variable / Pattern Binding Tests ----

#[test]
fn test_named_binding_1() {
    let pattern = expr! { Pattern[x, Blank[]] };
    let subject = expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_2() {
    let pattern = expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]]] };
    let subject = expr! { f[1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_3() {
    let pattern = expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]]] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_4() {
    let pattern = expr! { f[Pattern[x, Blank[]], Pattern[y, Blank[]]] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_5() {
    let pattern = expr! { f[Pattern[x, Blank[]], g[Pattern[x, Blank[]]]] };
    let subject = expr! { f[1, g[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_6() {
    let pattern = expr! { f[Pattern[x, Blank[]], g[Pattern[x, Blank[]]]] };
    let subject = expr! { f[1, g[2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_7() {
    let pattern = expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[x, Blank[]]] };
    let subject = expr! { f[3, 3, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_8() {
    let pattern = expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[x, Blank[]]] };
    let subject = expr! { f[3, 3, 4] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_9() {
    let pattern = expr! { f[Pattern[x, Blank[f]], Pattern[x, Blank[f]]] };
    let subject = expr! { f[f[1], f[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_binding_10() {
    let pattern = expr! { f[Pattern[x, Blank[f]], Pattern[x, Blank[f]]] };
    let subject = expr! { f[f[1], f[2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}
