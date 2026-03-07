// ------------------------------------------------------
// MULTISET (COMMUTATIVE) LITERAL TESTS
// ------------------------------------------------------
//
//  Pattern         | Test Expr       | Expected Matches
//  ----------------|-----------------|------------------
//  Add[1, 2]       | Add[1, 2]       | 1
//  Add[1, 2]       | Add[2, 1]       | 1
//  Add[1, 2, 3]    | Add[3, 1, 2]    | 1
//  Add[1, 2, 3]    | Add[1, 2, 4]    | 0
//  Add[1, 2]       | Add[1, 2, 3]    | 0
//  Add[1, 1, 2]    | Add[1, 2, 1]    | 1
//  Add[1, 1, 2]    | Add[2, 1, 1]    | 1
//  Add[1, 1, 2]    | Add[1, 2, 2]    | 0
//  Mul[2, 3]       | Mul[3, 2]       | 1
//  Mul[2, 3]       | Mul[2, 4]       | 0
//  Add[f[1], f[2]] | Add[f[2], f[1]] | 1
//  Add[f[1], f[2]] | Add[f[1], f[3]] | 0

// ------------------------------------------------
// MULTISET WILDCARD / BLANK TESTS
// ------------------------------------------------
//
//  Pattern      | Test Expr    | Expected Matches
//  -------------|--------------|------------------
//  Add[_, 1]    | Add[1, 2]    | 1
//  Add[_, 1]    | Add[2, 1]    | 1
//  Add[_, 1]    | Add[2, 3]    | 0
//  Add[_, _]    | Add[1, 2]    | 2
//  Add[_, _]    | Add[1, 1]    | 2
//  Add[_, _, _] | Add[1, 2, 3] | 6
//  Add[_, 1, 2] | Add[2, 3, 1] | 1
//  Add[_, 1, 2] | Add[2, 1, 1] | 1
//  Add[x_, x_]  | Add[1, 1]    | 2
//  Add[x_, x_]  | Add[1, 2]    | 0
//  Add[x_, y_]  | Add[1, 2]    | 2

// ------------------------------------------------
// MULTISET BLANKSEQ / BLANKNULLSEQ TESTS
// ------------------------------------------------
//
//  Pattern      | Test Expr    | Expected Matches
//  -------------|--------------|------------------
//  Add[___]     | Add[]        | 1
//  Add[___]     | Add[1, 2, 3] | 1
//  Add[__]      | Add[]        | 0
//  Add[__]      | Add[1]       | 1
//  Add[__]      | Add[1, 2, 3] | 1
//  Add[1, ___]  | Add[1]       | 1
//  Add[1, ___]  | Add[1, 2, 3] | 1
//  Add[1, __]   | Add[1]       | 0
//  Add[1, __]   | Add[1, 2]    | 1
//  Add[1, __]   | Add[2, 1, 3] | 1
//  Add[x___, 1] | Add[1]       | 1
//  Add[x___, 1] | Add[2, 3, 1] | 1
//  Add[x__, 1]  | Add[1]       | 0
//  Add[x__, 1]  | Add[2, 1]    | 1

use crate::expr;
use crate::pattern::tests::utils::count_matches;

// ---- Multiset (Commutative) Literal Tests ----

#[test]
fn test_multiset_literal_1() {
    let pattern = expr! { Add[1, 2] };
    let subject = expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_2() {
    let pattern = expr! { Add[1, 2] };
    let subject = expr! { Add[2, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_3() {
    let pattern = expr! { Add[1, 2, 3] };
    let subject = expr! { Add[3, 1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_4() {
    let pattern = expr! { Add[1, 2, 3] };
    let subject = expr! { Add[1, 2, 4] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_5() {
    let pattern = expr! { Add[1, 2] };
    let subject = expr! { Add[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_6() {
    let pattern = expr! { Add[1, 1, 2] };
    let subject = expr! { Add[1, 2, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_7() {
    let pattern = expr! { Add[1, 1, 2] };
    let subject = expr! { Add[2, 1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_8() {
    let pattern = expr! { Add[1, 1, 2] };
    let subject = expr! { Add[1, 2, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_9() {
    let pattern = expr! { Mul[2, 3] };
    let subject = expr! { Mul[3, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_10() {
    let pattern = expr! { Mul[2, 3] };
    let subject = expr! { Mul[2, 4] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_11() {
    let pattern = expr! { Add[f[1], f[2]] };
    let subject = expr! { Add[f[2], f[1]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_literal_12() {
    let pattern = expr! { Add[f[1], f[2]] };
    let subject = expr! { Add[f[1], f[3]] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

// ---- Multiset Wildcard / Blank Tests ----

#[test]
fn test_multiset_blank_1() {
    let pattern = expr! { Add[_, 1] };
    let subject = expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_2() {
    let pattern = expr! { Add[_, 1] };
    let subject = expr! { Add[2, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_3() {
    let pattern = expr! { Add[_, 1] };
    let subject = expr! { Add[2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_4() {
    let pattern = expr! { Add[_, _] };
    let subject = expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_5() {
    let pattern = expr! { Add[_, _] };
    let subject = expr! { Add[1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_6() {
    let pattern = expr! { Add[_, _, _] };
    let subject = expr! { Add[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        6,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_7() {
    let pattern = expr! { Add[_, 1, 2] };
    let subject = expr! { Add[2, 3, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_8() {
    let pattern = expr! { Add[_, 1, 2] };
    let subject = expr! { Add[2, 1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_9() {
    let pattern = expr! { Add[x_, x_] };
    let subject = expr! { Add[1, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_10() {
    let pattern = expr! { Add[x_, x_] };
    let subject = expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_blank_11() {
    let pattern = expr! { Add[x_, y_] };
    let subject = expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        2,
        "Number of found matched patterns unexpected"
    );
}

// ---- Multiset BlankSeq / BlankNullSeq Tests ----

#[test]
fn test_multiset_seq_1() {
    let pattern = expr! { Add[___] };
    let subject = expr! { Add[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_2() {
    let pattern = expr! { Add[___] };
    let subject = expr! { Add[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_3() {
    let pattern = expr! { Add[__] };
    let subject = expr! { Add[] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_4() {
    let pattern = expr! { Add[__] };
    let subject = expr! { Add[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_5() {
    let pattern = expr! { Add[__] };
    let subject = expr! { Add[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_6() {
    let pattern = expr! { Add[1, ___] };
    let subject = expr! { Add[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_7() {
    let pattern = expr! { Add[1, ___] };
    let subject = expr! { Add[1, 2, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_8() {
    let pattern = expr! { Add[1, __] };
    let subject = expr! { Add[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_9() {
    let pattern = expr! { Add[1, __] };
    let subject = expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_10() {
    let pattern = expr! { Add[1, __] };
    let subject = expr! { Add[2, 1, 3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_11() {
    let pattern = expr! { Add[x___, 1] };
    let subject = expr! { Add[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_12() {
    let pattern = expr! { Add[x___, 1] };
    let subject = expr! { Add[2, 3, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_13() {
    let pattern = expr! { Add[x__, 1] };
    let subject = expr! { Add[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_multiset_seq_14() {
    let pattern = expr! { Add[x__, 1] };
    let subject = expr! { Add[2, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}
