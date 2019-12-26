use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait CommutativeRing<Add, Mul>: UnitalRing<Add, Mul> + Commutativity<Mul>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

impl<Add, Mul, T> CommutativeRing<Add, Mul> for T
where
    T: UnitalRing<Add, Mul> + Commutativity<Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}
