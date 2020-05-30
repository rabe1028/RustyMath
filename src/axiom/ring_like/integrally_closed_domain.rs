use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait IntegrallyClosedDomain<Add, Mul>: IntegralDomain<Add, Mul> + IntegrallyClosed
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> IntegrallyClosedDomain<Add, Mul> for T
where
    T: IntegralDomain<Add, Mul> + IntegrallyClosed,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
