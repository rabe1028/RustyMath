use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Groupoid<T>: Category<T> + Invertivility<T>
where
    T: BinaryOperator<Self>,
{
}
