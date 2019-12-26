pub trait BinaryOperator<A, B, C> {
    #[inline(always)]
    fn operate(lhs: A, rhs: B) -> C;
}

pub trait InternalBinaryOperator<T>: BinaryOperator<T, T, T> {
    #[inline(always)]
    fn operate(lhs: T, rhs: T) -> T {
        <Self as BinaryOperator<T, T, T>>::operate(lhs, rhs)
    }
}

pub trait ExternalBinaryOperator<S, T>: BinaryOperator<S, T, T> {
    #[inline(always)]
    fn operate(lhs: S, rhs: T) -> T {
        <Self as BinaryOperator<S, T, T>>::operate(lhs, rhs)
    }
}

pub struct Addition {}

pub struct Multiplication {}
