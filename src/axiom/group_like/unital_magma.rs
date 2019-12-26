use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UnitalMagma<T>: Magma<T> + Identity<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> UnitalMagma<Op> for T
where
    T: Magma<Op> + Identity<Op>,
    Op: InternalBinaryOperator<T>,
{
}
