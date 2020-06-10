pub mod rational;
pub use rational::*;

pub mod basic_array;
pub use basic_array::*;

pub mod polynomial;
pub use polynomial::*;

pub mod function;
pub use function::*;

use crate::axiom::*;
use frunk::*;

use crate::operator::*;
use crate::property::*;

macro_rules! impl_field {
    ($($ty: ty),*) => {
        $(
            impl_euclidean_domain! { $ty }

            impl Invertivility<Multiplication> for $ty {
                type Inverse = $ty;

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

            impl EuclidianDomain<Addition, Multiplication> for $ty {
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
            impl_group! { $ty }

            impl Invertivility<Addition> for $ty {
                type Inverse = $ty;

                #[inline(always)]
                fn inverse(&self) -> Self {
                    -self
                }
            }

            impl Ring<Addition, Multiplication> for $ty {
                #[inline(always)]
                fn sub(self, other: Self) -> Self {
                    self - other
                }
            }
        )*
    }
}

macro_rules! impl_group {
    ($($ty: ty),*) => {
        $(
            impl Morphism for $ty {
                type Domain = ();
                type Codomain = ();
            }

            impl Endomorphism for $ty {
                type Object = ();
            }

            // No invertivility for addition op
            forward_internal_binop! {Addition, $ty, (lhs, rhs) => {
                lhs + rhs
            }}

            impl InternalBinaryOperator<$ty> for Addition {}

            impl Totality<Addition> for $ty {}
            impl Associativity<Addition, $ty, $ty> for $ty {}

            impl LeftIdentity<Addition> for $ty {
                #[inline(always)]
                fn left_identity() -> Self {
                    0 as $ty
                }
            }

            impl RightIdentity<Addition> for $ty {
                #[inline(always)]
                fn right_identity() -> Self {
                    0 as $ty
                }
            }

            impl Identity<Addition> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    0 as $ty
                }
            }

            impl Commutativity<Addition> for $ty {}

            forward_internal_binop! {Multiplication, $ty, (lhs, rhs) => {
                lhs * rhs
            }}

            impl InternalBinaryOperator<$ty> for Multiplication {}

            impl Totality<Multiplication> for $ty {}
            impl Associativity<Multiplication, $ty, $ty> for $ty {}

            impl Commutativity<Multiplication> for $ty {}

            impl LeftIdentity<Multiplication> for $ty {
                #[inline(always)]
                fn left_identity() -> Self {
                    1 as $ty
                }
            }

            impl RightIdentity<Multiplication> for $ty {
                #[inline(always)]
                fn right_identity() -> Self {
                    1 as $ty
                }
            }

            impl Identity<Multiplication> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    1 as $ty
                }
            }

            impl RightDistributivity<Addition, Multiplication> for $ty {}
            impl LeftDistributivity<Addition, Multiplication> for $ty {}
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

impl_group! {
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
