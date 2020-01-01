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
    Tail: HList + IndexShape,
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
Slice Implementation
ref : https://docs.rs/typenum/1.10.0/typenum/uint/struct.UInt.html

HCons Impl
- U0(UTerm)
- U1(UInt<UTerm,B1>)
- UInt<U,B0>
- UInt<UInt<U,B>,B1>

0, 1, Odd, Even(>1)に構造で分ける
- Trait制約だけだと，重複implが起きる

UInt<UInt<U,B>,B1>は U1 を除外したいから

Caution:
UInt<UInt<U,B>,B1> - B1 not equal UInt<U,B>
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

/*
U0 to
- U0(UTerm)
- U1(UInt<UTerm,B1>)
- UInt<U,B0> (U2, U4...)
- UInt<UInt<U,B>,B1> (U3, U5...)
*/
impl HSliceable<UTerm, UTerm> for HNil {
    /*
        from 0 to 0 => return HNil
    */
    type Sliced = HNil;

    fn slice(self, _: U0, _: U0) -> Self::Sliced {
        HNil
    }
}

impl<Head, Tail> HSliceable<UTerm, UTerm> for HCons<Head, Tail>
where
    Tail: HList + TypeLength,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    /*
        from 0 to 0 => return HNil
    */
    type Sliced = HNil;

    fn slice(self, _: U0, _: U0) -> Self::Sliced {
        HNil
    }
}

impl<Head, Tail> HSliceable<UTerm, UInt<UTerm, B1>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    /*
        from 0 to 1 => return Hlist!(Head)
    */
    type Sliced = HCons<Head, HNil>;

    fn slice(self, _: U0, _: UInt<UTerm, B1>) -> Self::Sliced {
        HCons {
            head: self.head,
            tail: HNil,
        }
    }
}

impl<U: Unsigned, B: Bit, Head, Tail> HSliceable<UTerm, UInt<UInt<U, B>, B0>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, Sub1<UInt<UInt<U, B>, B0>>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<U, B>, B0>: Sub<B1>,
    Sub1<UInt<UInt<U, B>, B0>>: Unsigned,
{
    /*
        from 0 to k => return head + tail slice 0 to k-1
    */
    type Sliced = HCons<Head, <Tail as HSliceable<UTerm, Sub1<UInt<UInt<U, B>, B0>>>>::Sliced>;

    fn slice(self, from: UTerm, to: UInt<UInt<U, B>, B0>) -> Self::Sliced {
        HCons {
            head: self.head,
            tail: self.tail.slice(from, to - B1::new()),
        }
    }
}

impl<U: Unsigned, B: Bit, Head, Tail> HSliceable<UTerm, UInt<UInt<U, B>, B1>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, UInt<UInt<U, B>, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    /*
        from 0 to k => return head + tail slice 0 to k-1
    */
    type Sliced = HCons<Head, <Tail as HSliceable<UTerm, UInt<UInt<U, B>, B0>>>::Sliced>;

    fn slice(self, from: UTerm, to: UInt<UInt<U, B>, B1>) -> Self::Sliced {
        HCons {
            head: self.head,
            tail: self.tail.slice(from, to - B1::new()),
        }
    }
}

/*
U1 to
- U1(UInt<UTerm,B1>)
- UInt<U,B0>
- UInt<UInt<U,B>,B1>
*/
impl<Head, Tail> HSliceable<U1, U1> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, UTerm>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    /*
        from 1 to 1 => return HNil
    */
    type Sliced = <Tail as HSliceable<UTerm, UTerm>>::Sliced;

    fn slice(self, _: U1, _: U1) -> Self::Sliced {
        self.tail.slice(U0::new(), U0::new())
    }
}

impl<U: Unsigned, B: Bit, Head, Tail> HSliceable<U1, UInt<UInt<U, B>, B0>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, Sub1<UInt<UInt<U, B>, B0>>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<U, B>, B0>: Sub<B1>,
    Sub1<UInt<UInt<U, B>, B0>>: Unsigned,
{
    type Sliced = <Tail as HSliceable<UTerm, Sub1<UInt<UInt<U, B>, B0>>>>::Sliced;

    fn slice(self, _: U1, to: UInt<UInt<U, B>, B0>) -> Self::Sliced {
        self.tail.slice(U0::new(), to - B1::new())
    }
}

impl<U: Unsigned, B: Bit, Head, Tail> HSliceable<U1, UInt<UInt<U, B>, B1>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UTerm, UInt<UInt<U, B>, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = <Tail as HSliceable<UTerm, UInt<UInt<U, B>, B0>>>::Sliced;

    fn slice(self, _: U1, to: UInt<UInt<U, B>, B1>) -> Self::Sliced {
        self.tail.slice(U0::new(), to - B1::new())
    }
}

/*
UInt<U,B0> to
- UInt<U,B0>
- UInt<UInt<U,B>,B1>
*/

impl<_1: Unsigned, _2: Unsigned, _B1: Bit, _B2: Bit, Head, Tail>
    HSliceable<UInt<UInt<_1, _B1>, B0>, UInt<UInt<_2, _B2>, B0>> for HCons<Head, Tail>
