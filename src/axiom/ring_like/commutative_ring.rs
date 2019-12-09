use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait CommutativeRing<Add, Mul>: UnitalRing<Add, Mul> + Commutativity<Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}

impl<Add, Mul, T> CommutativeRing<Add, Mul> for T
where
    T: UnitalRing<Add, Mul> + Commutativity<Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
