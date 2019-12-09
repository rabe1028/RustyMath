use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Magma<T>: Totality<T>
where
    T: BinaryOperator<Self>,
{
    fn operate(lhs: Self, rhs: Self) -> Self {
        <T as BinaryOperator<Self>>::operate(lhs, rhs)
    }
}

impl<Op, T> Magma<Op> for T
where
    T: Totality<Op>,
    Op: BinaryOperator<T>,
{
}

//pub trait Magma<T> = Totality<T> where T : binaryOperator;
