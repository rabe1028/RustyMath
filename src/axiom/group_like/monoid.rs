use crate::axiom::*;
use crate::operator::*;


pub trait Monoid<T>: Semigroup<T> + Category<T> + UnitalMagma<T>
where
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Monoid<Op> for T
where
    T: Semigroup<Op> + Category<Op> + UnitalMagma<Op>,
    Op: InternalBinaryOperator<T>,
{
}
