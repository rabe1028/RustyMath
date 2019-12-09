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
    #[inline(always)]
    fn identity() -> Self;
    #[inline(always)]
    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
    #[inline(always)]
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
    #[inline(always)]
    fn check_commutativity(x: Self, y: Self) -> bool {
        T::operate(x, y) == T::operate(y, x)
    }
}

pub trait RightDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: BinaryOperator<Self>,
    Add: BinaryOperator<Self>,
{
    #[inline(always)]
    fn check_right_distributivity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(Add::operate(x, y), z) == Add::operate(Mul::operate(x, z), Mul::operate(y, z))
    }
}

pub trait LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: BinaryOperator<Self>,
    Add: BinaryOperator<Self>,
{
    #[inline(always)]
    fn check_left_distributivity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(x, Add::operate(y, z)) == Add::operate(Mul::operate(x, y), Mul::operate(x, z))
    }
}

pub trait Distributivity<Add, Mul>:
    RightDistributivity<Add, Mul> + LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: BinaryOperator<Self>,
    Add: BinaryOperator<Self>,
{
    #[inline(always)]
    fn check_distributivity(x: Self, y: Self, z: Self) -> bool {
        Self::check_left_distributivity(x, y, z) && Self::check_right_distributivity(x, y, z)
    }
}

impl<T, Add, Mul> Distributivity<Add, Mul> for T
where
    T: LeftDistributivity<Add, Mul> + RightDistributivity<Add, Mul>,
    Add: BinaryOperator<T>,
    Mul: BinaryOperator<T>,
{
}
