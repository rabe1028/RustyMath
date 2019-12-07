use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait AbelianGroup<T>: Group<T> + Commutativity<T>
where
    T: BinaryOperator<Self>,
{
}
