use crate::operator::*;

pub trait Totality<T>
where
    Self: Sized,
    T: BinaryOperator<Self>,
{
}
pub trait Associativity<T>
where
    Self: Sized + PartialEq + Copy,
    T: BinaryOperator<Self>,
{
    fn check_associativity(x: Self, y: Self, z: Self) -> bool {
        T::operate(x, T::operate(y, z)) == T::operate(T::operate(x, y), z)
    }
}
pub trait Identity<T>
where
    Self: Sized + PartialEq + Copy,
    T: BinaryOperator<Self>,
{
    fn identity() -> Self;
    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }

    fn check_identity(x: Self) -> bool {
        (T::operate(x, Self::identity()) == x) && (T::operate(Self::identity(), x) == x)
    }
}
pub trait Invertivility<T>
where
    Self: Sized,
    T: BinaryOperator<Self>,
{
    fn inverse(&self) -> Self;
}
pub trait Commutativity<T>
where
    Self: Sized + PartialEq + Copy,
    T: BinaryOperator<Self>,
{
    fn check_commutativity(x: Self, y: Self) -> bool {
        T::operate(x, y) == T::operate(y, x)
    }
}
