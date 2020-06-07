use crate::axiom::*;
use crate::operator::*;

pub trait Monoid<T>:
    Semigroup<T> + Category<T, Self, Self, Self, Self, Self, Self> + UnitalMagma<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Monoid<Op> for T
where
    T: Semigroup<Op> + Category<Op, T, T, T, T, T, T> + UnitalMagma<Op>,
    Op: InternalBinaryOperator<T>,
{
}
