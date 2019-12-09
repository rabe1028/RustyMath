use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Field<Add, Mul>: AbelianGroup<Add> + AbelianGroup<Mul> + UnitalRing<Add, Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
    fn reciprocal(&self) -> Option<Self> {
        if (self.is_zero()) {
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
    T: AbelianGroup<Add> + AbelianGroup<Mul> + UnitalRing<Add, Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
