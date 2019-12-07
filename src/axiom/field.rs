use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Field: AbelianGroup<Addition> + AbelianGroup<Multiplication>
where
    Addition: BinaryOperator<Self>,
    Multiplication: BinaryOperator<Self>,
{
}
