use std::ops::Add;
use std::ops::Sub;

use frunk::hlist::HList;
use frunk::*;
use typenum::uint::Unsigned;
use typenum::*;

pub trait TypeLength {
    type Length: Unsigned; // HList length (type int)
}

impl TypeLength for HNil {
    type Length = U0;
}

impl<Head, Tail> TypeLength for HCons<Head, Tail>
where
    //Head: Unsigned,
    Tail: HList + TypeLength,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Length = <<Tail as TypeLength>::Length as Add<U1>>::Output;
}

pub trait IndexShape {
    // all usize hlist
    type Shape: HList;
    //type Capacity;

    fn get_index(index: Self::Shape) -> (usize, usize);
    fn get_capacity() -> usize;
}

impl IndexShape for HNil {
    type Shape = HNil;
    //type Capacity = U1;

    fn get_index(_: Self::Shape) -> (usize, usize) {
        (0, 1)
    }

    fn get_capacity() -> usize {
        1
    }
}

impl<Head, Tail> IndexShape for HCons<Head, Tail>
where
    Head: Unsigned,
    //Head: std::ops::Mul<<Tail as IndexShape>::Capacity>,
    Tail: HList + IndexShape,
    //<Tail as IndexShape>::Capacity: Unsigned,
{
    type Shape = HCons<usize, <Tail as IndexShape>::Shape>;
    //type Capacity = Prod<Head, <Tail as IndexShape>::Capacity>;

    fn get_index(index: Self::Shape) -> (usize, usize) {
        let (h, tail) = index.pop();
        let (offset, width) = Tail::get_index(tail);
        assert!(h < Head::to_usize());
        (h * width + offset, Head::to_usize())
    }

    fn get_capacity() -> usize {
        Head::to_usize() * Tail::get_capacity()
    }
}

/*
//HListのAddが対応

pub trait HAppendable<RHS> {
    type Appended: HList;

    fn append(self, other: RHS) -> Self::Appended;
}

impl<T: HList> HAppendable<T> for HNil {
    type Appended = T;

    fn append(self, rhs: T) -> Self::Appended {
        rhs
    }
}

impl<H1, T1, RHS> HAppendable<RHS> for HCons<H1, T1>
where
    T1: HList + HAppendable<RHS>,
    RHS: HList,
{
    type Appended = HCons<H1, <T1 as HAppendable<RHS>>::Appended>;

    fn append(self, rhs: RHS) -> Self::Appended {
        HCons {
            head: self.head,
            tail: self.tail.append(rhs),
        }
    }
}
*/

pub trait HSliceable<F, T>: TypeLength
where
    // 0 <= F <= T <= <Self as TypeLength>::Length
    F: Unsigned, // + IsGreaterOrEqual<U0, Output = B1> + IsLessOrEqual<T, Output = B1>,
    T: Unsigned,
    //+ IsGreaterOrEqual<F, Output = B1>
    //+ IsLessOrEqual<<Self as TypeLength>::Length, Output = B1>,
    Self: HList + TypeLength,
{
    type Sliced: HList;
    fn slice(self, from: F, to: T) -> Self::Sliced;
}

impl HSliceable<UTerm, UTerm> for HNil {
    type Sliced = HNil;

    fn slice(self, _: U0, _: U0) -> Self::Sliced {
        HNil
    }
}

/*
impl<Head, Tail> HSliceable<UTerm, UTerm> for HCons<Head, Tail>
where
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = HNil;

    fn slice(self, _: U0, _: U0) -> Self::Sliced {
        HNil
    }
}
*/

/*
impl<U, Head, Tail> HSliceable<UTerm, U> for HCons<Head, Tail>
where
    U: Unsigned + Sub<B1>,
    Sub1<U>: Unsigned,
    Tail: HList + TypeLength + HSliceable<UTerm, <U as Sub<B1>>::Output>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = HCons<Head, <Tail as HSliceable<UTerm, <U as Sub<B1>>::Output>>::Sliced>;

    fn slice(self, from: UTerm, to: U) -> Self::Sliced {
        let to = to - B1::new();
        HCons {
            head: self.head,
            tail: self.tail.slice(from, to),
        }
    }
}
*/

impl<U: Unsigned, B: Bit, Head, Tail> HSliceable<UTerm, UInt<UInt<U, B>, B0>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, UInt<U, B>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = HCons<Head, <Tail as HSliceable<UTerm, UInt<U, B>>>::Sliced>;

    fn slice(self, from: UTerm, to: UInt<UInt<U, B>, B0>) -> Self::Sliced {
        let to: UInt<U, B> = UInt::new();
        HCons {
            head: self.head,
            tail: self.tail.slice(from, to),
        }
    }
}

