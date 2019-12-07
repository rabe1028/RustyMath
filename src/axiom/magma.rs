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

//pub trait Magma<T> = Totality<T> where T : binaryOperator;

/*
impl Magma<Addition> for isize {
    fn operate(lhs: Self, rhs: Self) -> Self {
        lhs + rhs
    }
}
*/
