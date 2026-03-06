use std::ops;

use numbers::{Number, integer::BigInteger};

use crate::ast::ParserAst;

impl ops::Add for ParserAst {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ParserAst::new_add(vec![self, other])
    }
}

impl ops::Add<u64> for ParserAst {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        ParserAst::new_add(vec![
            self,
            ParserAst::new_constant(Number::Integer(BigInteger::from_u64(other))),
        ])
    }
}

impl ops::Add<ParserAst> for u64 {
    type Output = ParserAst;

    fn add(self, other: ParserAst) -> ParserAst {
        ParserAst::new_add(vec![
            ParserAst::new_constant(Number::Integer(BigInteger::from_u64(self))),
            other,
        ])
    }
}

impl ops::Sub for ParserAst {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ParserAst::new_sub(self, other)
    }
}

impl ops::Mul for ParserAst {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ParserAst::new_mul(vec![self, other])
    }
}

impl ops::Div for ParserAst {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        ParserAst::new_div(self, other)
    }
}

impl ops::Mul<u64> for ParserAst {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        ParserAst::new_mul(vec![
            self,
            ParserAst::new_constant(Number::Integer(BigInteger::from_u64(other))),
        ])
    }
}

impl ops::Mul<ParserAst> for u64 {
    type Output = ParserAst;

    fn mul(self, other: ParserAst) -> ParserAst {
        ParserAst::new_mul(vec![
            ParserAst::new_constant(Number::Integer(BigInteger::from_u64(self))),
            other,
        ])
    }
}
