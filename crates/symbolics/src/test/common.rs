use crate::expr::Expr;

pub fn pattern(bind_name: &str, a2: Expr<()>) -> Expr<()> {
    Expr::new_compound(
        Expr::new_symbol("Pattern"),
        vec![Expr::new_symbol(bind_name), a2],
    )
}

pub fn blank() -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Blank"), Vec::new())
}

pub fn x() -> Expr<()> {
    Expr::new_symbol("x")
}

pub fn y() -> Expr<()> {
    Expr::new_symbol("y")
}
