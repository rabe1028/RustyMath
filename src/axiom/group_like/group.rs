use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Group<T>: Loop<T> + Monoid<T> + InverseSemigroup<T> + Groupoid<T>
where
    T: BinaryOperator<Self>,
{
}
