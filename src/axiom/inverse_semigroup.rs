use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait InverseSemigroup<T>: Semigroup<T> + QuasiGroup<T>
where
    T: BinaryOperator<Self>,
{
}
