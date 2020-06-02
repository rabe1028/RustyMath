use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait CommutativeMonoid<'a, T>: Monoid<'a, T> + Commutativity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> CommutativeMonoid<'a, Op> for T
where
    T: Monoid<'a, Op> + Commutativity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
