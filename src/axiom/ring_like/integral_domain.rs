use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait IntegralDomain<'a, Add, Mul>: CommutativeRing<'a, Add, Mul> + NoZeroDivisor
where
    Add: InternalBinaryOperator<'a,Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

impl<'a,Add, Mul, T> IntegralDomain<'a,Add, Mul> for T
where
    T: CommutativeRing<'a, Add, Mul> + NoZeroDivisor,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
