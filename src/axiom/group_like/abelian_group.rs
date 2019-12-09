use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait AbelianGroup<T>: Group<T> + Commutativity<T>
where
    T: BinaryOperator<Self>,
{
}

impl<Op, T> AbelianGroup<Op> for T
where
    T: Group<Op> + Commutativity<Op>,
    Op: BinaryOperator<T>,
{
}
