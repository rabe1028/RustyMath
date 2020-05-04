use std::borrow::Cow;

pub trait BinaryOperator<A, B, C>
where
    A: std::clone::Clone,
    B: std::clone::Clone,
{
    #[inline(always)]
    fn operate<'a, 'b>(lhs: impl Into<Cow<'a, A>>, rhs: impl Into<Cow<'b, B>>) -> C
    where
        A: 'a,
        B: 'b;
}

pub trait InternalBinaryOperator<T>: BinaryOperator<T, T, T>
where
    T: std::clone::Clone,
{
    #[inline(always)]
    fn operate<'a, U>(lhs: U, rhs: U) -> T
    where
        U: Into<Cow<'a, T>>,
        T: 'a,
    {
        <Self as BinaryOperator<T, T, T>>::operate(lhs, rhs)
    }
}

pub trait ExternalBinaryOperator<S, T>: BinaryOperator<S, T, T>
where
    S: std::clone::Clone,
    T: std::clone::Clone,
{
    #[inline(always)]
    fn operate<'a, 'b, N, M>(lhs: N, rhs: M) -> T
    where
        N: Into<Cow<'a, S>>,
        M: Into<Cow<'b, T>>,
        S: 'a,
        T: 'b,
    {
        <Self as BinaryOperator<S, T, T>>::operate(lhs, rhs)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Addition {}

#[derive(Debug, Eq, PartialEq)]
pub enum Multiplication {}
