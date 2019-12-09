use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Groupoid<T>: Category<T> + Invertivility<T>
where
    T: BinaryOperator<Self>,
{
}

impl<Op, T> Groupoid<Op> for T
where
    T: Category<Op> + Invertivility<Op>,
    Op: BinaryOperator<T>,
{
}
