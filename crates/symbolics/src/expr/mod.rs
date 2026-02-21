pub mod accessors;
pub mod constructors;
pub mod convert;
pub mod fmt;
pub mod generator;
pub mod hash;
pub mod norm;
pub mod ops;
pub mod transform;
pub mod types;
pub mod walk;

pub use types::*;

#[cfg(test)]
mod tests {
    use crate::{
        expr::{
            generator::{ExprBuilder, SymbolGenerator, cos, exp, pow},
            walk::ExprBottomUpWalker,
        },
        symbol,
    };

    use super::*;

    fn dd(f: Expr, x: SymbolGenerator) -> Expr {
        Expr::new_compound("D", vec![f, x.build()])
    }

    #[test]
    fn test_expr_ordering() {
        let x: Expr<()> = Expr::new_symbol("x");

        let expr1: Expr<()> = 2 + x + 3 * (Expr::from_i64(5) + 2);
        let expr2 = expr1.clone();

        assert_eq!(expr1, expr2);

        let x: Expr<()> = Expr::new_symbol("x");
        assert!(x > Expr::from_i64(2));
    }

    #[test]
    fn test_walker() {
        let (x, y, z) = symbol!("x", "y", "z");

        let expr = 2 + x * cos(x + dd(exp(pow(y, 2) + 7 * z), x));
        dbg!(&expr);
        for e in ExprBottomUpWalker::new(&expr) {
            dbg!(e);
        }
    }
}
