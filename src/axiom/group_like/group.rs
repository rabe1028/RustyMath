use crate::axiom::*;
use crate::operator::*;

pub trait Group<'a, T>: Loop<'a, T> + Monoid<'a, T> + InverseSemigroup<'a, T> + Groupoid<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Group<'a, Op> for T
where
    T: Loop<'a, Op> + Monoid<'a, Op> + InverseSemigroup<'a, Op> + Groupoid<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
