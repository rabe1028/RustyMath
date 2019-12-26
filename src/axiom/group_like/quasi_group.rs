use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait QuasiGroup<T>: Magma<T> + Invertivility<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> QuasiGroup<Op> for T
where
    T: Magma<Op> + Invertivility<Op>,
    Op: InternalBinaryOperator<T>,
{
}
