use crate::expr::{Expr, ExprKind, RawExpr};

impl<S> Expr<S> {
    pub fn replace<F>(self, f: &F) -> RawExpr
    where
        F: Fn(&Expr<S>) -> Option<Expr<S>> + Copy,
    {
        if let Some(replacement) = f(&self) {
            return replacement.into_raw();
        }

        match self.kind {
            ExprKind::Atom { .. } => {
                let e = Expr::new_unchecked(self.kind);
                f(&e).unwrap_or(e).into_raw()
            }
            ExprKind::Node { head, args, .. } => {
                let head = f(&head).map(Self::into_raw).unwrap_or(head.replace(f));
                let args = args
                    .into_iter()
                    .map(|arg| f(&arg).map(Self::into_raw).unwrap_or(arg.replace(f)))
                    .collect();

                Expr::new_node(head, args)
            }
        }
    }
}

impl RawExpr {
    pub fn map_top_down<F>(self, f: &F) -> RawExpr
    where
        F: Fn(RawExpr) -> RawExpr + Copy,
    {
        let transformed = f(self);

        use ExprKind::*;
        match transformed.kind {
            Atom { .. } => transformed,
            Node { head, args, .. } => {
                let head = head.map_top_down(f);
                let args = args.into_iter().map(|a| a.map_top_down(f)).collect();
                Expr::new_node(head, args)
            }
        }
    }

    pub fn map_bottom_up<F>(self, f: &F) -> RawExpr
    where
        F: Fn(RawExpr) -> RawExpr + Copy,
    {
        use ExprKind::*;
        match self.kind {
            Atom { .. } => self,
            Node { head, args, .. } => {
                let head = head.map_bottom_up(f);
                let args = args.into_iter().map(|a| a.map_bottom_up(f)).collect();
                f(Expr::new_node(head, args))
            }
        }
    }
}