impl<U: Unsigned, Head, Tail> HSliceable<UTerm, UInt<U, B1>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, UInt<U, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = HCons<Head, <Tail as HSliceable<UTerm, UInt<U, B0>>>::Sliced>;

    fn slice(self, from: UTerm, to: UInt<U, B1>) -> Self::Sliced {
        let to: UInt<U, B0> = UInt::new();
        HCons {
            head: self.head,
            tail: self.tail.slice(from, to),
        }
    }
}

/*
impl<U1: Unsigned, U2: Unsigned, Head, Tail> HSliceable<UInt<U1, B1>, UInt<U2, B1>>
    for HCons<Head, Tail>
where
    Self: TypeLength,
    Tail: HList + TypeLength + HSliceable<UInt<U1, B0>, UInt<U2, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = <Tail as HSliceable<UInt<U1, B0>, UInt<U2, B0>>>::Sliced;

    fn slice(self, from: UInt<U1, B1>, to: UInt<U2, B1>) -> Self::Sliced {
        let from: UInt<U1, B0> = UInt::new();
        let to: UInt<U2, B0> = UInt::new();
        self.tail.slice(from, to)
    }
}

impl<U1: Unsigned, U2: Unsigned, B: Bit, Head, Tail> HSliceable<UInt<U1, B1>, UInt<UInt<U2, B>, B0>>
    for HCons<Head, Tail>
where
    Self: TypeLength,
    Tail: HList + TypeLength + HSliceable<UInt<U1, B0>, UInt<U2, B>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = <Tail as HSliceable<UInt<U1, B0>, UInt<U2, B>>>::Sliced;

    fn slice(self, from: UInt<U1, B1>, to: UInt<UInt<U2, B>, B0>) -> Self::Sliced {
        let from: UInt<U1, B0> = UInt::new();
        let to: UInt<U2, B> = UInt::new();
        self.tail.slice(from, to)
    }
}

impl<U1: Unsigned, U2: Unsigned, B: Bit, Head, Tail> HSliceable<UInt<UInt<U1, B>, B0>, UInt<U2, B1>>
    for HCons<Head, Tail>
where
    Self: TypeLength,
    Tail: HList + TypeLength + HSliceable<UInt<U1, B>, UInt<U2, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = <Tail as HSliceable<UInt<U1, B>, UInt<U2, B0>>>::Sliced;

    fn slice(self, from: UInt<UInt<U1, B>, B0>, to: UInt<U2, B1>) -> Self::Sliced {
        let from: UInt<U1, B> = UInt::new();
        let to: UInt<U2, B0> = UInt::new();
        self.tail.slice(from, to)
    }
}

impl<U1: Unsigned, U2: Unsigned, BU1: Bit, BU2: Bit, Head, Tail>
    HSliceable<UInt<UInt<U1, BU1>, B0>, UInt<UInt<U2, BU2>, B0>> for HCons<Head, Tail>
where
    Self: TypeLength,
    Tail: HList + TypeLength + HSliceable<UInt<U1, BU1>, UInt<U2, BU2>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = <Tail as HSliceable<UInt<U1, BU1>, UInt<U2, BU2>>>::Sliced;

    fn slice(self, from: UInt<UInt<U1, BU1>, B0>, to: UInt<UInt<U2, BU2>, B0>) -> Self::Sliced {
        let from: UInt<U1, BU1> = UInt::new();
        let to: UInt<U2, BU2> = UInt::new();
        self.tail.slice(from, to)
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::util::*;
    use frunk::hlist::HList;
    use frunk::*;
    use typenum::uint::Unsigned;
    use typenum::*;

    #[test]
    fn append() {
        let h1 = hlist![1, "hi"];
        let h2 = hlist!["hoge", 11];

        //assert_eq!(h1.append(h2), hlist![1, "hi", "hoge", 11]);
        assert_eq!(h1 + h2, hlist![1, "hi", "hoge", 11]);
    }

    #[test]
    fn type_length() {
        assert_type_eq!(<Hlist!(U1, U1, U1) as TypeLength>::Length, U3);
    }

    #[test]
    fn slice_hnil() {
        let h = hlist![];
        let s = h.slice(UTerm::new(), UTerm::new());
    }

    #[test]
    fn slice() {
        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        //let sub = h.slice(U0::new(), U1::new());
        //assert_eq!(sub, hlist![1, "hi", U1::new()]);
    }
}
