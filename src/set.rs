pub mod rational_number;

pub use rational_number::*;

pub mod tensor;

pub use tensor::*;

pub mod basic_array;

use frunk::*;

macro_rules! impl_scalar {
    ($($ty: ty),*) => {
        $(
            impl Tensor<$ty, HNil, HNil> for $ty {
                type Joined = HNil;
                fn index<I: Into<HNil>, J: Into<HNil>>(&self, cont: I, cov: J) -> &Self {
                    &self
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

impl_scalar! {
    isize, i8, i16, i32, i64, usize, u8, u16, u32, u64, f32, f64
}
