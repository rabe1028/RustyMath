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

impl<ElementType, Contravariant, Covariant> BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
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

    fn index_mut<
        I: Into<<Contravariant as IndexShape>::Shape>,
        J: Into<<Covariant as IndexShape>::Shape>,
    >(
        &mut self,
        cont: I,
        cov: J,
    ) -> &mut ElementType {
        let cont = cont.into();
        let cov = cov.into();
        let (offset, _) = Self::Joined::get_index(cont + cov);
        &mut self._inner[offset]
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
        <Self as Tensor<_, _, _>>::index(self, cont, cov)
    }
}

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

// forward_internal_binop! {
//     Addition,
//     BasicArray<ElementType, Contravariant, Contravariant>
//     where
//         ElementType: Add + Copy,
//         Contravariant: HList + IndexShape,
//         Covariant: HList + IndexShape,
//     (lhs, rhs) => {
//         assert_eq!(lhs._inner.len(), rhs._inner.len());
//         let new_vec = lhs
//             ._inner
//             .iter()
//             .zip(rhs._inner.iter())
//             .map(|(l, r)| *l + *r)
//             .collect();

//         BasicArray {
//             _inner: new_vec,
//             _contravariant: PhantomData,
//             _covariant: PhantomData,
//         }
//     }
// }

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
        // assert_eq!(lhs._inner.len(), rhs._inner.len());
        // let new_vec = lhs
        //     ._inner
        //     .iter()
        //     .zip(rhs._inner.iter())
        //     .map(|(l, r)| *l + *r)
        //     .collect();

        // BasicArray {
        //     _inner: new_vec,
        //     _contravariant: PhantomData,
        //     _covariant: PhantomData,
        // }

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

impl<ElementType, Contravariant, Covariant> Associativity<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + PartialEq + Copy,
    Contravariant: HList + IndexShape + PartialEq + Add<Covariant> + Clone,
    Covariant: HList + IndexShape + PartialEq + Clone,
{
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

// impl<ElementType, _1, _2> Totality<Multiplication> for BasicMatrix<ElementType, _1, _2>
// where
//     BasicMatrix<ElementType, _1, _2>: Clone,
//     ElementType: std::ops::Mul<Output = ElementType> + Copy,
//     _1: Unsigned,
//     _2: Unsigned,
// {
// }

// impl<ElementType, _1, _2> Associativity<Multiplication> for BasicMatrix<ElementType, _1, _2>
// where
//     BasicMatrix<ElementType, _1, _2>: Clone,
//     ElementType: std::ops::Mul<Output = ElementType> + Copy,
//     _1: Unsigned,
//     _2: Unsigned,
// {
// }

// for Lazy Evaluation
// Addの戻り値にライフタイムがあると無理っぽい

// 20200409
// 関連型にlifetimeのgenericsが使えないため，
// LazyBinaryOperationを4タイプに分けて実装する
// 1 - Owned & Owned
// 2 - Borrowed & Owned
// 3 - Owned & Borrowed
// 4 - Borrowed & Borrowed
// このようにしないと，BinaryOperatorのGenericsに入れる型引数が悪さをする
// Owned&Ownedのときに，lifetime parameterがimplの文中にないから，怒られる

// 20200505
// LazyOperationの設計方針
//
// LazyBinaryOperationXXにLazyBinaryOperationXXが入るときに，Opの制約を満たせない
// 対応法
// 1. BasicArrayにもLazyOperationを実装し，LazyOperation::eval()経由で評価後の値を取れるようにする
//  -- Opには常にLazyOperation:::Outputの値が入る
//  -- 問題が発覚 : &'a BasicArray::Outputの型でBinaryOperationを組むと，参照が入ってしまう
//        (ex: )
// 2. Left, RightにLazyOperationが入った時の実装を行う
//  -- つらみ
// 1.を選択

// 20200508
// 設計方針の変更
// Inputを正規化(Cowに変換)する処理は，InputSanitizerに移譲する
// InputSanitizerは，LazyOperationXXにも付与する
// これにより，LazyBinaryOperation一つで管理可能
// (他を使うと，借用先で借用することになり，失敗する)

pub trait LazyOperation {
    type Output;
    fn eval(self) -> Self::Output;
}

pub trait InputSanitizer {
    type InputShape;
    fn sanitize(self) -> Self::InputShape;
}

impl<ElementType, Contravariant, Covariant> InputSanitizer
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType>,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
    type InputShape = Self;
    fn sanitize(self) -> Self::InputShape {
        self
    }
}

type Sanitize<A> = <A as InputSanitizer>::InputShape;

impl<ElementType, Contravariant, Covariant> InputSanitizer
    for &'_ BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType>,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
    type InputShape = Self;
    fn sanitize(self) -> Self::InputShape {
        self
    }
}

