use std::fmt;

pub mod ops;

pub mod integer;
pub mod rational;

#[derive(Debug, Clone, PartialEq)]
pub enum RealScalar {
    Integer(integer::BigInteger),
    Rational(rational::Rational),
    Pi,
    EulerNumber,
}

pub enum Scalar {
    Real(RealScalar),
    Complex(RealScalar, RealScalar),
}

impl RealScalar {
    pub fn from_f64(_value: f64) -> Self {
        todo!()
    }
}

impl fmt::Display for RealScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RealScalar::Integer(i) => write!(f, "{}", i),
            RealScalar::Rational(r) => write!(f, "{}/{}", r.numerator(), r.denominator()),
            RealScalar::Pi => write!(f, "π"),
            RealScalar::EulerNumber => write!(f, "e"),
        }
    }
}
