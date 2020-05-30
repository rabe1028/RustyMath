use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait PrincipalIdealDomain<Add, Mul>:
    UniqueFactorizationDomain<Add, Mul> + UniquePrimeFactorizable
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> PrincipalIdealDomain<Add, Mul> for T
where
    T: UniqueFactorizationDomain<Add, Mul> + UniquePrimeFactorizable,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
