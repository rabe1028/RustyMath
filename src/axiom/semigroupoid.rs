use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<T>: Associativity<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Semigroupoid<Op> for T
where
    T: Semigroupoid<Op>,
    Op: InternalBinaryOperator<T>,
{
}
