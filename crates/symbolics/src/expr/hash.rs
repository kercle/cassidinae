use std::hash::{DefaultHasher, Hash, Hasher};

use crate::expr::{Expr, ExprKind};

impl<S> Hash for Expr<S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind().hash(state);
    }
}

impl<E: Hash> Hash for ExprKind<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use ExprKind::*;

        match self {
            Atom { entry, .. } => {
                0u8.hash(state);
                entry.hash(state);
            }
            Node { head, args, .. } => {
                1u8.hash(state);
                head.hash(state);
                args.len().hash(state);
                for a in args {
                    a.hash(state);
                }
            }
        }
    }
}

impl<E: Hash> ExprKind<E> {
    pub fn digest(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        state.finish()
    }
}
