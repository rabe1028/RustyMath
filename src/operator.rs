use std::borrow::Cow;

pub trait BinaryOperator<A, B, C>
where
    A: std::clone::Clone,
    B: std::clone::Clone,
{
    #[inline(always)]
    fn operate(lhs: &A, rhs: &B) -> C;
}

pub trait InternalBinaryOperator<T>: BinaryOperator<T, T, T>
where
    T: std::clone::Clone,
{
    #[inline(always)]
    fn operate(lhs: &T, rhs: &T) -> T {
        <Self as BinaryOperator<T, T, T>>::operate(lhs, rhs)
    }
}

pub trait ExternalBinaryOperator<S, T>: BinaryOperator<S, T, T>
where
    S: std::clone::Clone,
    T: std::clone::Clone,
{
    #[inline(always)]
    fn operate(lhs: &S, rhs: &T) -> T {
        <Self as BinaryOperator<S, T, T>>::operate(&lhs, &rhs)
    }
}

pub struct Addition {}

pub struct Multiplication {}
