use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Monoid<T>: Semigroup<T> + Category<T> + UnitalMagma<T>
where
    T: BinaryOperator<Self>,
{
}

impl<Op, T> Monoid<Op> for T
where
    T: Semigroup<Op> + Category<Op> + UnitalMagma<Op>,
    Op: BinaryOperator<T>,
{
}
