use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UnitalRing<Add, Mul>: Ring<Add, Mul> + Monoid<Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}
