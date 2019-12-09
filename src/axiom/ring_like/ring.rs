use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Ring<Add, Mul>: AbelianGroup<Add> + Semigroup<Mul> + Distributivity<Add, Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}

impl<Add, Mul, T> Ring<Add, Mul> for T
where
    T: AbelianGroup<Add> + Semigroup<Mul> + Distributivity<Add, Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
