use crate::axiom::*;
use crate::operator::*;

pub trait Monoid<'a, T>: Semigroup<'a, T> + Category<'a, T> + UnitalMagma<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Monoid<'a, Op> for T
where
    T: Semigroup<'a, Op> + Category<'a, Op> + UnitalMagma<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
