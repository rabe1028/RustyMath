use crate::operator::*;

pub trait Totality<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}

pub trait Associativity<T>
where
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<Self>,
{
    fn check_associativity(x: Self, y: Self, z: Self) -> bool {
        <T as InternalBinaryOperator<Self>>::operate(
            x,
            <T as InternalBinaryOperator<Self>>::operate(y, z),
        ) == <T as InternalBinaryOperator<Self>>::operate(
            <T as InternalBinaryOperator<Self>>::operate(x, y),
            z,
        )
    }
}

pub trait Identity<T>
where
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn identity() -> Self;
    #[inline(always)]
    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
    #[inline(always)]
    fn check_identity(x: Self) -> bool {
        (<T as InternalBinaryOperator<Self>>::operate(x, Self::identity()) == x)
            && (<T as InternalBinaryOperator<Self>>::operate(Self::identity(), x) == x)
    }
}
pub trait Invertivility<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
    fn inverse(&self) -> Self;
}
pub trait Commutativity<T>
where
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_commutativity(x: Self, y: Self) -> bool {
        <T as InternalBinaryOperator<Self>>::operate(x, y)
            == <T as InternalBinaryOperator<Self>>::operate(y, x)
    }
}

pub trait RightDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_right_distributivity(x: Self, y: Self, z: Self) -> bool {
        <Mul as InternalBinaryOperator<Self>>::operate(
            <Add as InternalBinaryOperator<Self>>::operate(x, y),
            z,
        ) == <Add as InternalBinaryOperator<Self>>::operate(
            <Mul as InternalBinaryOperator<Self>>::operate(x, z),
            <Mul as InternalBinaryOperator<Self>>::operate(y, z),
        )
    }
}

pub trait LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_left_distributivity(x: Self, y: Self, z: Self) -> bool {
        <Mul as InternalBinaryOperator<Self>>::operate(
            x,
            <Add as InternalBinaryOperator<Self>>::operate(y, z),
        ) == <Add as InternalBinaryOperator<Self>>::operate(
            <Mul as InternalBinaryOperator<Self>>::operate(x, y),
            <Mul as InternalBinaryOperator<Self>>::operate(x, z),
        )
    }
}

pub trait Distributivity<Add, Mul>:
    RightDistributivity<Add, Mul> + LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_distributivity(x: Self, y: Self, z: Self) -> bool {
        Self::check_left_distributivity(x, y, z) && Self::check_right_distributivity(x, y, z)
    }
}

impl<T, Add, Mul> Distributivity<Add, Mul> for T
where
    T: LeftDistributivity<Add, Mul> + RightDistributivity<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}

pub trait Absorbency<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_absorbency(x: Self, y: Self) -> bool {
        <Mul as InternalBinaryOperator<Self>>::operate(
            x,
            <Add as InternalBinaryOperator<Self>>::operate(x, y),
        ) == x
            && <Add as InternalBinaryOperator<Self>>::operate(
                x,
                <Mul as InternalBinaryOperator<Self>>::operate(x, y),
            ) == x
    }
}

pub trait Divisibility<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

pub trait LeftCancellative<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}
pub trait RightCancellative<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}
pub trait Cancellative<T>: LeftCancellative<T> + RightCancellative<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Cancellative<Op> for T
where
    T: LeftCancellative<Op> + RightCancellative<Op>,
    Op: InternalBinaryOperator<T>,
{
}

pub trait Mediality<T>
where
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_mediality(a: Self, b: Self, c: Self, d: Self) -> bool {
        <T as InternalBinaryOperator<Self>>::operate(
            <T as InternalBinaryOperator<Self>>::operate(a, b),
            <T as InternalBinaryOperator<Self>>::operate(c, d),
        ) == <T as InternalBinaryOperator<Self>>::operate(
            <T as InternalBinaryOperator<Self>>::operate(a, c),
            <T as InternalBinaryOperator<Self>>::operate(b, d),
        )
    }
}
