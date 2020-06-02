use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Ring<'a, Add, Mul>: AbelianGroup<'a, Add> + Semigroup<'a, Mul> + Distributivity<'a, Add, Mul>
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Add::operate(self, other)
    }

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        <Self as Invertivility<'a, Add>>::inv_op(self, other)
    }

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Mul::operate(self, other)
    }

    #[inline(always)]
    fn negation(&self) -> Self {
        <Self as Invertivility<'a, Add>>::inverse(self)
    }

    #[inline(always)]
    fn zero() -> Self {
        <Self as Identity<'a, Add>>::identity()
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        <Self as Identity<'a, Add>>::is_identity(self)
    }
}

impl<'a, Add, Mul, T> Ring<'a, Add, Mul> for T
where
    T: AbelianGroup<'a, Add> + Semigroup<'a, Mul> + Distributivity<'a, Add, Mul>,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
