use std::collections::HashMap;

use crate::expr::Expr;

type SymbolId = String;

enum BindingVar {
    Named(SymbolId),
    Anonymous(usize),
}

pub struct MatchContext<'a, A>
where
    A: Clone + PartialEq,
{
    bindings: HashMap<BindingVar, &'a Expr<A>>,
}

impl<'a, A: Clone + PartialEq + Default> MatchContext<'a, A> {
    pub fn new() -> Self {
        MatchContext {
            bindings: HashMap::new(),
        }
    }
}

impl<A: Clone + PartialEq + Default> Expr<A> {
    pub fn matches<'a>(&'a self, pattern: &Self) -> Option<MatchContext<'a, A>> {
        let mut match_ctx = MatchContext::<'a, A>::new();

        if self.matches_inner(&mut match_ctx, pattern) {
            Some(match_ctx)
        } else {
            None
        }
    }

    fn matches_inner<'a>(&'a self, ctx: &mut MatchContext<'a, A>, pattern: &Self) -> bool {
        use Expr::*;

        match (self, pattern) {
            (Atom { entry: e, .. }, Atom { entry: oe, .. }) => e == oe,
            (Atom { .. }, Compound { .. }) => todo!(),
            (Compound { .. }, Compound { .. }) => todo!(),
            _ => unimplemented!(),
        }
    }
}
