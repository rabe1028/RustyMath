use std::borrow::Cow;

use crate::operator::*;
use crate::property::*;

pub trait Magma<T>: Totality<T>
where
    T: InternalBinaryOperator<Self>,
{
    fn operate<'a, 'b, 'long: 'a + 'b>(
        lhs: impl Into<Cow<'a, Self>>,
        rhs: impl Into<Cow<'b, Self>>,
    ) -> Self
    where
        Self: 'long,
    {
        let lhs = lhs.into();
        let rhs = rhs.into();
        <T as InternalBinaryOperator<Self>>::operate(lhs, rhs)
    }
}

impl<Op, T> Magma<Op> for T
where
    T: Totality<Op>,
    Op: InternalBinaryOperator<T>,
{
}

//pub trait Magma<T> = Totality<T> where T : binaryOperator;
