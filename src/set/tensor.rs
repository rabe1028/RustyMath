use std::ops::Add;

use frunk::hlist::HList;
use frunk::*; //{HCons, HNil};
use typenum::uint::Unsigned;
use typenum::*;

/*
trait Size {}
trait StaticSize: Size {}

macro_rules! impl_static {
    ($($ty: ty),*) => {
        $(
            impl Size for $ty {}
        )*
    }
}

impl_static! {
    U1,U2,U3,U4,U5
}
*/

pub trait TypeLen {
    type Length: Unsigned; // HList length (type int)

    fn elem_count() -> usize;
}

impl TypeLen for HNil {
    type Length = U0;

    fn elem_count() -> usize {
        0
    }
}

impl<Head, Tail> TypeLen for HCons<Head, Tail>
where
    Head: Unsigned,
    Tail: HList + TypeLen,
    <Tail as TypeLen>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLen>::Length as Add<U1>>::Output: Unsigned,
{
    type Length = <<Tail as TypeLen>::Length as Add<U1>>::Output;

    fn elem_count() -> usize {
        <Head as Unsigned>::to_usize() + <Tail as TypeLen>::elem_count()
    }
}

pub trait IndexShape {
    // all usize hlist
    type Shape: HList;

    fn get_index(index: Self::Shape) -> (usize, usize);
    fn get_capacity() -> usize;
}

impl IndexShape for HNil {
    type Shape = HNil;

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

pub trait Tensor<ElementType, Shape>
where
    Shape: HList + IndexShape,
{
    // index
    // a[[1, 2, 3]] = a.index([1, 2, 3])
    fn index<I: Into<<Shape as IndexShape>::Shape>>(&self, index: I) -> &ElementType;
}

pub trait Scalar<ElementType>: Tensor<ElementType, HNil> {}
pub trait Vector<ElementType, _1>: Tensor<ElementType, Hlist!(_1)>
where
    _1: Unsigned,
{
}

pub trait Matrix<ElementType, _1, _2>: Tensor<ElementType, Hlist!(_1, _2)>
where
    _1: Unsigned,
    _2: Unsigned,
{
}
