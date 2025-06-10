use std::{cmp, ops};

use crate::integer::BigInteger;

impl ops::Add for BigInteger {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        BigInteger::add(&self, &other)
    }
}

impl ops::Add for &BigInteger {
    type Output = BigInteger;

    fn add(self, other: &BigInteger) -> Self::Output {
        BigInteger::add(self, other)
    }
}

impl ops::Mul for BigInteger {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        BigInteger::mul(&self, &other)
    }
}

impl ops::Mul for &BigInteger {
    type Output = BigInteger;

    fn mul(self, other: &BigInteger) -> Self::Output {
        BigInteger::mul(self, other)
    }
}

impl cmp::PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
