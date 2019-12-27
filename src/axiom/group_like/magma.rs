
use crate::operator::*;
use crate::property::*;

pub trait Magma<T>: Totality<T>
where
    T: InternalBinaryOperator<Self>,
{
    fn operate(lhs: Self, rhs: Self) -> Self {
        <T as InternalBinaryOperator<Self>>::operate(lhs, rhs)
    }
}

impl<Op, T> Magma<Op> for T
where
    T: Totality<Op>,
    Op: InternalBinaryOperator<T>,
{
}

//pub trait Magma<T> = Totality<T> where T : binaryOperator;
