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

pub trait Distributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: BinaryOperator<Self>,
    Add: BinaryOperator<Self>,
{
    fn check_commutativity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(x, Add::operate(y, z)) == Add::operate(Mul::operate(x, y), Mul::operate(x, z))
            && Mul::operate(Add::operate(x, y), z)
                == Add::operate(Mul::operate(x, z), Mul::operate(y, z))
    }
}
