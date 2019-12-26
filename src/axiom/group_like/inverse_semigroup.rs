use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait InverseSemigroup<T>: Semigroup<T> + QuasiGroup<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> InverseSemigroup<Op> for T
where
    T: Semigroup<Op> + QuasiGroup<Op>,
    Op: InternalBinaryOperator<T>,
{
}
