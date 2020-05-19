#[macro_use(forward_internal_binop)]
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

forward_internal_binop! { Addition, RationalNumber,
    (lhs, rhs) => {
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

forward_internal_binop! { Multiplication, RationalNumber,
    (lhs, rhs) => {
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
