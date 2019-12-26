use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait AbelianGroup<T>: Group<T> + Commutativity<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> AbelianGroup<Op> for T
where
    T: Group<Op> + Commutativity<Op>,
    Op: InternalBinaryOperator<T>,
{
}
