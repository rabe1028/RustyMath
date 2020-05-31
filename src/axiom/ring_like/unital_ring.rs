use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UnitalRing<Add, Mul>: Ring<Add, Mul> + Monoid<Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    // #[inline(always)]
    // fn one() -> Self {
    //     <Self as Identity<Mul>>::identity()
    // }

    // #[inline(always)]
    // fn is_one(&self) -> bool {
    //     <Self as Identity<Mul>>::is_identity(self)
    // }

    // #[inline(always)]
    // fn zero() -> Self {
    //     <Self as Identity<Add>>::identity()
    // }

    // #[inline(always)]
    // fn is_zero(&self) -> bool {
    //     <Self as Identity<Mul>>::is_identity(self)
    // }
}

impl<Add, Mul, T> UnitalRing<Add, Mul> for T
where
    T: Ring<Add, Mul> + Monoid<Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
