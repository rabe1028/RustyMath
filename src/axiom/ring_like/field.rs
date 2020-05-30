use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Field<Add, Mul>: AbelianGroup<Mul> + EuclidianDomain<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
    GreatestCommonDivisor: InternalBinaryOperator<Self>,
{
    fn reciprocal(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(<Self as Invertivility<Mul>>::inverse(self))
        }
    }

    fn negation(&self) -> Self {
        <Self as Invertivility<Add>>::inverse(self)
    }
}

impl<Add, Mul, T> Field<Add, Mul> for T
where
    T:  AbelianGroup<Mul> + EuclidianDomain<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
    GreatestCommonDivisor: InternalBinaryOperator<T>,
{
}
