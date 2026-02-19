use std::collections::HashMap;

use crate::expr::Expr;

pub enum Bound<'a, A> {
    One(&'a Expr<A>),
    Seq(&'a [Expr<A>]),
}

pub struct Binding<'a, A> {
    expr: &'a Expr<A>,
    rc: u32,
}

impl<'a, A> Binding<'a, A> {
    pub fn new(expr: &'a Expr<A>) -> Self {
        Self { expr, rc: 1 }
    }

    pub fn inc_bindings(&mut self) {
        self.rc += 1;
    }

    pub fn dec_bindings(&mut self) -> Result<(), ()> {
        if self.rc > 0 {
            self.rc -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn has_no_bindings(&self) -> bool {
        self.rc == 0
    }
}

pub struct MatchContext<'a, A> {
    bindings: HashMap<String, Binding<'a, A>>,
}

impl<'a, A> MatchContext<'a, A>
where
    A: PartialEq,
{
    pub fn new() -> Self {
        MatchContext {
            bindings: HashMap::new(),
        }
    }

    pub fn bind<T: AsRef<str>>(&mut self, name: T, expr: &'a Expr<A>) -> Result<(), ()> {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            if b.expr == expr {
                b.inc_bindings();
                Ok(())
            } else {
                Err(())
            }
        } else {
            self.bindings
                .insert(name.as_ref().to_string(), Binding::new(expr));
            Ok(())
        }
    }

    pub fn unbind<T: AsRef<str>>(&mut self, name: T) {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            let _ = b.dec_bindings();
            if b.has_no_bindings() {
                self.bindings.remove(name.as_ref());
            }
        }
    }
}
