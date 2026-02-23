#[macro_export]
macro_rules! symbol {
    ( $($x:expr),* $(,)? ) => {
        ( $($crate::expr::generator::SymbolGenerator::new($x)),* )
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
