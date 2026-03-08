#[macro_export]
macro_rules! expr_old {
    ($($tt:tt)*) => {
        $crate::expr_old::ExprOld::from(parser_macros::tcas_parse!($($tt)*))
    };
}

#[macro_export]
macro_rules! norm_expr_old {
    ($($tt:tt)*) => {
        $crate::expr_old::NormExprOld::new($crate::expr_old::ExprOld::from(parser_macros::tcas_parse!($($tt)*)))
    };
}

#[macro_export]
macro_rules! raw_expr {
    ($($tt:tt)*) => {
        $crate::expr::RawExpr::from(parser_macros::tcas_parse!($($tt)*))
    };
}

#[macro_export]
macro_rules! norm_expr {
    ($($tt:tt)*) => {
        $crate::raw_expr!($($tt)*).normalize()
    };
}

#[macro_export]
macro_rules! chain_replace_quick_and_dirty {
    // allow trailing comma
    ($expr:expr, $({ $($pat:tt)* } => { $($rep:tt)* }),+ $(,)?) => {{
        let mut __e = $expr;
        $(
            __e = __e.replace_all_quick_and_dirty(
                norm_expr! { $($pat)* },
                expr! { $($rep)* },
            );
        )+
        __e
    }};
}

#[macro_export]
macro_rules! rules {
    // entry: takes an identifier that is the rewriter expr (e.g. `rw;`)
    ($rw:expr; $(($lhs:tt) => $rhs:expr;)+) => {{
        let mut r = $rw;
        $(
            r = r.with_rule(
                norm_expr! { $lhs },
                |ctx| ctx.fill($rhs),
            );
        )+
        r
    }};
}
