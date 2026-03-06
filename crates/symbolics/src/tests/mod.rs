use crate::{expr, norm_expr};

#[test]
fn blub() {
    let x = norm_expr!( 1+2+a+b );
    dbg!(x);
}
