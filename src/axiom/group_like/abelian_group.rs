use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait AbelianGroup<'a, T>: Group<'a, T> + Commutativity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> AbelianGroup<'a, Op> for T
where
    T: Group<'a, Op> + Commutativity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
