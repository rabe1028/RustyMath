use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Loop<'a, T>: QuasiGroup<'a, T> + Identity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Loop<'a, Op> for T
where
    T: QuasiGroup<'a, Op> + Identity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
