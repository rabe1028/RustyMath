
use crate::operator::*;
use crate::property::*;

use std::marker::PhantomData;
use std::ops::Add;

use frunk::hlist::HList;
use frunk::*; //{HCons, HNil};
use typenum::uint::Unsigned;
use typenum::*;

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

trait TypeLen {
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

trait IndexShape {
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

trait Tensor<ElementType, Shape>
where
    Shape: HList + IndexShape,
{
    // index
    // a[[1, 2, 3]] = a.index([1, 2, 3])
    fn index<I: Into<<Shape as IndexShape>::Shape>>(&self, index: I) -> &ElementType;
}

trait Scalar<ElementType>: Tensor<ElementType, HNil> {}
trait Vector<ElementType, _1>: Tensor<ElementType, Hlist!(_1)>
where
    _1: Unsigned,
{
}
trait Matrix<ElementType, _1, _2>: Tensor<ElementType, Hlist!(_1, _2)>
where
    _1: Unsigned,
    _2: Unsigned,
{
}




/*
Test Array Struct Implementation
*/

struct BasicArray<ElementType, Shape> {
    _inner: Vec<ElementType>,
    _phantom: PhantomData<Shape>,
}

impl<ElementType,Shape> BasicArray<ElementType, Shape> 
where Shape: HList + IndexShape
{
    pub fn from_vec(vec: Vec<ElementType>) -> Self {
        assert!(vec.len() == Shape::get_capacity());
        BasicArray { _inner: vec, _phantom: PhantomData}
    }
    /*
    pub fn zeros<I: Into<Shape>>(_: I) -> Self {
        // Addition Zero Element
        BasicArray { 
            _inner: vec![0; Shape::get_capacity()],
            _phantom: PhantomData,
        }
    }
    */
}

impl<ElementType, Shape> Tensor<ElementType, Shape> for BasicArray<ElementType, Shape>
where
    Shape: HList + IndexShape,
{
    fn index<I: Into<<Shape as IndexShape>::Shape>>(&self, index: I) -> &ElementType {
        let (offset, _) = Shape::get_index(index.into());
        &self._inner[offset]
    }
}

impl<ElementType> Scalar<ElementType> for BasicArray<ElementType, HNil> {}

impl<ElementType, _1> Vector<ElementType, _1> for BasicArray<ElementType, Hlist!(_1)> where
    _1: Unsigned
{
}

impl<ElementType, _1, _2> Matrix<ElementType, _1, _2> for BasicArray<ElementType, Hlist!(_1, _2)>
where
    _1: Unsigned,
    _2: Unsigned,
{
}

type BasicScalar<ElementType> = BasicArray<ElementType, HNil>;

type BasicVector<ElementType, _1> = BasicArray<ElementType, Hlist!(_1)>;

type BasicMatrix<ElementType, _1, _2> = BasicArray<ElementType, Hlist!(_1, _2)>;

impl<ElementType, Shape>
    BinaryOperator<
        BasicArray<ElementType, Shape>,
        BasicArray<ElementType, Shape>,
        BasicArray<ElementType, Shape>,
    > for Addition
where
    ElementType: std::ops::Add,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
    Shape: HList,
{
    #[inline(always)]
    fn operate(
        lhs: BasicArray<ElementType, Shape>,
        rhs: BasicArray<ElementType, Shape>,
    ) -> BasicArray<ElementType, Shape> {
        let new_vec: Vec<ElementType> = lhs
            ._inner
            .into_iter()
            .zip(rhs._inner.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        BasicArray {
            _inner: new_vec,
            _phantom: PhantomData,
        }
    }
}
