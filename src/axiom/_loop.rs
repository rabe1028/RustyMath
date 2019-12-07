use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Loop<T>: QuasiGroup<T> + Identity<T>
where
    T: BinaryOperator<Self>,
{
}
