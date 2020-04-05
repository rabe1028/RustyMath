use crate::operator::*;
use crate::property::*;
use crate::set::*;
use crate::util::*;

use std::marker::PhantomData;

use std::borrow::Cow;

use frunk::hlist::HList;
use frunk::*; //{HCons, HNil};
use typenum::uint::Unsigned;
use typenum::*;

use std::ops::Add;

/*
Test Array Struct Implementation
*/

#[derive(Debug, Clone, PartialEq)]
struct BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    _inner: Vec<ElementType>,
    _contravariant: PhantomData<Contravariant>,
    _covariant: PhantomData<Covariant>,
}

impl<ElementType, Contravariant, Covariant> BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    // pub fn from_vec(vec: Vec<ElementType>) -> Self {
    //     assert!(vec.len() == Contravariant::get_capacity() * Covariant::get_capacity());
    //     BasicArray {
    //         _inner: vec,
    //         _contravariant: PhantomData,
    //         _covariant: PhantomData,
    //     }
    // }

    // pub fn zeros<I: Into<Shape>>(_: I) -> Self {
    //     // Addition Zero Element
    //     BasicArray {
    //         _inner: vec![0; Shape::get_capacity()],
    //         _phantom: PhantomData,
    //     }
    // }
}

impl<ElementType, Contravariant, Covariant> Tensor<ElementType, Contravariant, Covariant>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
    Join<Contravariant, Covariant>: IndexShape,
    <Contravariant as IndexShape>::Shape: Add<
        <Covariant as IndexShape>::Shape,
        Output = <Join<Contravariant, Covariant> as IndexShape>::Shape,
    >,
{
    type Joined = Join<Contravariant, Covariant>;
    fn index<
        I: Into<<Contravariant as IndexShape>::Shape>,
        J: Into<<Covariant as IndexShape>::Shape>,
    >(
        &self,
        cont: I,
        cov: J,
    ) -> &ElementType {
        let cont = cont.into();
        let cov = cov.into();
        let (offset, _) = Self::Joined::get_index(cont + cov);
        &self._inner[offset]
    }

    fn from_vec(vec: Vec<ElementType>) -> Self {
        assert!(vec.len() == Contravariant::get_capacity() * Covariant::get_capacity());
        BasicArray {
            _inner: vec,
            _contravariant: PhantomData,
            _covariant: PhantomData,
        }
    }
}

impl<ElementType> Scalar<ElementType> for BasicArray<ElementType, HNil, HNil> {
    fn new(elem: ElementType) -> Self {
        BasicArray::from_vec(vec![elem])
    }

    fn get(&self) -> &ElementType {
        self.index(HNil, HNil)
    }
}

impl<ElementType, _1> Vector<ElementType, _1> for BasicArray<ElementType, Hlist!(_1), HNil> where
    _1: Unsigned
{
}

impl<ElementType, _1> Covector<ElementType, _1> for BasicArray<ElementType, HNil, Hlist!(_1)> where
    _1: Unsigned
{
}

impl<ElementType, _1, _2> Matrix<ElementType, _1, _2>
    for BasicArray<ElementType, Hlist!(_1), Hlist!(_2)>
where
    _1: Unsigned,
    _2: Unsigned,
{
}

type BasicScalar<ElementType> = BasicArray<ElementType, HNil, HNil>;

type BasicVector<ElementType, _1> = BasicArray<ElementType, Hlist!(_1), HNil>;

type BasicMatrix<ElementType, _1, _2> = BasicArray<ElementType, Hlist!(_1), Hlist!(_2)>;

impl<ElementType, Contravariant, Covariant>
    BinaryOperator<
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    > for Addition
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
    #[inline(always)]
    fn operate<'a, 'b>(
        lhs: impl Into<Cow<'a, BasicArray<ElementType, Contravariant, Covariant>>>,
        rhs: impl Into<Cow<'b, BasicArray<ElementType, Contravariant, Covariant>>>,
    ) -> BasicArray<ElementType, Contravariant, Covariant>
    where
        BasicArray<ElementType, Contravariant, Covariant>: 'a + 'b,
    {
        let lhs = lhs.into();
        let rhs = rhs.into();

        let mut new_vec: Vec<ElementType> = vec![];
        for i in 0..lhs._inner.len() {
            new_vec.push(lhs._inner[i] + rhs._inner[i]);
        }
        BasicArray {
            _inner: new_vec,
            _contravariant: PhantomData,
            _covariant: PhantomData,
        }
    }
}

impl<ElementType, Contravariant, Covariant>
    InternalBinaryOperator<BasicArray<ElementType, Contravariant, Covariant>> for Addition
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
}

impl<ElementType, Contravariant, Covariant> Totality<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
}

impl<ElementType, Contravariant, Covariant> Associativity<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy + PartialEq,
    Contravariant: HList + IndexShape + PartialEq + Add<Covariant>,
    Covariant: HList + IndexShape + PartialEq,
    std::vec::Vec<ElementType>: std::iter::FromIterator<<ElementType as std::ops::Add>::Output>,
{
}

// for Lazy Evaluation

struct LazyBinaryOperation<Op, Left, Right> {
    lhs: Left,
    rhs: Right,
    _op: PhantomData<Op>,
}

impl<Op, Left, Right> LazyBinaryOperation<Op, Left, Right> {
    fn eval(self) {
        // TODO: implement
    }
}

impl<Rhs, ElementType, Contravariant, Covariant> Add<Rhs> for BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = LazyBinaryOperation<Addition, Self, Rhs>;
    fn add(self, other: Rhs) -> Self::Output {
        LazyBinaryOperation {
            lhs: self,
            rhs: other,
            _op: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::set::basic_array::*;

    #[test]
    fn construct_0d() {
        let _a: BasicArray<isize, HNil, HNil> = BasicArray::from_vec(vec![1]);
        let _b: BasicScalar<isize> = BasicArray::from_vec(vec![1]);

        assert_eq!(_a, _b);
    }

    #[test]
    #[should_panic]
    fn construct_0d_should_panic() {
        let _a: BasicArray<isize, Hlist!(U5), HNil> = BasicArray::from_vec(vec![1]);
    }

    #[test]
    fn test_index_0d() {
        let a: BasicScalar<isize> = BasicScalar::new(3);
        assert_eq!(*a.get(), 3);
    }

    #[test]
    fn construct_1d() {
        let _a: BasicArray<isize, Hlist!(U4), HNil> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let _b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(_a, _b);
    }

    #[test]
    #[should_panic]
    fn construct_1d_should_panic() {
        let _a: BasicArray<isize, Hlist!(U3), HNil> = BasicArray::from_vec(vec![1, 2, 3, 4]);
    }
}
