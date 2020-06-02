use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semiring<'a, Add, Mul>:
    CommutativeMonoid<'a, Add> + Monoid<'a, Mul> + Distributivity<'a, Add, Mul>
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn one() -> Self {
        <Self as Identity<'a, Mul>>::identity()
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        <Self as Identity<'a, Mul>>::is_identity(self)
    }
}

impl<'a, Add, Mul, T> Semiring<'a, Add, Mul> for T
where
    T: CommutativeMonoid<'a, Add> + Monoid<'a, Mul> + Distributivity<'a, Add, Mul>,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
