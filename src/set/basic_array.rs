use crate::operator::*;
use crate::property::*;
use crate::set::*;
use crate::util::IndexShape;

use std::marker::PhantomData;

use std::borrow::Cow;

use frunk::hlist::HList;
use frunk::*; //{HCons, HNil};
use typenum::uint::Unsigned;
use typenum::*;

/*
Test Array Struct Implementation
*/

#[derive(Debug, Clone, PartialEq)]
struct BasicArray<ElementType, Shape> {
    _inner: Vec<ElementType>,
    _phantom: PhantomData<Shape>,
}

impl<ElementType, Shape> BasicArray<ElementType, Shape>
where
    Shape: HList + IndexShape,
{
    pub fn from_vec(vec: Vec<ElementType>) -> Self {
        assert!(vec.len() == Shape::get_capacity());
        BasicArray {
            _inner: vec,
            _phantom: PhantomData,
        }
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

impl<ElementType> Scalar<ElementType> for BasicArray<ElementType, HNil> {
    fn new(elem: ElementType) -> Self {
        BasicArray::from_vec(vec![elem])
    }

    fn get(&self) -> &ElementType {
        self.index(HNil)
    }
}

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
    BasicArray<ElementType, Shape>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Shape: HList,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
    #[inline(always)]
    fn operate<'a, 'b>(
        lhs: impl Into<Cow<'a, BasicArray<ElementType, Shape>>>,
        rhs: impl Into<Cow<'b, BasicArray<ElementType, Shape>>>,
    ) -> BasicArray<ElementType, Shape>
    where
        BasicArray<ElementType, Shape>: 'a + 'b,
    {
        let lhs = lhs.into();
        let rhs = rhs.into();

        let mut new_vec: Vec<ElementType> = vec![];
        for i in 0..lhs._inner.len() {
            new_vec.push(lhs._inner[i] + rhs._inner[i]);
        }
        BasicArray {
            _inner: new_vec,
            _phantom: PhantomData,
        }
    }
}

impl<ElementType, Shape> InternalBinaryOperator<BasicArray<ElementType, Shape>> for Addition
where
    BasicArray<ElementType, Shape>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Shape: HList,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
}

impl<ElementType, Shape> Totality<Addition> for BasicArray<ElementType, Shape>
where
    BasicArray<ElementType, Shape>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Shape: HList,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
}

impl<ElementType, Shape> Associativity<Addition> for BasicArray<ElementType, Shape>
where
    BasicArray<ElementType, Shape>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy + PartialEq,
    Shape: HList + PartialEq,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
}

#[cfg(test)]
mod tests {
    use crate::set::basic_array::*;

    #[test]
    fn construct_0d() {
        let _a: BasicArray<isize, HNil> = BasicArray::from_vec(vec![1]);
        let _b: BasicScalar<isize> = BasicArray::from_vec(vec![1]);

        assert_eq!(_a, _b);
    }

    #[test]
    #[should_panic]
    fn construct_0d_should_panic() {
        let _a: BasicArray<isize, Hlist!(U5)> = BasicArray::from_vec(vec![1]);
    }

    #[test]
    fn test_index_0d() {
        let a: BasicScalar<isize> = BasicScalar::new(3);
        assert_eq!(*a.get(), 3);
    }

    #[test]
    fn construct_1d() {
        let _a: BasicArray<isize, Hlist!(U4)> = BasicArray::from_vec(vec![1, 2, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn construct_1d_should_panic() {
        let _a: BasicArray<isize, Hlist!(U3)> = BasicArray::from_vec(vec![1, 2, 3, 4]);
    }
}
