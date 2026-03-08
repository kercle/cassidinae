// ------------------------------------------------------------------------------------------------
// BLANKSEQ (1 OR MORE) TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                   | Test Expr     | Expected Matches
//  --------------------------|---------------|------------------
//  f[BlankSeq[]]             | f[1]          | 1
//  f[BlankSeq[]]             | f[1, 2, 3]    | 1
//  f[BlankSeq[]]             | f[]           | 0
//  f[1, BlankSeq[]]          | f[1, 2]       | 1
//  f[1, BlankSeq[]]          | f[1]          | 0
//  f[1, BlankSeq[]]          | f[1, 2, 3, 4] | 1
//  f[BlankSeq[], 1]          | f[2, 1]       | 1
//  f[BlankSeq[], 1]          | f[1]          | 0
//  f[BlankSeq[], 1]          | f[2, 3, 1]    | 1
//  f[1, BlankSeq[], 2]       | f[1, 99, 2]   | 1
//  f[1, BlankSeq[], 2]       | f[1, 2]       | 0
//  f[1, BlankSeq[], 2]       | f[1, 3, 4, 2] | 1
//  f[BlankSeq[], BlankSeq[]] | f[1]          | 0
//  f[BlankSeq[], BlankSeq[]] | f[1, 2]       | 1
//  f[BlankSeq[], BlankSeq[]] | f[1, 2, 3]    | 2
//  f[BlankSeq[], BlankSeq[]] | f[1, 2, 3, 4] | 3

// -------------------------------------------------------------------
// BLANKNULLSEQ (0 OR MORE) TESTS
// -------------------------------------------------------------------
//
//  Pattern                           | Test Expr  | Expected Matches
//  ----------------------------------|------------|------------------
//  f[BlankNullSeq[]]                 | f[]        | 1
//  f[BlankNullSeq[]]                 | f[1]       | 1
//  f[BlankNullSeq[]]                 | f[1, 2, 3] | 1
//  f[1, BlankNullSeq[]]              | f[1]       | 1
//  f[1, BlankNullSeq[]]              | f[1, 2, 3] | 1
//  f[BlankNullSeq[], 1]              | f[1]       | 1
//  f[BlankNullSeq[], 1]              | f[2, 3, 1] | 1
//  f[BlankNullSeq[], BlankNullSeq[]] | f[]        | 1
//  f[BlankNullSeq[], BlankNullSeq[]] | f[1]       | 2
//  f[BlankNullSeq[], BlankNullSeq[]] | f[1, 2]    | 3
//  f[BlankNullSeq[], BlankNullSeq[]] | f[1, 2, 3] | 4
//  f[BlankSeq[], BlankNullSeq[]]     | f[1]       | 1
//  f[BlankSeq[], BlankNullSeq[]]     | f[1, 2]    | 2
//  f[BlankSeq[], BlankNullSeq[]]     | f[1, 2, 3] | 3
//  f[BlankNullSeq[], BlankSeq[]]     | f[1]       | 1
//  f[BlankNullSeq[], BlankSeq[]]     | f[1, 2]    | 2

// ----------------------------------------------------------------------------------------------
// NAMED SEQUENCE BINDING TESTS
// ----------------------------------------------------------------------------------------------
//
//  Pattern                                                    | Test Expr     | Expected Matches
//  -----------------------------------------------------------|---------------|-----------------
//  f[Pattern[x, BlankSeq[]]]                                  | f[1]          | 1
//  f[Pattern[x, BlankSeq[]]]                                  | f[1, 2, 3]    | 1
//  f[Pattern[x, BlankNullSeq[]]]                              | f[]           | 1
//  f[Pattern[x, BlankNullSeq[]]]                              | f[1, 2]       | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]          | f[1, 1]       | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]          | f[1, 2, 1, 2] | 1
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]          | f[1, 2]       | 0
//  f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]]          | f[1, 2, 3]    | 0
//  f[Pattern[x, BlankNullSeq[]], Pattern[x, BlankNullSeq[]]]  | f[]           | 1
//  f[Pattern[x, BlankNullSeq[]], Pattern[x, BlankNullSeq[]]]  | f[1, 1]       | 1
//  f[1, Pattern[x, BlankSeq[]], 2]                            | f[1, 5, 6, 2] | 1
//  f[Pattern[x, BlankSeq[]], Blank[], Pattern[x, BlankSeq[]]] | f[1, 2, 1]    | 1
//  f[Pattern[x, BlankSeq[]], Blank[], Pattern[x, BlankSeq[]]] | f[1, 2, 3]    | 0

