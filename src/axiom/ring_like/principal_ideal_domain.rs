use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait PrincipalIdealDomain<'a, Add, Mul>:
    UniqueFactorizationDomain<'a, Add, Mul> + UniquePrimeFactorizable
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Add, Mul, T> PrincipalIdealDomain<'a, Add, Mul> for T
where
    T: UniqueFactorizationDomain<'a, Add, Mul> + UniquePrimeFactorizable,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
