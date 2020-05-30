use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait EuclidianDomain<Add, Mul>:
    PrincipalIdealDomain<Add, Mul> + AbelianGroup<GreatestCommonDivisor>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
    GreatestCommonDivisor: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> EuclidianDomain<Add, Mul> for T
where
    T: PrincipalIdealDomain<Add, Mul> + AbelianGroup<GreatestCommonDivisor>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
    GreatestCommonDivisor: InternalBinaryOperator<T>,
{
}
