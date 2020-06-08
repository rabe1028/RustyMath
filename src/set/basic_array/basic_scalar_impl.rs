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

impl<ElementType> BinaryOperator<BasicScalar<ElementType>, BasicScalar<ElementType>>
    for Multiplication
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
    type Output = BasicScalar<ElementType>;
    #[inline(always)]
    fn operate(lhs: BasicScalar<ElementType>, rhs: BasicScalar<ElementType>) -> Self::Output {
        <HadamardProduct as BinaryOperator<_, _>>::operate(lhs, rhs)
    }
}

impl<'a, ElementType> BinaryOperator<&'a BasicScalar<ElementType>, BasicScalar<ElementType>>
    for Multiplication
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
    type Output = BasicScalar<ElementType>;
    #[inline(always)]
    fn operate(lhs: &'a BasicScalar<ElementType>, rhs: BasicScalar<ElementType>) -> Self::Output {
        <HadamardProduct as BinaryOperator<_, _>>::operate(lhs, rhs)
    }
}

impl<'a, ElementType> BinaryOperator<BasicScalar<ElementType>, &'a BasicScalar<ElementType>>
    for Multiplication
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
    type Output = BasicScalar<ElementType>;
    #[inline(always)]
    fn operate(lhs: BasicScalar<ElementType>, rhs: &'a BasicScalar<ElementType>) -> Self::Output {
        <HadamardProduct as BinaryOperator<_, _>>::operate(lhs, rhs)
    }
}

impl<'a, ElementType> BinaryOperator<&'a BasicScalar<ElementType>, &'a BasicScalar<ElementType>>
    for Multiplication
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
    type Output = BasicScalar<ElementType>;
    #[inline(always)]
    fn operate(
        lhs: &'a BasicScalar<ElementType>,
        rhs: &'a BasicScalar<ElementType>,
    ) -> Self::Output {
        <HadamardProduct as BinaryOperator<_, _>>::operate(lhs, rhs)
    }
}

impl<ElementType> InternalBinaryOperator<BasicScalar<ElementType>> for Multiplication
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
}

impl<ElementType> Totality<Multiplication> for BasicScalar<ElementType>
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
}

impl<ElementType> Associativity<Multiplication, Self, Self> for BasicScalar<ElementType>
where
    ElementType: Scalar<ElementType> + Copy + PartialEq,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
}