use crate::norm_expr;
use crate::pattern::tests::utils::count_matches;

// ---- BlankSeq (1 or more) Tests ----

#[test]
fn test_blank_seq_1() {
    let pattern = norm_expr! { f[BlankSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_2() {
    let pattern = norm_expr! { f[BlankSeq[]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_3() {
    let pattern = norm_expr! { f[BlankSeq[]] };
    let subject = norm_expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_4() {
    let pattern = norm_expr! { f[1, BlankSeq[]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_5() {
    let pattern = norm_expr! { f[1, BlankSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_6() {
    let pattern = norm_expr! { f[1, BlankSeq[]] };
    let subject = norm_expr! { f[1, 2, 3, 4] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_7() {
    let pattern = norm_expr! { f[BlankSeq[], 1] };
    let subject = norm_expr! { f[2, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_8() {
    let pattern = norm_expr! { f[BlankSeq[], 1] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_9() {
    let pattern = norm_expr! { f[BlankSeq[], 1] };
    let subject = norm_expr! { f[2, 3, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_10() {
    let pattern = norm_expr! { f[1, BlankSeq[], 2] };
    let subject = norm_expr! { f[1, 99, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_11() {
    let pattern = norm_expr! { f[1, BlankSeq[], 2] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_12() {
    let pattern = norm_expr! { f[1, BlankSeq[], 2] };
    let subject = norm_expr! { f[1, 3, 4, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_13() {
    let pattern = norm_expr! { f[BlankSeq[], BlankSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_14() {
    let pattern = norm_expr! { f[BlankSeq[], BlankSeq[]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_15() {
    let pattern = norm_expr! { f[BlankSeq[], BlankSeq[]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_seq_16() {
    let pattern = norm_expr! { f[BlankSeq[], BlankSeq[]] };
    let subject = norm_expr! { f[1, 2, 3, 4] };
    assert_eq!(
        count_matches(&pattern, &subject),
        3,
        "Number of found matched patterns unexpected"
    );
}

// ---- BlankNullSeq (0 or more) Tests ----

#[test]
fn test_blank_null_seq_1() {
    let pattern = norm_expr! { f[BlankNullSeq[]] };
    let subject = norm_expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_2() {
    let pattern = norm_expr! { f[BlankNullSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_3() {
    let pattern = norm_expr! { f[BlankNullSeq[]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_4() {
    let pattern = norm_expr! { f[1, BlankNullSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_5() {
    let pattern = norm_expr! { f[1, BlankNullSeq[]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_6() {
    let pattern = norm_expr! { f[BlankNullSeq[], 1] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_7() {
    let pattern = norm_expr! { f[BlankNullSeq[], 1] };
    let subject = norm_expr! { f[2, 3, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_8() {
    let pattern = norm_expr! { f[BlankNullSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_9() {
    let pattern = norm_expr! { f[BlankNullSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_10() {
    let pattern = norm_expr! { f[BlankNullSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        3,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_11() {
    let pattern = norm_expr! { f[BlankNullSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        4,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_12() {
    let pattern = norm_expr! { f[BlankSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_13() {
    let pattern = norm_expr! { f[BlankSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_14() {
    let pattern = norm_expr! { f[BlankSeq[], BlankNullSeq[]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        3,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_15() {
    let pattern = norm_expr! { f[BlankNullSeq[], BlankSeq[]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_blank_null_seq_16() {
    let pattern = norm_expr! { f[BlankNullSeq[], BlankSeq[]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

// ---- Named Sequence Binding Tests ----

#[test]
fn test_named_seq_1() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_2() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_3() {
    let pattern = norm_expr! { f[Pattern[x, BlankNullSeq[]]] };
    let subject = norm_expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_4() {
    let pattern = norm_expr! { f[Pattern[x, BlankNullSeq[]]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_5() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_6() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 2, 1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_7() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_8() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_9() {
    let pattern = norm_expr! { f[Pattern[x, BlankNullSeq[]], Pattern[x, BlankNullSeq[]]] };
    let subject = norm_expr! { f[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_10() {
    let pattern = norm_expr! { f[Pattern[x, BlankNullSeq[]], Pattern[x, BlankNullSeq[]]] };
    let subject = norm_expr! { f[1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_11() {
    let pattern = norm_expr! { f[1, Pattern[x, BlankSeq[]], 2] };
    let subject = norm_expr! { f[1, 5, 6, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_12() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Blank[], Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 2, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_named_seq_13() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Blank[], Pattern[x, BlankSeq[]]] };
    let subject = norm_expr! { f[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}
