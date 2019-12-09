pub trait BinaryOperator<T>: Sized {
    #[inline(always)]
    fn operate(lhs: T, rhs: T) -> T;
}

pub struct Addition {}

pub struct Multiplication {}
