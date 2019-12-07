use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Ring: AbelianGroup<Addition> + Semigroup<Multiplication>
where
    Addition: BinaryOperator<Self>,
    Multiplication: BinaryOperator<Self>,
{
}
