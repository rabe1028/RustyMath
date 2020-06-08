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

impl<'a, ElementType, Contravariant, Covariant>
    From<BasicArray<ElementType, Contravariant, Covariant>>
    for Cow<'a, BasicArray<ElementType, Contravariant, Covariant>>
where
    Contravariant: HList + IndexShape + std::clone::Clone,
    Covariant: HList + IndexShape + std::clone::Clone,
    ElementType: std::clone::Clone,
{
    fn from(
        t: BasicArray<ElementType, Contravariant, Covariant>,
    ) -> Cow<'a, BasicArray<ElementType, Contravariant, Covariant>> {
        Cow::Owned(t)
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    From<&'a BasicArray<ElementType, Contravariant, Covariant>>
    for Cow<'a, BasicArray<ElementType, Contravariant, Covariant>>
where
    Contravariant: HList + IndexShape + std::clone::Clone,
    Covariant: HList + IndexShape + std::clone::Clone,
    ElementType: std::clone::Clone,
{
    fn from(
        t: &'a BasicArray<ElementType, Contravariant, Covariant>,
    ) -> Cow<'a, BasicArray<ElementType, Contravariant, Covariant>> {
        Cow::Borrowed(t)
    }
}

impl<ElementType, Contravariant, Covariant, I, J> std::ops::Index<(I, J)>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: IndexShape + Add<Covariant>,
    Covariant: IndexShape,
    Join<Contravariant, Covariant>: IndexShape,
    <Contravariant as IndexShape>::Shape: Add<
        <Covariant as IndexShape>::Shape,
        Output = <Join<Contravariant, Covariant> as IndexShape>::Shape,
    >,
    I: Into<<Contravariant as IndexShape>::Shape>,
    J: Into<<Covariant as IndexShape>::Shape>,
{
    type Output = ElementType;
    fn index(&self, (cont, cov): (I, J)) -> &Self::Output {
        let cont = cont.into();
        let cov = cov.into();
        let (offset, _) = <Self as Tensor<_, _, _>>::Joined::get_index(cont + cov);
        &self._inner[offset]
    }
}

impl<ElementType, Contravariant, Covariant, I, J> std::ops::IndexMut<(I, J)>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: IndexShape + Add<Covariant>,
    Covariant: IndexShape,
    Join<Contravariant, Covariant>: IndexShape,
    <Contravariant as IndexShape>::Shape: Add<
        <Covariant as IndexShape>::Shape,
        Output = <Join<Contravariant, Covariant> as IndexShape>::Shape,
    >,
    I: Into<<Contravariant as IndexShape>::Shape>,
    J: Into<<Covariant as IndexShape>::Shape>,
{
    fn index_mut(&mut self, (cont, cov): (I, J)) -> &mut Self::Output {
        let cont = cont.into();
        let cov = cov.into();
        let (offset, _) = <Self as Tensor<_, _, _>>::Joined::get_index(cont + cov);
        &mut self._inner[offset]
    }
}

// Addition(Elementwise)

impl<ElementType, Contravariant, Covariant>
    BinaryOperator<
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    > for Addition
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: BasicArray<ElementType, Contravariant, Covariant>,
        rhs: BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        <Addition as BinaryOperator<
            &BasicArray<ElementType, Contravariant, Covariant>,
            &BasicArray<ElementType, Contravariant, Covariant>,
        >>::operate(&lhs, &rhs)
    }
}

// forward_binop! macro cannot use generics, so we impl all.
impl<'a, ElementType, Contravariant, Covariant>
    BinaryOperator<
        &'a BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    > for Addition
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
        rhs: BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        <Addition as BinaryOperator<
            &BasicArray<ElementType, Contravariant, Covariant>,
            &BasicArray<ElementType, Contravariant, Covariant>,
        >>::operate(lhs, &rhs)
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    BinaryOperator<
        BasicArray<ElementType, Contravariant, Covariant>,
        &'a BasicArray<ElementType, Contravariant, Covariant>,
    > for Addition
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: BasicArray<ElementType, Contravariant, Covariant>,
        rhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        <Addition as BinaryOperator<
            &BasicArray<ElementType, Contravariant, Covariant>,
            &BasicArray<ElementType, Contravariant, Covariant>,
        >>::operate(&lhs, rhs)
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    BinaryOperator<
        &'a BasicArray<ElementType, Contravariant, Covariant>,
        &'a BasicArray<ElementType, Contravariant, Covariant>,
    > for Addition
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    // &'a ElementType: std::ops::Add<Output = ElementType>,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
        rhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        assert_eq!(lhs._inner.len(), rhs._inner.len());
        let new_vec = lhs
            ._inner
            .iter()
            .zip(rhs._inner.iter())
            .map(|(l, r)| *l + *r)
            .collect();

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
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
}

