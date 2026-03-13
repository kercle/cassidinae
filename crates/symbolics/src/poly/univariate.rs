use numbers::Number;

use crate::expr::NormExpr;

#[derive(Clone)]
pub struct UnivariatePolynomial {
    coeff: Vec<Number>,
    symbol: String,
}

#[derive(Debug, Clone)]
pub enum LongDivisionError {
    SymbolMismatch,
}

impl UnivariatePolynomial {
    pub fn from_expr(_expr: &NormExpr) -> Option<Self> {
        // needs polynom checking predicate
        // which in turn needs Optional in
        // pattern matcher.
        todo!()
    }

    pub fn zero<T: AsRef<str>>(symbol: T) -> Self {
        Self {
            coeff: vec![Number::zero()],
            symbol: symbol.as_ref().to_string(),
        }
    }

    pub fn degree(&self) -> usize {
        self.coeff.len() - 1
    }

    pub fn coeff(&self, i: usize) -> &Number {
        self.coeff.get(i).unwrap_or(&numbers::ZERO)
    }

    pub fn long_division(&self, other: &Self) -> Result<(Self, Self), LongDivisionError> {
        if self.symbol != other.symbol {
            return Err(LongDivisionError::SymbolMismatch);
        }

        if other.degree() > self.degree() {
            return Ok((UnivariatePolynomial::zero(&self.symbol), other.clone()));
        }
        todo!()
    }
}
