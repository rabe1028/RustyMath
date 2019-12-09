use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait IntegralDomain<Add, Mul>: CommutativeRing<Add, Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}

impl<Add, Mul, T> IntegralDomain<Add, Mul> for T
where
    T: CommutativeRing<Add, Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
