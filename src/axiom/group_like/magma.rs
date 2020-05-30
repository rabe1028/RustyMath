use crate::operator::*;
use crate::property::*;

pub trait Magma<T>: Totality<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Magma<Op> for T
where
    T: Totality<Op>,
    Op: InternalBinaryOperator<T>,
{
}
