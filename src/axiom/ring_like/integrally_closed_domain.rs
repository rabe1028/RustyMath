use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait IntegrallyClosedDomain<'a, Add, Mul>: IntegralDomain<'a, Add, Mul> + IntegrallyClosed
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Add, Mul, T> IntegrallyClosedDomain<'a, Add, Mul> for T
where
    T: IntegralDomain<'a, Add, Mul> + IntegrallyClosed,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
