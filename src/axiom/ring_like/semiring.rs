use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semiring<Add, Mul>:
    CommutativeMonoid<Add> + Monoid<Mul> + Distributivity<Add, Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}

impl<Add, Mul, T> Semiring<Add, Mul> for T
where
    T: CommutativeMonoid<Add> + Monoid<Mul> + Distributivity<Add, Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
