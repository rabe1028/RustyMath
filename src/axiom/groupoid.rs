use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Groupoid<'a, T>: Category<'a, T> + Invertivility<'a, T>
where
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Groupoid<'a, Op> for T
where
    T: Category<'a, Op> + Invertivility<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}
