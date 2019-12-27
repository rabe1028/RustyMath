use crate::operator::*;
use crate::property::*;

#[derive(Clone, Copy)]
struct RationalNumber {
    p: isize,
    q: isize,
}

impl RationalNumber {
    pub fn new(p: isize, q: isize) -> Option<Self> {
        if q == 0 {
            None
        } else {
            Some(RationalNumber { p, q })
        }
    }
}

impl PartialEq for RationalNumber {
    fn eq(&self, other: &Self) -> bool {
        self.p * other.q == self.q * other.p
    }
}

impl BinaryOperator<RationalNumber, RationalNumber, RationalNumber> for Addition {
    #[inline(always)]
    fn operate(lhs: RationalNumber, rhs: RationalNumber) -> RationalNumber {
        RationalNumber {
            p: lhs.p * rhs.q + rhs.p * lhs.q,
            q: lhs.q * rhs.q,
        }
    }
}

impl InternalBinaryOperator<RationalNumber> for Addition {}

impl Totality<Addition> for RationalNumber {}
impl Associativity<Addition> for RationalNumber {}
impl Identity<Addition> for RationalNumber {
    #[inline(always)]
    fn identity() -> Self {
        RationalNumber { p: 0, q: 1 }
    }
}

impl Invertivility<Addition> for RationalNumber {
    #[inline(always)]
    fn inverse(&self) -> Self {
        RationalNumber {
            p: -self.p,
            q: self.q,
        }
    }
}

impl Commutativity<Addition> for RationalNumber {}

impl BinaryOperator<RationalNumber, RationalNumber, RationalNumber> for Multiplication {
    #[inline(always)]
    fn operate(lhs: RationalNumber, rhs: RationalNumber) -> RationalNumber {
        RationalNumber {
            p: lhs.p * rhs.p,
            q: lhs.q * rhs.q,
        }
    }
}

impl InternalBinaryOperator<RationalNumber> for Multiplication {}

impl Totality<Multiplication> for RationalNumber {}
impl Associativity<Multiplication> for RationalNumber {}

impl RightDistributivity<Addition, Multiplication> for RationalNumber {}
impl LeftDistributivity<Addition, Multiplication> for RationalNumber {}
//impl Distributivity<Addition, Multiplication> for RationalNumber {}

impl Commutativity<Multiplication> for RationalNumber {}
impl Invertivility<Multiplication> for RationalNumber {
    #[inline(always)]
    fn inverse(&self) -> Self {
        RationalNumber {
            p: self.q,
            q: self.p,
        }
    }
}

impl Identity<Multiplication> for RationalNumber {
    #[inline(always)]
    fn identity() -> Self {
        RationalNumber { p: 1, q: 1 }
    }
}
