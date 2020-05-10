pub mod rational_number;

pub use rational_number::*;

pub mod basic_array;

use crate::axiom::*;
use frunk::*;

use std::borrow::Cow;

use crate::operator::*;
use crate::property::*;

macro_rules! impl_unital_ring {
    ($($ty: ty),*) => {
        $(
            impl BinaryOperator<$ty,$ty,$ty> for Addition {
                #[inline(always)]
                fn operate<'a, 'b>(
                    lhs: impl Into<Cow<'a, $ty>>,
                    rhs: impl Into<Cow<'b, $ty>>,
                ) -> $ty {
                    let lhs = match lhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    let rhs = match rhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    lhs + rhs
                }
            }

            impl InternalBinaryOperator<$ty> for Addition {}

            impl Totality<Addition> for $ty {}
            impl Associativity<Addition> for $ty {}
            impl Identity<Addition> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    0 as $ty
                }
            }

            impl Invertivility<Addition> for $ty {
                #[inline(always)]
                fn inverse(&self) -> Self {
                    -self
                }
            }

            impl Commutativity<Addition> for $ty {}

            impl BinaryOperator<$ty,$ty,$ty>  for Multiplication {
                #[inline(always)]
                fn operate<'a, 'b>(
                    lhs: impl Into<Cow<'a, $ty>>,
                    rhs: impl Into<Cow<'b, $ty>>,
                ) -> $ty {
                    let lhs = match lhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    let rhs = match rhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    lhs * rhs
                }
            }

            impl InternalBinaryOperator<$ty> for Multiplication {}

            impl Totality<Multiplication> for $ty {}
            impl Associativity<Multiplication> for $ty {}

            impl RightDistributivity<Addition, Multiplication> for $ty {}
            impl LeftDistributivity<Addition, Multiplication> for $ty {}

            impl Commutativity<Multiplication> for $ty {}

            impl Invertivility<Multiplication> for $ty {
                #[inline(always)]
                fn inverse(&self) -> Self {
                    1 as $ty / *self
                }
            }

            impl Identity<Multiplication> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    1 as $ty
                }
            }
        )*
    }
}

macro_rules! impl_unital_ring_unsigned {
    ($($ty: ty),*) => {
        $(
            impl BinaryOperator<$ty,$ty,$ty> for Addition {
                #[inline(always)]
                fn operate<'a, 'b>(
                    lhs: impl Into<Cow<'a, $ty>>,
                    rhs: impl Into<Cow<'b, $ty>>,
                ) -> $ty {
                    let lhs = match lhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    let rhs = match rhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    lhs + rhs
                }
            }

            impl InternalBinaryOperator<$ty> for Addition {}

            impl Totality<Addition> for $ty {}
            impl Associativity<Addition> for $ty {}
            impl Identity<Addition> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    0 as $ty
                }
            }

            impl Commutativity<Addition> for $ty {}

            impl BinaryOperator<$ty,$ty,$ty>  for Multiplication {
                #[inline(always)]
                fn operate<'a, 'b>(
                    lhs: impl Into<Cow<'a, $ty>>,
                    rhs: impl Into<Cow<'b, $ty>>,
                ) -> $ty {
                    let lhs = match lhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    let rhs = match rhs.into() {
                        Cow::Borrowed(b) => *b,
                        Cow::Owned(b) => b
                    };
                    lhs * rhs
                }
            }

            impl InternalBinaryOperator<$ty> for Multiplication {}

            impl Totality<Multiplication> for $ty {}
            impl Associativity<Multiplication> for $ty {}

            impl RightDistributivity<Addition, Multiplication> for $ty {}
            impl LeftDistributivity<Addition, Multiplication> for $ty {}

            impl Commutativity<Multiplication> for $ty {}

            impl Invertivility<Multiplication> for $ty {
                #[inline(always)]
                fn inverse(&self) -> Self {
                    1 as $ty / *self
                }
            }

            impl Identity<Multiplication> for $ty {
                #[inline(always)]
                fn identity() -> Self {
                    1 as $ty
                }
            }
        )*
    }
}

macro_rules! impl_scalar {
    ($($ty: ty),*) => {
        $(
            impl Tensor<$ty, HNil, HNil> for $ty {
                type Joined = HNil;
                fn index<I: Into<HNil>, J: Into<HNil>>(&self, _cont: I, _cov: J) -> &Self {
                    &self
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

impl_unital_ring! {
    isize, i8, i16, i32, i64, f32, f64
}

// impl_scalar! {
//     isize, i8, i16, i32, i64, usize, u8, u16, u32, u64, f32, f64
// }


impl_scalar! {
    isize, i8, i16, i32, i64, f32, f64
}