use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UnitalRing<'a, Add, Mul>: Ring<'a, Add, Mul> + Monoid<'a, Mul>
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
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

impl<'a, Add, Mul, T> UnitalRing<'a, Add, Mul> for T
where
    T: Ring<'a, Add, Mul> + Monoid<'a, Mul>,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
