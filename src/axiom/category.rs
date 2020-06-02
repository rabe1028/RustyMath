use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Category<'a, T>: Semigroupoid<'a, T> + Identity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Category<'a, Op> for T
where
    T: Semigroupoid<'a, Op> + Identity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
