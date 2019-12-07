use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait QuasiGroup<T>: Magma<T> + Invertivility<T>
where
    T: BinaryOperator<Self>,
{
    fn inverse(&self) -> Self {
        <Self as Invertivility<T>>::inverse(self)
    }
}
