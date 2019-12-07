use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semigroup<T>: Magma<T> + Associativity<T>
where
    T: BinaryOperator<Self>,
{
}
