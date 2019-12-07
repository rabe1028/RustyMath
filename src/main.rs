use num::CheckedDiv;
use std::error::Error;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

//use num_bigint as bigint;

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

trait Zero: Sized + Add<Output = Self> + IdentityElement + PartialEq {
    fn zero() -> Self;
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

//type Zero = AddIdentity;
//type Zero = Identity<Addition>

trait One: Sized + Mul<Output = Self> + IdentityElement + PartialEq {
    fn one() -> Self;
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

//type One = MulIdentity;
//type One = Identity<Multiplication>

trait AddInverse: Sized + Neg<Output = Self> + Copy + Zero {
    fn inverse(&self) -> Self {
        -*self
    }
}

trait MulInverse: Sized + One {
    fn inverse(arg: Self) -> Option<Self>;
}

trait AdditiveGroup: Sized + Add + Sub + Zero + AddInverse {}

trait Ring: AdditiveGroup + Mul + One + MulInverse {}

trait Field: Ring + Div {}

// 整数に対して定義

impl IdentityElement for isize {}
impl Zero for isize {
    fn zero() -> Self {
        0
    }
}

impl AddInverse for isize {}
impl AdditiveGroup for isize {}

impl One for isize {
    fn one() -> Self {
        1
    }
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

#[derive(Clone, Copy, PartialEq)]
struct RationalNumber {
    p: isize,
    q: isize,
}

#[derive(Debug)]
struct ValueError;

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot division by zero")
    }
}

impl Error for ValueError {
    fn description(&self) -> &str {
        "Zero division error"
    }
}

impl RationalNumber {
    fn new(p: isize, q: isize) -> Result<Self, ValueError> {
        if q == 0 {
            Err(ValueError)
        } else {
            Ok(RationalNumber { p, q })
        }
    }
}

impl IdentityElement for RationalNumber {}

impl Add for RationalNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        RationalNumber {
            p: self.p + rhs.p,
            q: self.q + rhs.q,
        }
    }
}

impl Zero for RationalNumber {
    fn zero() -> Self {
        RationalNumber { p: 0, q: 1 }
    }
}

//impl Ring for RationalNumber {}

//impl Field for RationalNumber {}

fn main() {
    let a = 0u32;
    //let b = <u32 as CheckedDiv>::checked_div(&2, &a);
    //let b = u32::checked_div(2, a);
    let b = 2.checked_div(&a);
    println!("{:?}", b);

    //let b = 2 / a;

    println!("Test Division : {}", 1 / a);
    println!("Hello, world!");
}
