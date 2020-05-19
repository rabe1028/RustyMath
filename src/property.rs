use crate::operator::*;

pub trait Totality<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}

pub trait Associativity<T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<Self>,
{
    fn check_associativity(x: Self, y: Self, z: Self) -> bool {
        <T as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            <T as InternalBinaryOperator<Self>>::operate(y.clone(), z.clone()),
        ) == <T as InternalBinaryOperator<Self>>::operate(
            <T as InternalBinaryOperator<Self>>::operate(x, y),
            z,
        )
    }
}

pub trait Identity<T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<Self>,
{
    fn identity() -> Self;
    #[inline(always)]
    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
    #[inline(always)]
    fn check_identity(x: Self) -> bool {
        let id = Self::identity();
        let left = <T as InternalBinaryOperator<Self>>::operate(x.clone(), id.clone());
        let right = <T as InternalBinaryOperator<Self>>::operate(id.clone(), x.clone());
        (left == x) && (right == x)
    }
}
pub trait Invertivility<T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<Self>,
{
    fn inverse(&self) -> Self;
}
pub trait Commutativity<T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_commutativity(x: Self, y: Self) -> bool {
        <T as InternalBinaryOperator<Self>>::operate(x.clone(), y.clone())
            == <T as InternalBinaryOperator<Self>>::operate(y, x)
    }
}

pub trait RightDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_right_distributivity(x: Self, y: Self, z: Self) -> bool {
        <Mul as InternalBinaryOperator<Self>>::operate(
            <Add as InternalBinaryOperator<Self>>::operate(x.clone(), y.clone()),
            z.clone(),
        ) == <Add as InternalBinaryOperator<Self>>::operate(
            <Mul as InternalBinaryOperator<Self>>::operate(x.clone(), z.clone()),
            <Mul as InternalBinaryOperator<Self>>::operate(y.clone(), z.clone()),
        )
    }
}

pub trait LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_left_distributivity(x: Self, y: Self, z: Self) -> bool {
        <Mul as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            <Add as InternalBinaryOperator<Self>>::operate(y.clone(), z.clone()),
        ) == <Add as InternalBinaryOperator<Self>>::operate(
            <Mul as InternalBinaryOperator<Self>>::operate(x.clone(), y),
            <Mul as InternalBinaryOperator<Self>>::operate(x.clone(), z),
        )
    }
}

pub trait Distributivity<Add, Mul>:
    RightDistributivity<Add, Mul> + LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_distributivity(x: Self, y: Self, z: Self) -> bool {
        Self::check_left_distributivity(x.clone(), y.clone(), z.clone())
            && Self::check_right_distributivity(x, y, z)
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
    Self: Sized + PartialEq + Clone,
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_absorbency(x: Self, y: Self) -> bool {
        let add_xy = <Add as InternalBinaryOperator<Self>>::operate(x.clone(), y.clone());
        let mul_xy = <Mul as InternalBinaryOperator<Self>>::operate(x.clone(), y.clone());

        <Mul as InternalBinaryOperator<Self>>::operate(x.clone(), add_xy) == x
            && <Add as InternalBinaryOperator<Self>>::operate(x.clone(), mul_xy) == x
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
    Self: Sized + Clone,
    T: InternalBinaryOperator<Self>,
{
}
pub trait RightCancellative<T>
where
    Self: Sized + Clone,
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
        let ab = <T as InternalBinaryOperator<Self>>::operate(a.clone(), b.clone());
        let cd = <T as InternalBinaryOperator<Self>>::operate(c.clone(), d.clone());
        let ac = <T as InternalBinaryOperator<Self>>::operate(a, c);
        let bd = <T as InternalBinaryOperator<Self>>::operate(b, d);

        <T as InternalBinaryOperator<Self>>::operate(ab, cd)
            == <T as InternalBinaryOperator<Self>>::operate(ac, bd)
    }
}
