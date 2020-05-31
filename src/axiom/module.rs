use crate::axiom::*;
use crate::operator::*;

pub trait Module<Coeff, Add, Mul>: Group<Add>
where
    Add: InternalBinaryOperator<Self> + InternalBinaryOperator<Coeff>,
    Mul: ExternalBinaryOperator<Coeff, Self> + InternalBinaryOperator<Coeff>,
    Coeff: UnitalRing<Add, Mul>,
{
}
