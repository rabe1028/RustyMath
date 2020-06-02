use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UniqueFactorizationDomain<'a, Add, Mul>:
    IntegrallyClosedDomain<'a, Add, Mul> + UniqueFactorizable
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Add, Mul, T> UniqueFactorizationDomain<'a, Add, Mul> for T
where
    T: IntegrallyClosedDomain<'a, Add, Mul> + UniqueFactorizable,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
