pub trait BinaryOperator<T>: Sized {
    fn operate(lhs: T, rhs: T) -> T;
}

pub struct Addition {}

pub struct Multiplication {}
