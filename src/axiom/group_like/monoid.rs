use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Monoid<T>: Semigroup<T> + Category<T> + UnitalMagma<T>
where
    T: BinaryOperator<Self>,
{
}
