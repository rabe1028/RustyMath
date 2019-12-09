use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Nearring<Add, Mul>: Group<Add> + Semigroup<Mul> + Distributivity<Add, Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}

impl<Add, Mul, T> Nearring<Add, Mul> for T
where
    T: Group<Add> + Semigroup<Mul> + Distributivity<Add, Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}