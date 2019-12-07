use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Category<T>: Semigroupoid<T> + Identity<T>
where
    T: BinaryOperator<Self>,
{
}
