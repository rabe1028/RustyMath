use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Loop<T>: QuasiGroup<T> + Identity<T>
where
    T: BinaryOperator<Self>,
{
}

impl<Op, T> Loop<Op> for T
where
    T: QuasiGroup<Op> + Identity<Op>,
    Op: BinaryOperator<T>,
{
}
