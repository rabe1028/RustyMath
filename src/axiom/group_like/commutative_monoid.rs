use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait CommutativeMonoid<T>: Monoid<T> + Commutativity<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> CommutativeMonoid<Op> for T
where
    T: Monoid<Op> + Commutativity<Op>,
    Op: InternalBinaryOperator<T>,
{
}
