use crate::operator::*;
use crate::property::*;

pub trait Magma<'a, T>: Totality<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
    fn op(self, other: Self) -> Self {
        T::operate(self, other)
    }
}

impl<'a, Op, T> Magma<'a, Op> for T
where
    T: Totality<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
