use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Field<Add, Mul>: AbelianGroup<Add> + AbelianGroup<Mul>
where
    Add: BinaryOperator<Self>,
    Mul: BinaryOperator<Self>,
{
}

impl<Add, Mul, T> Field<Add, Mul> for T
where
    T: AbelianGroup<Add> + AbelianGroup<Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
