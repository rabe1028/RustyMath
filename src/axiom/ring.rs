use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Ring<Add, Mul>: AbelianGroup<Add> + Semigroup<Mul> + Distributivity<Add, Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}
