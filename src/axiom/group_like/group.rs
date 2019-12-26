use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Group<T>: Loop<T> + Monoid<T> + InverseSemigroup<T> + Groupoid<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Group<Op> for T
where
    T: Loop<Op> + Monoid<Op> + InverseSemigroup<Op> + Groupoid<Op>,
    Op: InternalBinaryOperator<T>,
{
}