impl<ElementType, Contravariant, Covariant> Totality<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
}

impl<ElementType, Contravariant, Covariant> Associativity<Addition, Self, Self>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + PartialEq + Copy,
    Contravariant: HList + IndexShape + PartialEq + Add<Covariant> + Clone,
    Covariant: HList + IndexShape + PartialEq + Clone,
{
}

impl<ElementType, Contravariant, Covariant> Identity<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    Self: Tensor<ElementType, Contravariant, Covariant>,
    ElementType: UnitalRing<Addition, Multiplication>
        + std::ops::Add<Output = ElementType>
        + PartialEq
        + Copy,
    Contravariant: HList + IndexShape + PartialEq + Add<Covariant> + Clone,
    Covariant: HList + IndexShape + PartialEq + Clone,
    Addition: InternalBinaryOperator<ElementType>,
    Multiplication: InternalBinaryOperator<ElementType>,
{
    #[inline(always)]
    fn identity() -> Self {
        let cap = Contravariant::get_capacity() * Covariant::get_capacity();
        Self::from_vec(vec![ElementType::zero(); cap])
    }
}

// Hadamard Product(Elementwise)


impl<ElementType, Contravariant, Covariant>
    BinaryOperator<
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    > for HadamardProduct
where
    ElementType: Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: BasicArray<ElementType, Contravariant, Covariant>,
        rhs: BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        <Self as BinaryOperator<
            &BasicArray<ElementType, Contravariant, Covariant>,
            &BasicArray<ElementType, Contravariant, Covariant>,
        >>::operate(&lhs, &rhs)
    }
}

// forward_binop! macro cannot use generics, so we impl all.
impl<'a, ElementType, Contravariant, Covariant>
    BinaryOperator<
        &'a BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    > for HadamardProduct
where
    ElementType: Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
        rhs: BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        <Self as BinaryOperator<
            &BasicArray<ElementType, Contravariant, Covariant>,
            &BasicArray<ElementType, Contravariant, Covariant>,
        >>::operate(lhs, &rhs)
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    BinaryOperator<
        BasicArray<ElementType, Contravariant, Covariant>,
        &'a BasicArray<ElementType, Contravariant, Covariant>,
    > for HadamardProduct
where
    ElementType: Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: BasicArray<ElementType, Contravariant, Covariant>,
        rhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        <Self as BinaryOperator<
            &BasicArray<ElementType, Contravariant, Covariant>,
            &BasicArray<ElementType, Contravariant, Covariant>,
        >>::operate(&lhs, rhs)
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    BinaryOperator<
        &'a BasicArray<ElementType, Contravariant, Covariant>,
        &'a BasicArray<ElementType, Contravariant, Covariant>,
    > for HadamardProduct
where
    ElementType: Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Output = BasicArray<ElementType, Contravariant, Covariant>;
    fn operate(
        lhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
        rhs: &'a BasicArray<ElementType, Contravariant, Covariant>,
    ) -> BasicArray<ElementType, Contravariant, Covariant> {
        assert_eq!(lhs._inner.len(), rhs._inner.len());
        let new_vec = lhs
            ._inner
            .iter()
            .zip(rhs._inner.iter())
            .map(|(l, r)| Multiplication::operate(*l, *r))
            .collect();

        BasicArray {
            _inner: new_vec,
            _contravariant: PhantomData,
            _covariant: PhantomData,
        }
    }
}

impl<ElementType, Contravariant, Covariant>
    InternalBinaryOperator<BasicArray<ElementType, Contravariant, Covariant>> for HadamardProduct
where
    ElementType: Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
}

impl<ElementType, Contravariant, Covariant> Totality<HadamardProduct>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
}

impl<ElementType, Contravariant, Covariant> Associativity<HadamardProduct, Self, Self>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: PartialEq + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Contravariant: HList + IndexShape + PartialEq + Add<Covariant> + Clone,
    Covariant: HList + IndexShape + PartialEq + Clone,
{
}