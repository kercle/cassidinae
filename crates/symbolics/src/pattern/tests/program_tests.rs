use crate::atom::Atom;
use crate::expr::Expr;
use crate::pattern::program::Compiler;
use expr_macro::expr;

#[test]
fn test_program_compilation() {
    let pattern = expr! {
        Blank[Pattern[x,Blank]]
    };

    let program = Compiler::new().compile(&pattern);

    dbg!(program);
}
