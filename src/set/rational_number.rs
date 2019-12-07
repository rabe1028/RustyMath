use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

#[derive(Clone, Copy, PartialEq)]
struct RationalNumber {
    p: isize,
    q: isize,
}

impl RationalNumber {
    fn new(p: isize, q: isize) -> Option<Self> {
        if q == 0 {
            None
        } else {
            Some(RationalNumber { p, q })
        }
    }
}

impl BinaryOperator<RationalNumber> for Addition {
    #[inline(always)]
    fn operate(lhs: RationalNumber, rhs: RationalNumber) -> RationalNumber {
        RationalNumber {
            p: lhs.p + rhs.p,
            q: lhs.q + rhs.q,
        }
    }
}

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

impl Magma<Addition> for RationalNumber {}
impl Semigroupoid<Addition> for RationalNumber {}

impl QuasiGroup<Addition> for RationalNumber {}
impl Semigroup<Addition> for RationalNumber {}
impl UnitalMagma<Addition> for RationalNumber {}
impl Category<Addition> for RationalNumber {}

impl Loop<Addition> for RationalNumber {}
impl InverseSemigroup<Addition> for RationalNumber {}
impl Monoid<Addition> for RationalNumber {}
impl Groupoid<Addition> for RationalNumber {}

impl Group<Addition> for RationalNumber {}

impl AbelianGroup<Addition> for RationalNumber {}

impl BinaryOperator<RationalNumber> for Multiplication {
    #[inline(always)]
    fn operate(lhs: RationalNumber, rhs: RationalNumber) -> RationalNumber {
        RationalNumber {
            p: lhs.p + rhs.p,
            q: lhs.q + rhs.q,
        }
    }
}

impl Totality<Multiplication> for RationalNumber {}
impl Associativity<Multiplication> for RationalNumber {}

impl Magma<Multiplication> for RationalNumber {}

impl Semigroup<Multiplication> for RationalNumber {}

impl Distributivity<Addition, Multiplication> for RationalNumber {}

impl Ring<Addition, Multiplication> for RationalNumber {}
