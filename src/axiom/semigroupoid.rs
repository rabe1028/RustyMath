use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<T>: Associativity<T>
where
    T: BinaryOperator<Self>,
{
}
