use crate::axiom::*;
use crate::operator::*;

pub trait InverseSemigroup<'a, T>: Semigroup<'a, T> + QuasiGroup<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> InverseSemigroup<'a, Op> for T
where
    T: Semigroup<'a, Op> + QuasiGroup<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