impl<Op, Left, Right> InputSanitizer for LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    type InputShape = <Op as BinaryOperator<Sanitize<Left>, Sanitize<Right>>>::Output;
    fn sanitize(self) -> Self::InputShape {
        self.eval()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    lhs: Left,
    rhs: Right,
    _op: PhantomData<Op>,
}

// for no blas normal impls
impl<'a, Op, Left, Right> LazyOperation for LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    type Output = <Op as BinaryOperator<Sanitize<Left>, Sanitize<Right>>>::Output;
    fn eval(self) -> Self::Output {
        Op::operate(self.lhs.sanitize(), self.rhs.sanitize())
    }
}

// Cannot use this impl because of orphan rules
// impl<LazyOp, ElementType, Contravariant, Covariant>
//     Add<BasicArray<ElementType, Contravariant, Covariant>> for LazyOp
// where
//     BasicArray<ElementType, Contravariant, Covariant>: Clone,
//     ElementType: std::ops::Add<Output = ElementType> + Copy,
//     Contravariant: HList + IndexShape + Add<Covariant>,
//     Covariant: HList + IndexShape,
//     LazyOp: LazyOperation<Output = BasicArray<ElementType, Contravariant, Covariant>>,
// {
//     // OutputはこのLazyOperation計算後の型である必要がある
//     // LazyBinaryOperation::OutputとRhsの加算
//     type Output = LazyBinaryOperation<
//         Addition,
//         Self,
//         BasicArray<ElementType, Contravariant, Covariant>,
//         BasicArray<ElementType, Contravariant, Covariant>,
//     >;
// }

/*\
|*| Tensor Add Impl
\*/

impl<ElementType, Contravariant, Covariant, Rhs> Add<Rhs>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
    Self: InputSanitizer, //Left
    Rhs: InputSanitizer, //Right
    Addition: BinaryOperator<Sanitize<Self>, Sanitize<Rhs>>,
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

impl<ElementType, Contravariant, Covariant, Rhs> Add<Rhs>
    for &'_ BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: std::clone::Clone + HList + IndexShape + Add<Covariant>,
    Covariant: std::clone::Clone + HList + IndexShape,
    Self: InputSanitizer, //Left
    Rhs: InputSanitizer,  //Right
    Addition: std::clone::Clone + BinaryOperator<Sanitize<Self>, Sanitize<Rhs>>,
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

impl<Op, Left, Right, Rhs> Add<Rhs> for LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
    Self: InputSanitizer, //Left
    Rhs: std::clone::Clone + InputSanitizer,  //Right
    Addition: std::clone::Clone + BinaryOperator<Sanitize<Self>, Sanitize<Rhs>>,
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

    #[test]
    fn vec_index_test() {
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        //<Hlist!(U4) as IndexShape>::Shape::new(&b).unwrap();
        let _: <Hlist!(U4) as IndexShape>::Shape = hlist!(3);
        // assert_eq!(b.index(hlist!(1),hlist!()), 2);
        assert_eq!(b[(hlist!(1), hlist!())], 2);
        let b: BasicMatrix<isize, U2, U2> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(b[(hlist!(1), hlist!(1))], 4);
        assert_eq!(b[[1, 1]], 4);
    }

    #[test]
    fn vec_add_test() {
        let a: BasicArray<isize, Hlist!(U4), HNil> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicArray<isize, Hlist!(U4), HNil> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let c: BasicArray<isize, Hlist!(U4), HNil> = BasicArray::from_vec(vec![2, 4, 6, 8]);
        assert_eq!(<Addition as BinaryOperator<_, _>>::operate(&a, &b), c);
    }

    #[test]
    fn mat_mul_test() {
        let a: BasicMatrix<isize, U2, U2> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicMatrix<isize, U2, U2> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let c: BasicMatrix<isize, U2, U2> = BasicArray::from_vec(vec![7, 10, 15, 22]);
        assert_eq!(<Multiplication as BinaryOperator<_, _>>::operate(&a, &b), c);
        assert_eq!(<Multiplication as BinaryOperator<_, _>>::operate(&a, &a), c);
    }

    #[test]
    fn lazy_add_owned_owned() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            a + b,
            LazyBinaryOperation {
                lhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((a + b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_owned_borrowed() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            a + &b,
            LazyBinaryOperation {
                lhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((a + &b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_borrowed_owned() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            &a + b,
            LazyBinaryOperation {
                lhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((&a + b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_borrowed_borrowed() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            &a + &b,
            LazyBinaryOperation {
                lhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((&a + &b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_2op() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let c: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!((a + b + c).eval(), BasicArray::from_vec(vec![3, 6, 9, 12]));
    }
}
