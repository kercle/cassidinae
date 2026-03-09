use crate::{norm_expr, simplify::Simplifier};

#[test]
fn test_r_divided_by_r_squared() {
    let expr = norm_expr!(r / r ^ 2);
    let expected = norm_expr!(Pow[r, -1]);

    let expr = Simplifier::new(expr).simple();

    assert_eq!(expr, expected);
}

#[test]
fn test_pow_a_m_times_pow_b_m_is_pow_ab_m() {
    let expr = norm_expr!(x ^ (1 + k) * y ^ (1 + k));
    let expected = norm_expr!((x * y) ^ (1 + k));

    let expr = Simplifier::new(expr).simple();

    assert_eq!(expr, expected);
}
