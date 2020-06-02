use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semigroup<'a, T>: Magma<'a, T> + Associativity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Semigroup<'a, Op> for T
where
    T: Magma<'a, Op> + Associativity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
