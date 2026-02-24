use std::rc::Rc;

use crate::pattern::Pattern;

#[derive(Clone, Debug)]
pub struct PatSpan<'a, A>
where
    A: Clone + PartialEq,
{
    reference: Rc<[Pattern<'a, A>]>,
    start: usize,
}

impl<'a, A> PatSpan<'a, A>
where
    A: Clone + PartialEq,
{
    pub fn from(arr: Vec<Pattern<'a, A>>) -> Self {
        PatSpan {
            reference: Rc::from(arr),
            start: 0,
        }
    }

    pub fn as_slice(&self) -> &[Pattern<'a, A>] {
        &self.reference[self.start..]
    }

    pub fn first(&self) -> Option<&Pattern<'a, A>> {
        self.reference.get(self.start)
    }

    pub fn rest(mut self) -> Self {
        self.start += 1;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.reference.len()
    }
}
