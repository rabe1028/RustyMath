use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UniqueFactorizationDomain<Add, Mul>:
    IntegrallyClosedDomain<Add, Mul> + UniqueFactorizable
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> UniqueFactorizationDomain<Add, Mul> for T
where
    T: IntegrallyClosedDomain<Add, Mul> + UniqueFactorizable,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
