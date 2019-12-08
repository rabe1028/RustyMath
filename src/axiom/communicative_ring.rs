use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait CommunicativeRing<Add, Mul>: UnitalRing<Add, Mul> + Communicativity<Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}