where
    Tail: HList
        + TypeLength
        + HSliceable<Sub1<UInt<UInt<_1, _B1>, B0>>, Sub1<UInt<UInt<_2, _B2>, B0>>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<_1, _B1>, B0>: Sub<B1>,
    Sub1<UInt<UInt<_1, _B1>, B0>>: Unsigned,
    UInt<UInt<_2, _B2>, B0>: Sub<B1>,
    Sub1<UInt<UInt<_2, _B2>, B0>>: Unsigned,
{
    type Sliced =
        <Tail as HSliceable<Sub1<UInt<UInt<_1, _B1>, B0>>, Sub1<UInt<UInt<_2, _B2>, B0>>>>::Sliced;

    fn slice(self, from: UInt<UInt<_1, _B1>, B0>, to: UInt<UInt<_2, _B2>, B0>) -> Self::Sliced {
        self.tail.slice(from - B1::new(), to - B1::new())
    }
}

impl<_1: Unsigned, _2: Unsigned, _B1: Bit, _B2: Bit, Head, Tail>
    HSliceable<UInt<UInt<_1, _B1>, B0>, UInt<UInt<_2, _B2>, B1>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<Sub1<UInt<UInt<_1, _B1>, B0>>, UInt<UInt<_2, _B2>, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<_1, _B1>, B0>: Sub<B1>,
    Sub1<UInt<UInt<_1, _B1>, B0>>: Unsigned,
{
    type Sliced =
        <Tail as HSliceable<Sub1<UInt<UInt<_1, _B1>, B0>>, UInt<UInt<_2, _B2>, B0>>>::Sliced;

    fn slice(self, from: UInt<UInt<_1, _B1>, B0>, to: UInt<UInt<_2, _B2>, B1>) -> Self::Sliced {
        self.tail.slice(from - B1::new(), to - B1::new())
    }
}

/*
UInt<UInt<U,B>,B1> to
- UInt<U,B0>
- UInt<UInt<U,B>,B1>
*/

impl<_1: Unsigned, _2: Unsigned, _B1: Bit, _B2: Bit, Head, Tail>
    HSliceable<UInt<UInt<_1, _B1>, B1>, UInt<UInt<_2, _B2>, B0>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UInt<UInt<_1, _B1>, B0>, Sub1<UInt<UInt<_2, _B2>, B0>>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<_2, _B2>, B0>: Sub<B1>,
    Sub1<UInt<UInt<_2, _B2>, B0>>: Unsigned,
{
    type Sliced =
        <Tail as HSliceable<UInt<UInt<_1, _B1>, B0>, Sub1<UInt<UInt<_2, _B2>, B0>>>>::Sliced;

    fn slice(self, from: UInt<UInt<_1, _B1>, B1>, to: UInt<UInt<_2, _B2>, B0>) -> Self::Sliced {
        self.tail.slice(from - B1::new(), to - B1::new())
    }
}

impl<_1: Unsigned, _2: Unsigned, _B1: Bit, _B2: Bit, Head, Tail>
    HSliceable<UInt<UInt<_1, _B1>, B1>, UInt<UInt<_2, _B2>, B1>> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HSliceable<UInt<UInt<_1, _B1>, B0>, UInt<UInt<_2, _B2>, B0>>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Sliced = <Tail as HSliceable<UInt<UInt<_1, _B1>, B0>, UInt<UInt<_2, _B2>, B0>>>::Sliced;

    fn slice(self, from: UInt<UInt<_1, _B1>, B1>, to: UInt<UInt<_2, _B2>, B1>) -> Self::Sliced {
        self.tail.slice(from - B1::new(), to - B1::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::*;

    #[test]
    fn type_length() {
        assert_type_eq!(<Hlist!(U1, U1, U1) as TypeLength>::Length, U3);
    }

    #[test]
    fn slice_hnil() {
        let h = hlist![];
        let s = h.slice(UTerm::new(), UTerm::new());
        assert_eq!(s, hlist![]);
    }

    #[test]
    fn slice_0_to_n() {
        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        // U4 -> U3, U5 -> U4になってる？
        // fix
        let sub = h.slice(U0::new(), U4::new());
        assert_eq!(sub, hlist![1, "hi", U1::new(), U1::new()]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U0::new(), U3::new());
        assert_eq!(sub, hlist![1, "hi", U1::new()]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U0::new(), U2::new());
        assert_eq!(sub, hlist![1, "hi"]);

        let _h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = sub.slice(U0::new(), U1::new());
        assert_eq!(sub, hlist![1]);
    }

    #[test]
    fn slice_1_to_n() {
        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U1::new(), U1::new());
        assert_eq!(sub, hlist![]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U1::new(), U2::new());
        assert_eq!(sub, hlist!["hi"]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U1::new(), U3::new());
        assert_eq!(sub, hlist!["hi", U1::new()]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U1::new(), U4::new());
        assert_eq!(sub, hlist!["hi", U1::new(), U1::new()]);
    }

    #[test]
    fn slice_odd_to_n() {
        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U2::new(), U2::new());
        assert_eq!(sub, hlist![]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U2::new(), U3::new());
        assert_eq!(sub, hlist![U1::new()]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U2::new(), U4::new());
        assert_eq!(sub, hlist![U1::new(), U1::new()]);
    }

    #[test]
    fn slice_even_to_n() {
        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U3::new(), U3::new());
        assert_eq!(sub, hlist![]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U3::new(), U4::new());
        assert_eq!(sub, hlist![U1::new()]);

        let h = hlist![1, "hi", U1::new(), U1::new(), "Foo", 12.345];

        let sub = h.slice(U3::new(), U5::new());
        assert_eq!(sub, hlist![U1::new(), "Foo"]);
    }
}
