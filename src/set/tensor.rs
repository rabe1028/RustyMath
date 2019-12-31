use frunk::hlist::HList;
use frunk::*; //{HCons, HNil};
use typenum::uint::Unsigned;

use crate::util::IndexShape;

pub trait Tensor<ElementType, Shape>
where
    Shape: HList + IndexShape,
{
    // index
    // a[[1, 2, 3]] = a.index([1, 2, 3])
    fn index<I: Into<<Shape as IndexShape>::Shape>>(&self, index: I) -> &ElementType;
}

pub trait Scalar<ElementType>: Tensor<ElementType, HNil> {
    fn new(elem: ElementType) -> Self;

    fn get(&self) -> &ElementType;
}

//impl<ElementType, T> Scalar<ElementType> for T where T: Tensor<ElementType, HNil> {}

pub trait Vector<ElementType, _1>: Tensor<ElementType, Hlist!(_1)>
where
    _1: Unsigned,
{
}

//impl<ElementType, _1, T> Vector<ElementType, _1> for T where T: Tensor<ElementType, Hlist!(_1)> {}

pub trait Matrix<ElementType, _1, _2>: Tensor<ElementType, Hlist!(_1, _2)>
where
    _1: Unsigned,
    _2: Unsigned,
{
}

/*
impl<ElementType, _1, _2, T> Matrix<ElementType, _1, _2> for T where
    T: Tensor<ElementType, Hlist!(_1, _2)>
{
}
*/
