use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semigroup<T>: Magma<T> + Associativity<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Semigroup<Op> for T
where
    T: Magma<Op> + Associativity<Op>,
    Op: InternalBinaryOperator<T>,
{
}
