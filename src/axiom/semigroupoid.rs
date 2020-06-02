use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<'a, T>: Associativity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Semigroupoid<'a, Op> for T
where
    T: Associativity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
