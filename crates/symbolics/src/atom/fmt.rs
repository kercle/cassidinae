use std::fmt::{Debug, Error, Formatter};

use crate::atom::Atom;

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Atom::Number(value) => write!(f, "{value}"),
            Atom::Symbol(name) => write!(f, "{name}"),
            Atom::StringLiteral(value) => write!(f, "{value}"),
        }
    }
}
