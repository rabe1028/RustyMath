use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait CommutativeRing<'a, Add, Mul>: UnitalRing<'a, Add, Mul> + Commutativity<'a, Mul>
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Add, Mul, T> CommutativeRing<'a, Add, Mul> for T
where
    T: UnitalRing<'a, Add, Mul> + Commutativity<'a, Mul>,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}
