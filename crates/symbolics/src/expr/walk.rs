use crate::expr::Expr;

pub struct ExprTopDownWalker<'a, A> {
    stack: Vec<&'a Expr<A>>,
}

impl<'a, A> ExprTopDownWalker<'a, A>
where
    A: PartialEq,
{
    pub fn new(root: &'a Expr<A>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, A> Iterator for ExprTopDownWalker<'a, A> {
    type Item = &'a Expr<A>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Expr::Compound { head, args, .. } = node {
            for a in args.iter().rev() {
                self.stack.push(a);
            }
            self.stack.push(head);
        }

        Some(node)
    }
}

enum Visit<'a, A> {
    Enter(&'a Expr<A>),
    Exit(&'a Expr<A>),
}

pub struct ExprBottomUpWalker<'a, A> {
    stack: Vec<Visit<'a, A>>,
}

impl<'a, A> ExprBottomUpWalker<'a, A> {
    pub fn new(root: &'a Expr<A>) -> Self {
        Self {
            stack: vec![Visit::Enter(root)],
        }
    }
}

impl<'a, A> Iterator for ExprBottomUpWalker<'a, A> {
    type Item = &'a Expr<A>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(visit) = self.stack.pop() {
            match visit {
                Visit::Enter(node) => {
                    self.stack.push(Visit::Exit(node));
                    if let Expr::Compound { head, args, .. } = node {
                        self.stack.push(Visit::Enter(head));
                        for a in args.iter().rev() {
                            self.stack.push(Visit::Enter(a));
                        }
                    }
                }
                Visit::Exit(node) => {
                    return Some(node);
                }
            }
        }
        None
    }
}
