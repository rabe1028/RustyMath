use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait IntegralDomain<Add, Mul>: CommutativeRing<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> IntegralDomain<Add, Mul> for T
where
    T: CommutativeRing<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
