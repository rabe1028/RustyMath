use std::borrow::Cow;

use crate::operator::*;

pub trait Totality<T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<Self>,
{
}

pub trait Associativity<T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<Self>,
{
    fn check_associativity<'a, 'b, 'c, 'd: 'a + 'b + 'c, L, M, N>(x: L, y: M, z: N) -> bool
    where
        Self: 'd,
        L: Into<Cow<'a, Self>>,
        M: Into<Cow<'b, Self>>,
        N: Into<Cow<'c, Self>>,
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        <T as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(
                y.clone(),
                z.clone(),
            )),
        ) == <T as InternalBinaryOperator<Self>>::operate(
            Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(x, y)),
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
    fn check_identity<'a, 'b: 'a>(x: impl Into<Cow<'a, Self>>) -> bool
    where
        Self: 'b,
    {
        let x = x.into();
        let id: Cow<'_, Self> = Cow::Owned(Self::identity());
        let left: Cow<'a, Self> = Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            id.clone(),
        ));
        let right: Cow<'a, Self> = Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(
            id.clone(),
            x.clone(),
        ));
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
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_commutativity<'a, 'b, 'c: 'a + 'b>(
        x: impl Into<Cow<'a, Self>>,
        y: impl Into<Cow<'b, Self>>,
    ) -> bool
    where
        Self: 'c,
    {
        let x = x.into();
        let y = y.into();
        <T as InternalBinaryOperator<Self>>::operate(x.clone(), y.clone())
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
    fn check_right_distributivity<'a, 'b, 'c, 'd: 'a + 'b + 'c, L, M, N>(x: L, y: M, z: N) -> bool
    where
        Self: 'd,
        L: Into<Cow<'a, Self>>,
        M: Into<Cow<'b, Self>>,
        N: Into<Cow<'c, Self>>,
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        <Mul as InternalBinaryOperator<Self>>::operate(
            Cow::Owned(<Add as InternalBinaryOperator<Self>>::operate(
                x.clone(),
                y.clone(),
            )),
            z.clone(),
        ) == <Add as InternalBinaryOperator<Self>>::operate(
            Cow::Owned(<Mul as InternalBinaryOperator<Self>>::operate(
                x.clone(),
                z.clone(),
            )),
            Cow::Owned(<Mul as InternalBinaryOperator<Self>>::operate(
                y.clone(),
                z.clone(),
            )),
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
    fn check_left_distributivity<'a, 'b, 'c, 'd: 'a + 'b + 'c, L, M, N>(x: L, y: M, z: N) -> bool
    where
        Self: 'd,
        L: Into<Cow<'a, Self>>,
        M: Into<Cow<'b, Self>>,
        N: Into<Cow<'c, Self>>,
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();

        <Mul as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            Cow::Owned(<Add as InternalBinaryOperator<Self>>::operate(
                y.clone(),
                z.clone(),
            )),
        ) == <Add as InternalBinaryOperator<Self>>::operate(
            Cow::Owned(<Mul as InternalBinaryOperator<Self>>::operate(x.clone(), y)),
            Cow::Owned(<Mul as InternalBinaryOperator<Self>>::operate(x.clone(), z)),
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
    fn check_distributivity<'a, 'b, 'c, 'd: 'a + 'b + 'c, L, M, N>(x: L, y: M, z: N) -> bool
    where
        Self: 'd,
        L: Into<Cow<'a, Self>>,
        M: Into<Cow<'b, Self>>,
        N: Into<Cow<'c, Self>>,
    {
        let x = x.into();
        let y = y.into();
        let z = z.into();
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
    Self: Sized + PartialEq + Copy,
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_absorbency<'a, 'b, 'c: 'a + 'b>(
        x: impl Into<Cow<'a, Self>>,
        y: impl Into<Cow<'b, Self>>,
    ) -> bool
    where
        Self: 'c,
    {
        let x = x.into();
        let y = y.into();

        let add_xy = Cow::Owned(<Add as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            y.clone(),
        ));
        let mul_xy = Cow::Owned(<Mul as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            y.clone(),
        ));

        Cow::Borrowed(&<Mul as InternalBinaryOperator<Self>>::operate(
            x.clone(),
            add_xy,
        )) == x
            && Cow::Borrowed(&<Add as InternalBinaryOperator<Self>>::operate(
                x.clone(),
                mul_xy,
            )) == x
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
    fn check_mediality<'a, 'b, 'c, 'd, 'long: 'a + 'b + 'c + 'd, A, B, C, D>(
        a: A,
        b: B,
        c: C,
        d: D,
    ) -> bool
    where
        Self: 'long,
        A: Into<Cow<'a, Self>>,
        B: Into<Cow<'b, Self>>,
        C: Into<Cow<'c, Self>>,
        D: Into<Cow<'d, Self>>,
    {
        let a = a.into();
        let b = b.into();
        let c = c.into();
        let d = d.into();

        let ab = Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(
            a.clone(),
            b.clone(),
        ));
        let cd = Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(
            c.clone(),
            d.clone(),
        ));
        let ac = Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(a, c));
        let bd = Cow::Owned(<T as InternalBinaryOperator<Self>>::operate(b, d));

        <T as InternalBinaryOperator<Self>>::operate(ab, cd)
            == <T as InternalBinaryOperator<Self>>::operate(ac, bd)
    }
}
