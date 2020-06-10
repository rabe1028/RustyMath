use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Group<Op>:
    Loop<Op> + Monoid<Op> + InverseSemigroup<Op> + Groupoid<Op, Self, Self, Self, Self, Self, Self>
where
    Self: Endomorphism + Invertivility<Op, Inverse = Self>,
    Op: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Group<Op> for T
where
    T: Loop<Op>
        + Monoid<Op>
        + InverseSemigroup<Op>
        + Groupoid<Op, Self, Self, Self, Self, Self, Self>
        + Endomorphism
        + Invertivility<Op, Inverse = Self>,
    Op: InternalBinaryOperator<T>,
{
}
