use crate::{expr::Expr, pattern::{program::Compiler, runtime::{Environment, Runtime}}};

pub(super) fn first_match<'p, 's>(
    program: &'p crate::pattern::program::Program<()>,
    subject: &'s Expr,
) -> Option<Environment<'p, 's, ()>> {
    Runtime::new(program, subject).next()
}

pub(super) fn count_matches(pattern: &Expr, subject: &Expr) -> usize {
    let program = Compiler::new().compile(pattern);
    let runtime = Runtime::new(&program, subject);
    runtime.count()
}
