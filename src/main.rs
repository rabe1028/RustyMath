use std::ops::{Add, Sub, Mul, Div};

/*
trait Magma {
    fn op(self, rhs: Self) -> Self;
}

trait RightIdentityElement {}
trait LeftIdentityElement {}


impl<T> Magma for T
    where T: Add<Output=T> {
    fn op(self, rhs: Self) -> Self {
        self + rhs
    }
}

trait SubGroup: Magma {
    fn op(self, rhs: Self) -> Self;
}
*/

trait IdentityElement {}

trait InverseElement: IdentityElement {}

trait Zero: Sized + Add<Output=Self> + IdentityElement + PartialEq {
    fn zero() -> Self;
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

trait One: Sized + Mul<Output=Self> + IdentityElement + PartialEq {
    fn one() -> Self;
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

trait AddInverse: Sized + Sub<Output=Self> + Zero{
    fn inverse(arg: Self) -> Self {
        Self::zero() - arg
    }
}

trait MulInverse: Sized + One{
    fn inverse(arg: Self) -> Option<Self>;
}

trait AdditiveGroup: Sized+Add+Sub+Zero+AddInverse {}

trait Ring: AdditiveGroup+Mul+MulInverse {}

trait Field: Ring+Div{}

impl IdentityElement for isize {}
impl Zero for isize {
    fn zero() -> Self { 0 }
}

impl AddInverse for isize {}
impl AdditiveGroup for isize {}

impl One for isize {
    fn one() -> Self { 1 }
}
impl MulInverse for isize {
    fn inverse(arg: Self) -> Option<Self> {
        if arg * arg == 1 {
            Some(1)
        } else {
            None
        }
    }
}

impl Ring for isize {}
impl Field for isize {}

fn main() {
    println!("Hello, world!");
}
