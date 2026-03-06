// ------------------------------------------------------------------------------------------------------------
// NESTED / MIXED DEEP STRUCTURE TESTS
// ------------------------------------------------------------------------------------------------------------
//
//  Pattern                                                             | Test Expr         | Expected Matches
//  --------------------------------------------------------------------|-------------------|------------------
//  f[g[Blank[], Blank[]]]                                              | f[g[1, 2]]        | 1
//  f[g[Blank[], Blank[]]]                                              | f[g[1]]           | 0
//  f[g[BlankSeq[]]]                                                    | f[g[1, 2, 3]]     | 1
//  f[g[Pattern[x, Blank[]]], Pattern[x, Blank[]]]                      | f[g[5], 5]        | 1
//  f[g[Pattern[x, Blank[]]], Pattern[x, Blank[]]]                      | f[g[5], 6]        | 0
//  f[BlankSeq[], g[BlankSeq[]]]                                        | f[1, 2, g[3, 4]]  | 1
//  f[BlankSeq[], g[BlankSeq[]]]                                        | f[g[3, 4]]        | 0
//  Add[Mul[Blank[], Blank[]], Blank[]]                                 | Add[Mul[2, 3], 4] | 2
//  Add[Mul[Blank[], Blank[]], Blank[]]                                 | Add[4, Mul[2, 3]] | 2
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]], Pattern[x, Blank[]]]] | f[1, g[1, 1]]     | 1
//  f[Pattern[x, Blank[]], g[Pattern[x, Blank[]], Pattern[x, Blank[]]]] | f[1, g[1, 2]]     | 0
//  Add[f[Pattern[x, Blank[]]], f[Pattern[x, Blank[]]]]                 | Add[f[3], f[3]]   | 2
//  Add[f[Pattern[x, Blank[]]], f[Pattern[x, Blank[]]]]                 | Add[f[3], f[4]]   | 0

// ---------------------------------------------------------------------------------------------------------------
// EDGE CASES / DEGENERATE INPUTS
// ---------------------------------------------------------------------------------------------------------------
//
//  Pattern                                                                   | Test Expr      | Expected Matches
//  --------------------------------------------------------------------------|----------------|------------------
//  f[BlankSeq[], BlankSeq[], BlankSeq[]]                                     | f[1, 2, 3]     | 3
//  f[BlankSeq[], BlankSeq[], BlankSeq[]]                                     | f[1, 2]        | 1
//  f[BlankSeq[], BlankSeq[], BlankSeq[]]                                     | f[1]           | 0
//  f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]]                         | f[]            | 1
//  f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]]                         | f[1]           | 3
//  f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]]                         | f[1, 2]        | 6
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] | f[1, 1, 1]     | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] | f[1,2,1,2,1,2] | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] | f[1, 2, 3]     | 0
//  f[Blank[]]                                                                | f[]            | 0
//  f[]                                                                       | f[Blank[]]     | 0
//  Add[]                                                                     | Add[]          | 1
//  Add[1]                                                                    | Add[1]         | 1
//  f[g[]]                                                                    | f[g[]]         | 1
//  f[g[]]                                                                    | f[g[1]]        | 0

use crate::expr;
use crate::pattern::tests::utils::count_matches;

// ---- Nested / Mixed Deep Structure ----

#[test]
fn test_mixed_1() {
    let pattern = expr! { f[g[Blank[], Blank[]]] };
    let subject = expr! { f[g[1, 2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_2() {
    let pattern = expr! { f[g[Blank[], Blank[]]] };
    let subject = expr! { f[g[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_3() {
    let pattern = expr! { f[g[BlankSeq[]]] };
    let subject = expr! { f[g[1, 2, 3]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_4() {
    let pattern = expr! { f[g[Pattern[x, Blank[]]], Pattern[x, Blank[]]] };
    let subject = expr! { f[g[5], 5] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_5() {
    let pattern = expr! { f[g[Pattern[x, Blank[]]], Pattern[x, Blank[]]] };
    let subject = expr! { f[g[5], 6] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_6() {
    let pattern = expr! { f[BlankSeq[], g[BlankSeq[]]] };
    let subject = expr! { f[1, 2, g[3, 4]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_7() {
    let pattern = expr! { f[BlankSeq[], g[BlankSeq[]]] };
    let subject = expr! { f[g[3, 4]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_8() {
    let pattern = expr! { Add[Mul[Blank[], Blank[]], Blank[]] };
    let subject = expr! { Add[Mul[2, 3], 4] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_9() {
    let pattern = expr! { Add[Mul[Blank[], Blank[]], Blank[]] };
    let subject = expr! { Add[4, Mul[2, 3]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_10() {
    let pattern = expr! { f[Pattern[x, Blank[]], g[Pattern[x, Blank[]], Pattern[x, Blank[]]]] };
    let subject = expr! { f[1, g[1, 1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_11() {
    let pattern = expr! { f[Pattern[x, Blank[]], g[Pattern[x, Blank[]], Pattern[x, Blank[]]]] };
    let subject = expr! { f[1, g[1, 2]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_12() {
    let pattern = expr! { Add[f[Pattern[x, Blank[]]], f[Pattern[x, Blank[]]]] };
    let subject = expr! { Add[f[3], f[3]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_mixed_13() {
    let pattern = expr! { Add[f[Pattern[x, Blank[]]], f[Pattern[x, Blank[]]]] };
    let subject = expr! { Add[f[3], f[4]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

// ---- Edge Cases / Degenerate Inputs ----

#[test]
fn test_edge_1() {
    let pattern = expr! { f[BlankSeq[], BlankSeq[], BlankSeq[]] };
    let subject = expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_2() {
    let pattern = expr! { f[BlankSeq[], BlankSeq[], BlankSeq[]] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_3() {
    let pattern = expr! { f[BlankSeq[], BlankSeq[], BlankSeq[]] };
    let subject = expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_4() {
    let pattern = expr! { f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]] };
    let subject = expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_5() {
    let pattern = expr! { f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]] };
    let subject = expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        3,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_6() {
    let pattern = expr! { f[BlankNullSeq[], BlankNullSeq[], BlankNullSeq[]] };
    let subject = expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        6,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_7() {
    let pattern =
        expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = expr! { f[1, 1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_8() {
    let pattern =
        expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = expr! { f[1, 2, 1, 2, 1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_9() {
    let pattern =
        expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_10() {
    let pattern = expr! { f[Blank[]] };
    let subject = expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_11() {
    let pattern = expr! { f[] };
    let subject = expr! { f[Blank[]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_12() {
    let pattern = expr! { Add[] };
    let subject = expr! { Add[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_13() {
    let pattern = expr! { Add[1] };
    let subject = expr! { Add[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_14() {
    let pattern = expr! { f[g[]] };
    let subject = expr! { f[g[]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_edge_15() {
    let pattern = expr! { f[g[]] };
    let subject = expr! { f[g[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}
