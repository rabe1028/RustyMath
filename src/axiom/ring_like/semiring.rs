use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semiring<Add, Mul>:
    CommutativeMonoid<Add> + Monoid<Mul> + Distributivity<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn one() -> Self {
        <Self as Identity<Mul>>::identity()
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        <Self as Identity<Mul>>::is_identity(self)
    }
}

impl<Add, Mul, T> Semiring<Add, Mul> for T
where
    T: CommutativeMonoid<Add> + Monoid<Mul> + Distributivity<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
