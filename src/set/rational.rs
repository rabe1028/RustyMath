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

macro_rules! impl_helper {
    (impl $type:ident  $(< $( $lt:tt ),+ >)? ) => {
        impl<T> $type $(< $( $lt ),+ >)? for Rational<T>
        where
            T: UnitalRing<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {}
    };
    (
        impl $type:ident $(< $( $lt:tt ),+ >)?,
        $(#[$attr:meta])* fn $name:ident $args:tt -> $ret:ty $body:block
    ) => {
        impl<T> $type $(< $( $lt ),+ >)? for Rational<T>
        where
            T: UnitalRing<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            $(#[$attr])* fn $name $args -> $ret $body
        }
    };
}

// 20200531
// Rational<T>のTの制約をUnitalRihg + Cloneのみにすると
// Rational<Rational<T>>も存在できるから
// InterlanBinaryOperatorを再起的に定義してしまって，死ぬ
// -> Auto Traitのときのみ起こる
// -> genericsによる定義だと問題ないけど，Auto Implだと問題になる
// -> Auto Implだと，発生しうる型全てを探索するが，Genericsは使われている型に限るみたい？

#[repr(C)]
#[derive(Debug, Clone)]
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
            if denom == T::zero() {
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
        Multiplication::operate(self.numer.clone(), other.denom.clone())
            == Multiplication::operate(self.numer.clone(), other.denom.clone())
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

impl_helper! {impl Totality<Addition>}

impl_helper! {impl Associativity<Addition>}

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

impl_helper! {impl Invertivility<Addition>,
    #[inline(always)]
    fn inverse(&self) -> Self {
        Rational::new_unchecked(self.numer.negation(), self.denom.clone())
    }
}

impl_helper! {impl Commutativity<Addition>}

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

impl<T> InternalBinaryOperator<Rational<T>> for Multiplication
where
    T: UnitalRing<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
}

impl_helper! {impl Totality<Multiplication>}
impl_helper! {impl Associativity<Multiplication>}

impl_helper! {impl RightDistributivity<Addition, Multiplication>}
impl_helper! {impl LeftDistributivity<Addition, Multiplication>}

impl_helper! {impl Commutativity<Multiplication>}

impl_helper! {impl Invertivility<Multiplication>,
    #[inline(always)]
    fn inverse(&self) -> Self {
        Rational::new_unchecked(self.denom.clone(), self.numer.clone())
    }
}

impl_helper! {impl Identity<Multiplication>,
    #[inline(always)]
    fn identity() -> Self {
        Rational::new_unchecked(T::one(), T::one())
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
