use crate::expr::Expr;

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn annotation_to_default(self) -> Self {
        match self {
            Expr::Atom { entry, .. } => Expr::Atom {
                entry,
                annotation: A::default(),
            },
            Expr::Compound { head, args, .. } => Expr::Compound {
                head,
                args,
                annotation: A::default(),
            },
        }
    }

    pub fn drop_annotation(self) -> Expr {
        self.map_annotations(&|_| ())
    }

    pub fn with_annotation(self, annotation: A) -> Self {
        use Expr::*;
        match self {
            Atom { entry, .. } => Atom { entry, annotation },
            Compound { head, args, .. } => Compound {
                head,
                args,
                annotation,
            },
        }
    }

    pub fn map_annotations<B, F>(self, f: &F) -> Expr<B>
    where
        F: Fn(A) -> B + Copy,
    {
        match self {
            Expr::Atom { entry, annotation } => Expr::Atom {
                entry,
                annotation: f(annotation),
            },
            Expr::Compound {
                head,
                args,
                annotation,
            } => {
                let head = head.map_annotations(f);
                let args = args.into_iter().map(|a| a.map_annotations(f)).collect();
                let annotation = f(annotation);

                Expr::Compound {
                    head: Box::new(head),
                    args,
                    annotation,
                }
            }
        }
    }

    pub fn map_bottom_up<F>(self, f: &F) -> Expr<A>
    where
        F: Fn(Expr<A>) -> Expr<A> + Copy,
    {
        match self {
            Expr::Atom { .. } => f(self),
            Expr::Compound { head, args, .. } => {
                let head = f(head.map_bottom_up(f));
                let args = args.into_iter().map(|a| f(a.map_bottom_up(f))).collect();
                f(Expr::new_compound(head, args))
            }
        }
    }
}
