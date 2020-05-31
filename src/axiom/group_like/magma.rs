use crate::operator::*;
use crate::property::*;

pub trait Magma<T>: Totality<T>
where
    T: InternalBinaryOperator<Self>,
{
    fn op(self, other: Self) -> Self {
        T::operate(self, other)
    }
}

impl<Op, T> Magma<Op> for T
where
    T: Totality<Op>,
    Op: InternalBinaryOperator<T>,
{
}
