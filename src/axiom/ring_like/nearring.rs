use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Nearring<Add, Mul>: Group<Add> + Semigroup<Mul> + Distributivity<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> Nearring<Add, Mul> for T
where
    T: Group<Add> + Semigroup<Mul> + Distributivity<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
