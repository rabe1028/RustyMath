use crate::operator::*;

pub trait Totality<'a, T>
where
    Self: Sized,
    T: InternalBinaryOperator<'a, Self>,
{
}

pub trait Associativity<'a, T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<'a, Self>,
{
    fn check_associativity(x: Self, y: Self, z: Self) -> bool {
        T::operate(x.clone(), T::operate(y.clone(), z.clone())) == T::operate(T::operate(x, y), z)
    }
}

pub trait Identity<'a, T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<'a, Self>,
{
    fn identity() -> Self;
    #[inline(always)]
    fn is_identity(&self) -> bool {
        *self == Self::identity()
    }
    #[inline(always)]
    fn check_identity(x: Self) -> bool {
        let id = Self::identity();
        let left = T::operate(x.clone(), id.clone());
        let right = T::operate(id.clone(), x.clone());
        (left == x) && (right == x)
    }
}
pub trait Invertivility<'a, T>: Identity<'a, T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<'a, Self>,
{
    fn inverse(&self) -> Self {
        Self::identity().inv_op(self.clone())
    }

    fn inv_op(self, other: Self) -> Self {
        T::operate(self, other.inverse())
    }
}
pub trait Commutativity<'a, T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn check_commutativity(x: Self, y: Self) -> bool {
        T::operate(x.clone(), y.clone()) == T::operate(y, x)
    }
}

pub trait RightDistributivity<'a, Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<'a, Self>,
    Add: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn check_right_distributivity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(Add::operate(x.clone(), y.clone()), z.clone())
            == Add::operate(
                Mul::operate(x.clone(), z.clone()),
                Mul::operate(y.clone(), z.clone()),
            )
    }
}

pub trait LeftDistributivity<'a, Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<'a, Self>,
    Add: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn check_left_distributivity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(x.clone(), Add::operate(y.clone(), z.clone()))
            == Add::operate(Mul::operate(x.clone(), y), Mul::operate(x.clone(), z))
    }
}

pub trait Distributivity<'a, Add, Mul>:
    RightDistributivity<'a, Add, Mul> + LeftDistributivity<'a, Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<'a, Self>,
    Add: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn check_distributivity(x: Self, y: Self, z: Self) -> bool {
        Self::check_left_distributivity(x.clone(), y.clone(), z.clone())
            && Self::check_right_distributivity(x, y, z)
    }
}

impl<'a, T, Add, Mul> Distributivity<'a, Add, Mul> for T
where
    T: LeftDistributivity<'a, Add, Mul> + RightDistributivity<'a, Add, Mul>,
    Add: InternalBinaryOperator<'a, T>,
    Mul: InternalBinaryOperator<'a, T>,
{
}

pub trait Absorbency<'a, Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn check_absorbency(x: Self, y: Self) -> bool {
        let add_xy = Add::operate(x.clone(), y.clone());
        let mul_xy = Mul::operate(x.clone(), y.clone());

        Mul::operate(x.clone(), add_xy) == x && Add::operate(x.clone(), mul_xy) == x
    }
}

pub trait Divisibility<'a, Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
}

pub trait LeftCancellative<'a, T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<'a, Self>,
{
}
pub trait RightCancellative<'a, T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<'a, Self>,
{
}
pub trait Cancellative<'a, T>: LeftCancellative<'a, T> + RightCancellative<'a, T>
where
    Self: Sized,
    T: InternalBinaryOperator<'a, Self>,
{
}

impl<'a, Op, T> Cancellative<'a, Op> for T
where
    T: LeftCancellative<'a, Op> + RightCancellative<'a, Op>,
    Op: InternalBinaryOperator<'a, T>,
{
}

pub trait Mediality<'a, T>
where
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn check_mediality(a: Self, b: Self, c: Self, d: Self) -> bool {
        let ab = T::operate(a.clone(), b.clone());
        let cd = T::operate(c.clone(), d.clone());
        let ac = T::operate(a, c);
        let bd = T::operate(b, d);

        T::operate(ab, cd) == T::operate(ac, bd)
    }
}

pub trait NoZeroDivisor {}

pub trait IntegrallyClosed {}

pub trait UniqueFactorizable {}

pub trait UniquePrimeFactorizable: UniqueFactorizable {}
