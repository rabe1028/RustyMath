use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Nearring<'a, Add, Mul>: Group<'a, Add> + Semigroup<'a, Mul> + Distributivity<'a, Add, Mul>
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Add, Mul, T> Nearring<'a, Add, Mul> for T
where
    T: Group<'a, Add> + Semigroup<'a, Mul> + Distributivity<'a, Add, Mul>,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
