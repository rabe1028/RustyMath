use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Category<T>: Semigroupoid<T> + Identity<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Category<Op> for T
where
    T: Semigroupoid<Op> + Identity<Op>,
    Op: InternalBinaryOperator<T>,
{
}
