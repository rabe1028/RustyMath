use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait QuasiGroup<'a, T>: Magma<'a, T> + Invertivility<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> QuasiGroup<'a, Op> for T
where
    T: Magma<'a, Op> + Invertivility<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
