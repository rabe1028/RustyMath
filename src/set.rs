pub mod rational;
pub use rational::*;

pub mod basic_array;
pub use basic_array::*;

pub mod polynomial;
pub use polynomial::*;

use crate::axiom::*;
use frunk::*;

use crate::operator::*;
use crate::property::*;

macro_rules! impl_field {
    ($($ty: ty),*) => {
        $(
            impl_euclidean_domain! { $ty }

            impl<'a> Invertivility<'a, Multiplication> for $ty {
                #[inline(always)]
                fn inverse(&self) -> Self {
                    1 as $ty / *self
                }
            }
        )*
    }
}

macro_rules! impl_euclidean_domain {
    ($($ty: ty),*) => {
        $(
            impl_unital_ring! { $ty }

            impl NoZeroDivisor for $ty {}
            impl IntegrallyClosed for $ty {}
            impl UniqueFactorizable for $ty {}
            impl UniquePrimeFactorizable for $ty {}

            impl<'a> EuclidianDomain<'a, Addition, Multiplication> for $ty {
                #[inline(always)]
                fn div(&self, other: &Self) -> Self {
                    self / other
                }

                fn rem(&self, other: &Self) -> Self {
                    self % other
                }
            }
        )*
    }
}

macro_rules! impl_unital_ring {
    ($($ty: ty),*) => {
        $(
            impl_unital_ring_unsigned! { $ty }

            impl<'a> Invertivility<'a, Addition> for $ty {
                #[inline(always)]
                fn inverse(&self) -> Self {
                    -self
                }
            }
        )*
    }
}

macro_rules! impl_unital_ring_unsigned {
    ($($ty: ty),*) => {
        $(
            // No invertivility for addition op
            forward_internal_binop! {Addition, $ty, (lhs, rhs) => {
                lhs + rhs
            }}

            impl<'a> InternalBinaryOperator<'a, $ty> for Addition {}

            impl<'a> Totality<'a, Addition> for $ty {}
            impl<'a> Associativity<'a, Addition> for $ty {}
            impl<'a> Identity<'a, Addition> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    0 as $ty
                }
            }

            impl<'a> Commutativity<'a, Addition> for $ty {}

            forward_internal_binop! {Multiplication, $ty, (lhs, rhs) => {
                lhs * rhs
            }}

            impl<'a>  InternalBinaryOperator<'a, $ty> for Multiplication {}

            impl<'a>  Totality<'a, Multiplication> for $ty {}
            impl<'a>  Associativity<'a, Multiplication> for $ty {}

            impl<'a>  Commutativity<'a, Multiplication> for $ty {}

            impl<'a>  Identity<'a, Multiplication> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    1 as $ty
                }
            }

            impl<'a>  RightDistributivity<'a, Addition, Multiplication> for $ty {}
            impl<'a>  LeftDistributivity<'a, Addition, Multiplication> for $ty {}
        )*
    }
}

macro_rules! impl_scalar {
    ($($ty: ty),*) => {
        $(
            impl Tensor<$ty, HNil, HNil> for $ty {
                type Joined = HNil;
                fn index<I: Into<HNil>, J: Into<HNil>>(&self, _cont: I, _cov: J) -> &Self {
                    self
                }

                fn index_mut<I: Into<HNil>, J: Into<HNil>>(&mut self, _cont: I, _cov: J) -> &mut Self {
                    self
                }

                fn from_vec(vec: Vec<$ty>) -> Self {
                    assert!(vec.len() == 1);
                    vec[0]
                }
            }

            impl Scalar<$ty> for $ty {
                fn new(elem: $ty) -> Self {
                    elem
                }

                fn get(&self) -> &$ty {
                    &self
                }
            }
        )*
    }
}

impl_unital_ring_unsigned! {
    usize, u8, u16, u32, u64
}

impl_euclidean_domain! {
    isize, i8, i16, i32, i64
}

impl_field! {
    f32, f64
}

impl_scalar! {
    isize, i8, i16, i32, i64, f32, f64
}
