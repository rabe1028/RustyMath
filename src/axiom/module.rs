use crate::axiom::*;
use crate::operator::*;

pub trait Module<'a, Coeff, Add, Mul>: Group<'a, Add>
where
    Add: InternalBinaryOperator<'a, Self> + InternalBinaryOperator<'a, Coeff>,
    Mul: ExternalBinaryOperator<Coeff, Self> + InternalBinaryOperator<'a, Coeff>,
    Coeff: UnitalRing<'a, Add, Mul>,
{
}
