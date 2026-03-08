use std::fmt::{Debug, Error, Formatter};

use crate::expr::{Expr, ExprKind};

impl<S> Debug for Expr<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = match self.kind() {
            ExprKind::Atom { entry, .. } => format!("{entry:?}"),
            ExprKind::Node { head, args, .. } => {
                let head_str = if matches!(*head.kind(), ExprKind::Node { .. }) {
                    format!("({head:?})")
                } else {
                    format!("{head:?}")
                };
                let args_str: Vec<String> = args.iter().map(|a| format!("{a:?}")).collect();

                format!("{head_str}[{}]", args_str.join(", "))
            }
        };

        write!(f, "{s}")
    }
}
