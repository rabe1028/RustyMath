use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semiring<Add, Mul>:
    CommutativeMonoid<Add> + Monoid<Mul> + Distributivity<Add, Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> Semiring<Add, Mul> for T
where
    T: CommutativeMonoid<Add> + Monoid<Mul> + Distributivity<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
