use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Field<Add, Mul>: AbelianGroup<Mul> + EuclidianDomain<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn reciprocal(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(<Self as Invertivility<Mul>>::inverse(self))
        }
    }

    #[inline(always)]
    fn negation(&self) -> Self {
        <Self as Invertivility<Add>>::inverse(self)
    }
}

impl<Add, Mul, T> Field<Add, Mul> for T
where
    T:  AbelianGroup<Mul> + EuclidianDomain<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
