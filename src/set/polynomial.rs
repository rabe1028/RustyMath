use crate::axiom::*;
use crate::operator::*;
use crate::property::*;
use std::cmp::Ordering;
use std::ops::*;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Polynomial<Coeff>
where
    Coeff: Ring<Addition, Multiplication>,
    Addition: InternalBinaryOperator<Coeff>,
    Multiplication: InternalBinaryOperator<Coeff>,
{
    a: Vec<Coeff>,
}

impl<T> Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    #[inline(always)]
    pub fn fron_vec(a: Vec<T>) -> Self {
        Self { a }
    }

    #[inline(always)]
    pub fn reduce(mut self) -> Self {
        if self.a[self.degree() - 1] == T::identity() {
            self.a.pop();
            self.reduce()
        } else {
            self
        }
    }

    #[inline(always)]
    pub fn degree(&self) -> usize {
        self.a.len()
    }
}

macro_rules! forward_one_ref_binop {
    ($op:ty) => {
        impl<'a, T> BinaryOperator<&'a Polynomial<T>, Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn operate(lhs: &'a Polynomial<T>, rhs: Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(lhs, &rhs)
            }
        }

        impl<'a, T> BinaryOperator<Polynomial<T>, &'a Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn operate(lhs: Polynomial<T>, rhs: &'a Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(&lhs, rhs)
            }
        }

        impl<T> BinaryOperator<Polynomial<T>, Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn operate(lhs: Polynomial<T>, rhs: Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(&lhs, &rhs)
            }
        }
    };
}

macro_rules! forward_inter_binop {
    ($op:ty, ($l:ident, $r:ident) => $x: expr) => {
        impl<T> InternalBinaryOperator<Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
        }

        impl<'a, T> BinaryOperator<&'a Polynomial<T>, &'a Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;
            fn operate($l: &'a Polynomial<T>, $r: &'a Polynomial<T>) -> Self::Output {
                $x
            }
        }

        forward_one_ref_binop! {$op}
    };
}

macro_rules! impl_helper {
    (impl $type:ident  $(< $( $lt:tt ),+ >)? ) => {
        impl<T> $type $(< $( $lt ),+ >)? for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {}
    };
    (
        impl $type:ident $(< $( $lt:tt ),+ >)?,
        $(#[$attr:meta])* fn $name:ident $args:tt -> $ret:ty $body:block
    ) => {
        impl<T> $type $(< $( $lt ),+ >)? for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            $(#[$attr])* fn $name $args -> $ret $body
        }
    };
}

macro_rules! forward_assign {
    (impl $type:ident, fn $name:ident, $op:ty) => {
        impl<T> $type<Polynomial<T>> for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            fn $name(&mut self, other: Polynomial<T>) {
                *self = <$op as BinaryOperator<&Polynomial<T>, Polynomial<T>>>::operate(self, other)
            }
        }

        impl<'a, T> $type<&'a Polynomial<T>> for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            fn $name(&mut self, other: &'a Polynomial<T>) {
                *self =
                    <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(self, other)
            }
        }
    };
}

forward_inter_binop! { Addition,
    (lhs, rhs) => {
        use std::cmp::min;
        let min_size = min(lhs.a.len(), rhs.a.len());

        let a = (0..min_size).map(|i|
                Addition::operate(lhs.a[i].clone(),rhs.a[i].clone())
            ).collect();

        Polynomial {a}.reduce()
    }
}

forward_assign! {impl AddAssign, fn add_assign, Addition}

impl_helper! {impl Totality<Addition>}
impl_helper! {impl Associativity<Addition>}
impl_helper! {impl Commutativity<Addition>}

impl_helper! {impl Identity<Addition>,
    #[inline(always)]
    fn identity() -> Self {
        Polynomial { a: vec![] }
    }
}

impl_helper! {impl Invertivility<Addition>,
    #[inline(always)]
    fn inverse(&self) -> Self {
        Polynomial {
            a: self.a.iter().map(|i| i.inverse()).collect()
        }
    }
}

//  Convolution like Mul
forward_inter_binop! { Multiplication,
    (lhs, rhs) => {
        let out_len = lhs.a.len() + rhs.a.len();
        let mut a = vec![T::identity(); out_len];

        for l in 0..lhs.a.len() {
            for r in 0..rhs.a.len() {
                a[l+r] = Addition::operate(
                    Multiplication::operate(
                        lhs.a[l].clone(), rhs.a[r].clone()
                    ),
                    a[l+r].clone(),
                )
            }
        }

        Polynomial {a}.reduce()
    }
}

forward_assign! {impl MulAssign, fn mul_assign, Multiplication}

impl_helper! {impl Totality<Multiplication>}
impl_helper! {impl Associativity<Multiplication>}

// impl_helper! {impl Identity<Multiplication>,
//     #[inline(always)]
//     fn identity() -> Self {
//         Polynomial {
//             a: vec![ T::one() ]
//         }
//     }
// }

impl<T> Identity<Multiplication> for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Monoid<Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    #[inline(always)]
    fn identity() -> Self {
        Polynomial { a: vec![T::one()] }
    }
}

// impl_helper! {impl Commutativity<Multiplication>}

impl<T> Commutativity<Multiplication> for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Commutativity<Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
}

impl_helper! {impl RightDistributivity<Addition, Multiplication>}
impl_helper! {impl LeftDistributivity<Addition, Multiplication>}

// 多項式に距離を定義
// 距離は整列順序集合になるから，
// それを比較する

impl<T> Ord for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Eq + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.degree().cmp(&other.degree())
    }
}

impl<T> PartialOrd for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Eq + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl_helper! {impl NoZeroDivisor}
impl_helper! {impl IntegrallyClosed}
impl_helper! {impl UniqueFactorizable}
impl_helper! {impl UniquePrimeFactorizable}

impl<T> EuclidianDomain<Addition, Multiplication> for Polynomial<T>
where
    T: EuclidianDomain<Addition, Multiplication>  + Eq + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn div(&self, other: &Self) -> Self {
        assert!(!other.is_zero());

        unimplemented!()
    }
}
