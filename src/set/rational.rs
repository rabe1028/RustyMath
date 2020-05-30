#[macro_use(forward_internal_binop)]
use crate::operator::*;
use crate::property::*;
use crate::set::*;

// https://en.wikipedia.org/wiki/Field_of_fractions

macro_rules! forward_one_ref_binop {
    ($op:ty) => {
        impl<'a, T> BinaryOperator<&'a Rational<T>, Rational<T>> for $op
        where
            T: UnitalRing<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Rational<T>;

            #[inline]
            fn operate(lhs: &'a Rational<T>, rhs: Rational<T>) -> Self::Output {
                <$op as BinaryOperator<&Rational<T>, &Rational<T>>>::operate(lhs, &rhs)
            }
        }

        impl<'a, T> BinaryOperator<Rational<T>, &'a Rational<T>> for $op
        where
            T: UnitalRing<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Rational<T>;

            #[inline]
            fn operate(lhs: Rational<T>, rhs: &'a Rational<T>) -> Self::Output {
                <$op as BinaryOperator<&Rational<T>, &Rational<T>>>::operate(&lhs, rhs)
            }
        }

        impl<T> BinaryOperator<Rational<T>, Rational<T>> for $op
        where
            T: UnitalRing<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Rational<T>;

            #[inline]
            fn operate(lhs: Rational<T>, rhs: Rational<T>) -> Self::Output {
                <$op as BinaryOperator<&Rational<T>, &Rational<T>>>::operate(&lhs, &rhs)
            }
        }
    };
}

macro_rules! forward_inter_binop {
    ($op:ty, ($l:ident, $r:ident) => $x: expr) => {
        impl<'a, T> BinaryOperator<&'a Rational<T>, &'a Rational<T>> for $op
        where
            T: UnitalRing<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Rational<T>;
            fn operate($l: &'a Rational<T>, $r: &'a Rational<T>) -> Self::Output {
                $x
            }
        }

        forward_one_ref_binop! {$op}
    };
}

#[repr(C)]
#[derive(Clone)]
struct Rational<T>
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    numer: T,
    denom: T,
}

type Rational32 = Rational<i32>;
type Rational64 = Rational<i64>;
type RationalNum = Rational<isize>;

macro_rules! maybe_const {
    ($( $(#[$attr:meta])* pub fn $name:ident $args:tt -> $ret:ty $body:block )*) => {$(
        #[cfg(has_const_fn)]
        $(#[$attr])* pub const fn $name $args -> $ret $body

        #[cfg(not(has_const_fn))]
        $(#[$attr])* pub fn $name $args -> $ret $body
    )*};
    ($( $(#[$attr:meta])* fn $name:ident $args:tt -> $ret:ty $body:block )*) => {$(
        #[cfg(has_const_fn)]
        $(#[$attr])* const fn $name $args -> $ret $body

        #[cfg(not(has_const_fn))]
        $(#[$attr])* fn $name $args -> $ret $body
    )*};
}

impl<T> Rational<T>
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    maybe_const! {
        #[inline]
        pub fn new(numer: T, denom: T) -> Option<Self> {
            if denom == <T as Identity<Addition>>::identity() {
                None
            } else {
                Some(Rational { numer, denom })
            }
        }

        #[inline]
        pub fn new_unchecked(numer: T, denom: T) -> Self {
            Rational { numer, denom }
        }
    }
}

impl<T> PartialEq<Self> for Rational<T>
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn eq(&self, other: &Self) -> bool {
        <Multiplication as InternalBinaryOperator<_>>::operate(
            self.numer.clone(),
            other.denom.clone(),
        ) == <Multiplication as InternalBinaryOperator<_>>::operate(
            self.numer.clone(),
            other.denom.clone(),
        )
    }
}

forward_inter_binop! { Addition,
    (lhs, rhs) => {
        Rational::new_unchecked(
            <Addition as BinaryOperator<_,_>>::operate(
                <Multiplication as BinaryOperator<_,_>>::operate(
                    lhs.numer.clone(),
                    rhs.denom.clone(),
                ),
                <Multiplication as BinaryOperator<_,_>>::operate(
                    rhs.numer.clone(),
                    lhs.denom.clone(),
                )
            ),
            <Multiplication as BinaryOperator<_,_>>::operate(
                lhs.denom.clone(),
                rhs.denom.clone(),
            )
        )
    }
}

impl<T> InternalBinaryOperator<Rational<T>> for Addition
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
}

impl<T> Totality<Addition> for Rational<T>
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
}

impl<T> Associativity<Addition> for Rational<T>
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
}
impl<T> Identity<Addition> for Rational<T>
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    maybe_const! {
        #[inline(always)]
        fn identity() -> Self {
            Rational::new_unchecked(T::zero(), T::one())
        }
    }
}

impl Invertivility<Addition> for Rational32 {
    #[inline(always)]
    fn inverse(&self) -> Self {
        Rational32::new_unchecked(-self.numer, self.denom)
    }
}

impl Commutativity<Addition> for Rational32 {}

forward_inter_binop! { Multiplication,
    (lhs, rhs) => {
        Rational::new_unchecked(
            <Multiplication as BinaryOperator<_,_>>::operate(
                lhs.numer.clone(),
                rhs.numer.clone(),
            ),
            <Multiplication as BinaryOperator<_,_>>::operate(
                lhs.denom.clone(),
                rhs.denom.clone(),
            )
        )
    }
}

impl InternalBinaryOperator<Rational32> for Multiplication {}

impl Totality<Multiplication> for Rational32 {}
impl Associativity<Multiplication> for Rational32 {}

impl RightDistributivity<Addition, Multiplication> for Rational32 {}
impl LeftDistributivity<Addition, Multiplication> for Rational32 {}
//impl Distributivity<Addition, Multiplication> for Rational32 {}

impl Commutativity<Multiplication> for Rational32 {}
impl Invertivility<Multiplication> for Rational32 {
    #[inline(always)]
    fn inverse(&self) -> Self {
        Rational32::new_unchecked(self.denom, self.numer)
    }
}

impl Identity<Multiplication> for Rational32 {
    #[inline(always)]
    fn identity() -> Self {
        Rational32::new_unchecked(i32::one(), i32::one())
    }
}

#[cfg(test)]
mod tests {
    use crate::set::rational::*;

    #[test]
    fn construct() {
        let a = Rational::new(1, 1);
        assert!(a.is_some());

        let a = Rational::new(1, 0);
        assert!(a.is_none());
    }

    #[cfg(has_const_fn)]
    #[test]
    fn test_const() {
        const P: i32 = 1;
        const Q: i32 = 1;
        const R: Rational32 = Rational::new(R, I);

        assert!(R.is_some());
    }

    #[cfg(has_const_fn)]
    #[test]
    fn test_const_unchecked() {
        const P: i32 = 1;
        const Q: i32 = 1;
        const R: Rational32 = Rational::new_unchecked(R, I);
    }

    #[cfg(has_const_fn)]
    #[test]
    fn test_const_identity() {
        const R: Rational32 = Rational::identity();
    }
}
