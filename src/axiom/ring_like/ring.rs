use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Ring<Add, Mul>: AbelianGroup<Add> + Semigroup<Mul> + Distributivity<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    fn add(self, other: Self) -> Self {
        Add::operate(self, other)
    }

    fn sub(self, other: Self) -> Self {
        <Self as Invertivility<Add>>::inv_op(self, other)
    }

    fn mul(self, other: Self) -> Self {
        Mul::operate(self, other)
    }
}

impl<Add, Mul, T> Ring<Add, Mul> for T
where
    T: AbelianGroup<Add> + Semigroup<Mul> + Distributivity<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
