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
{
}

impl<ElementType, Contravariant, Covariant> Totality<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
}

impl<ElementType, Contravariant, Covariant> Associativity<Addition>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy + PartialEq,
    Contravariant: HList + IndexShape + PartialEq + Add<Covariant>,
    Covariant: HList + IndexShape + PartialEq,
{
}

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
// これにより，LazyBinaryOperatorOO一つで管理可能
// (他を使うと，借用先で借用することになり，失敗する)

pub trait LazyOperation {
    type Output: std::clone::Clone;
    fn eval(self) -> Self::Output;
}

pub trait InputSanitizer {
    type InputShape: std::clone::Clone;
    fn sanitize<'a>(self) -> std::borrow::Cow<'a, Self::InputShape>
    where
        Self: 'a;
}

impl<ElementType, Contravariant, Covariant> InputSanitizer
    for BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: std::clone::Clone + HList + IndexShape + Add<Covariant>,
    Covariant: std::clone::Clone + HList + IndexShape,
{
    type InputShape = Self;
    fn sanitize<'a>(self) -> std::borrow::Cow<'a, Self::InputShape> {
        Cow::Owned(self)
    }
}

type Sanitize<A> = <A as InputSanitizer>::InputShape;

impl<ElementType, Contravariant, Covariant> InputSanitizer
    for &'_ BasicArray<ElementType, Contravariant, Covariant>
where
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: std::clone::Clone + HList + IndexShape + Add<Covariant>,
    Covariant: std::clone::Clone + HList + IndexShape,
{
    type InputShape = BasicArray<ElementType, Contravariant, Covariant>;
    fn sanitize<'a>(self) -> std::borrow::Cow<'a, Self::InputShape>
    where
        Self: 'a,
    {
        Cow::Borrowed(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct LazyBinaryOperationOO<Op, Left, Right, Output>
where
    Left: std::clone::Clone + InputSanitizer,
    Right: std::clone::Clone + InputSanitizer,
    Op: std::clone::Clone + BinaryOperator<Sanitize<Left>, Sanitize<Right>, Output>,
{
    lhs: Left,
    rhs: Right,
    _op: PhantomData<Op>,
    _out: PhantomData<Output>,
}

impl<'a, Op, Left, Right, Output> LazyOperation for LazyBinaryOperationOO<Op, Left, Right, Output>
where
    Left: std::clone::Clone + InputSanitizer,
    Right: std::clone::Clone + InputSanitizer,
    Op: std::clone::Clone + BinaryOperator<Sanitize<Left>, Sanitize<Right>, Output>,
    Output: std::clone::Clone,
{
    type Output = Output;
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
//     // LazyBinaryOperationOO::OutputとRhsの加算
//     type Output = LazyBinaryOperationOO<
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
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: std::clone::Clone + HList + IndexShape + Add<Covariant>,
    Covariant: std::clone::Clone + HList + IndexShape,
    Self: std::clone::Clone + InputSanitizer, //Left
    Rhs: std::clone::Clone + InputSanitizer<InputShape = Self>, //Right
    Addition: std::clone::Clone + BinaryOperator<Sanitize<Self>, Sanitize<Rhs>, Self>,
{
    type Output = LazyBinaryOperationOO<Addition, Self, Rhs, Self>;
    fn add(self, other: Rhs) -> Self::Output {
        LazyBinaryOperationOO {
            lhs: self,
            rhs: other,
            _op: PhantomData,
            _out: PhantomData,
        }
    }
}

impl<ElementType, Contravariant, Covariant, Rhs> Add<Rhs>
    for &'_ BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: std::clone::Clone + HList + IndexShape + Add<Covariant>,
    Covariant: std::clone::Clone + HList + IndexShape,
    Self: std::clone::Clone + InputSanitizer, //Left
    Rhs: std::clone::Clone + InputSanitizer,  //Right
    Addition: std::clone::Clone + BinaryOperator<Sanitize<Self>, Sanitize<Rhs>, Sanitize<Self>>,
{
    type Output = LazyBinaryOperationOO<Addition, Self, Rhs, Sanitize<Self>>;
    fn add(self, other: Rhs) -> Self::Output {
        LazyBinaryOperationOO {
            lhs: self,
            rhs: other,
            _op: PhantomData,
            _out: PhantomData,
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
    fn lazy_add_owned_owned() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            a + b,
            LazyBinaryOperationOO {
                lhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
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
            LazyBinaryOperationOO {
                lhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
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
            LazyBinaryOperationOO {
                lhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
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
            LazyBinaryOperationOO {
                lhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((&a + &b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    // #[test]
    // fn lazy_add_2op() {
    //     let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
    //     let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
    //     let c: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

    //     assert_eq!((a + b + c).eval(), BasicArray::from_vec(vec![3, 6, 9, 12]));
    // }
}
