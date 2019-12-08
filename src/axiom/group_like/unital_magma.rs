use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait UnitalMagma<T>: Magma<T> + Identity<T>
where
    T: BinaryOperator<Self>,
{
}
