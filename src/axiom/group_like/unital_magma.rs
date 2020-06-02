use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UnitalMagma<'a, T>: Magma<'a, T> + Identity<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> UnitalMagma<'a, Op> for T
where
    T: Magma<'a, Op> + Identity<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
