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


impl<ElementType, _1, _2> std::ops::Index<[usize; 2]> for BasicMatrix<ElementType, _1, _2>
where
    _1: Unsigned,
    _2: Unsigned,
{
    type Output = ElementType;
    fn index(&self, ind: [usize; 2]) -> &Self::Output {
        <Self as Tensor<_, _, _>>::index(self, hlist!(ind[0]), hlist!(ind[1]))
    }
}

impl<ElementType, _1, _2, _3>
    BinaryOperator<BasicMatrix<ElementType, _1, _2>, BasicMatrix<ElementType, _2, _3>>
    for Multiplication
where
    ElementType: std::ops::Mul<Output = ElementType> + std::ops::Add<Output = ElementType> + Copy,
    ElementType: UnitalRing<Addition, Multiplication>,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
    _1: Unsigned,
    _2: Unsigned,
    _3: Unsigned,
{
    type Output = BasicMatrix<ElementType, _1, _3>;
    #[inline(always)]
    fn operate(
        lhs: BasicMatrix<ElementType, _1, _2>,
        rhs: BasicMatrix<ElementType, _2, _3>,
    ) -> BasicMatrix<ElementType, _1, _3> {
        <Multiplication as BinaryOperator<
            &BasicMatrix<ElementType, _1, _2>,
            &BasicMatrix<ElementType, _2, _3>,
        >>::operate(&lhs, &rhs)
    }
}

impl<'a, ElementType, _1, _2, _3>
    BinaryOperator<&'a BasicMatrix<ElementType, _1, _2>, BasicMatrix<ElementType, _2, _3>>
    for Multiplication
where
    ElementType: std::ops::Mul<Output = ElementType> + std::ops::Add<Output = ElementType> + Copy,
    ElementType: UnitalRing<Addition, Multiplication>,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
    _1: Unsigned,
    _2: Unsigned,
    _3: Unsigned,
{
    type Output = BasicMatrix<ElementType, _1, _3>;
    #[inline(always)]
    fn operate(
        lhs: &'a BasicMatrix<ElementType, _1, _2>,
        rhs: BasicMatrix<ElementType, _2, _3>,
    ) -> BasicMatrix<ElementType, _1, _3> {
        <Multiplication as BinaryOperator<
            &BasicMatrix<ElementType, _1, _2>,
            &BasicMatrix<ElementType, _2, _3>,
        >>::operate(lhs, &rhs)
    }
}

impl<'a, ElementType, _1, _2, _3>
    BinaryOperator<BasicMatrix<ElementType, _1, _2>, &'a BasicMatrix<ElementType, _2, _3>>
    for Multiplication
where
    ElementType: std::ops::Mul<Output = ElementType> + std::ops::Add<Output = ElementType> + Copy,
    ElementType: UnitalRing<Addition, Multiplication>,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
    _1: Unsigned,
    _2: Unsigned,
    _3: Unsigned,
{
    type Output = BasicMatrix<ElementType, _1, _3>;
    #[inline(always)]
    fn operate(
        lhs: BasicMatrix<ElementType, _1, _2>,
        rhs: &'a BasicMatrix<ElementType, _2, _3>,
    ) -> BasicMatrix<ElementType, _1, _3> {
        <Multiplication as BinaryOperator<
            &BasicMatrix<ElementType, _1, _2>,
            &BasicMatrix<ElementType, _2, _3>,
        >>::operate(&lhs, rhs)
    }
}

impl<'a, ElementType, _1, _2, _3>
    BinaryOperator<&'a BasicMatrix<ElementType, _1, _2>, &'a BasicMatrix<ElementType, _2, _3>>
    for Multiplication
where
    ElementType: std::ops::Mul<Output = ElementType> + std::ops::Add<Output = ElementType> + Copy,
    ElementType: UnitalRing<Addition, Multiplication>,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
    _1: Unsigned,
    _2: Unsigned,
    _3: Unsigned,
{
    type Output = BasicMatrix<ElementType, _1, _3>;
    #[inline(always)]
    fn operate(
        lhs: &'a BasicMatrix<ElementType, _1, _2>,
        rhs: &'a BasicMatrix<ElementType, _2, _3>,
    ) -> BasicMatrix<ElementType, _1, _3> {
        let mut new_mat = BasicMatrix::<ElementType, _1, _3>::zeros();

        for i in 0.._1::to_usize() {
            for j in 0.._3::to_usize() {
                for k in 0.._2::to_usize() {
                    *new_mat.index_mut(hlist!(i), hlist!(j)) = *new_mat.index(hlist!(i), hlist!(j))
                        + *lhs.index(hlist!(i), hlist!(k)) * *rhs.index(hlist!(k), hlist!(j));
                }
            }
        }

        new_mat
    }
}
