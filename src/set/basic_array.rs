use crate::operator::*;
use crate::property::*;
use crate::set::*;
use crate::util::*;

use std::marker::PhantomData;

use frunk::hlist::HList;
use frunk::*; //{HCons, HNil};
use typenum::uint::Unsigned;
use typenum::*;

use std::ops::Add;

pub mod basic_array_impl;
pub use basic_array_impl::*;

pub mod basic_matrix_impl;
pub use basic_matrix_impl::*;

pub mod basic_scalar_impl;
pub use basic_scalar_impl::*;

/*
Test Array Struct Implementation
*/

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    _inner: Vec<ElementType>,
    _contravariant: PhantomData<Contravariant>,
    _covariant: PhantomData<Covariant>,
}

impl<ElementType, Contravariant, Covariant> Morphism
    for BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Domain = ();
    type Codomain = ();
}

impl<ElementType, Contravariant, Covariant> Endomorphism
    for BasicArray<ElementType, Contravariant, Covariant>
where
    Contravariant: HList + IndexShape,
    Covariant: HList + IndexShape,
{
    type Object = ();
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
        let (offset, _) = <Self as Tensor<_, _, _>>::Joined::get_index(cont + cov);
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
        let (offset, _) = <Self as Tensor<_, _, _>>::Joined::get_index(cont + cov);
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

impl<ElementType> Scalar<ElementType> for BasicScalar<ElementType>
where
    ElementType: Scalar<ElementType> + Copy,
    Multiplication: InternalBinaryOperator<ElementType>,
    Addition: InternalBinaryOperator<ElementType>,
{
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

pub type BasicScalar<ElementType> = BasicArray<ElementType, HNil, HNil>;

pub type BasicVector<ElementType, _1> = BasicArray<ElementType, Hlist!(_1), HNil>;

pub type BasicMatrix<ElementType, _1, _2> = BasicArray<ElementType, Hlist!(_1), Hlist!(_2)>;

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
    Rhs: InputSanitizer,  //Right
    Addition: BinaryOperator<Sanitize<Self>, Sanitize<Rhs>>,
{
    type Output = LazyBinaryOperation<Addition, Self, Rhs>;
    fn add(self, other: Rhs) -> Self::Output {
        LazyBinaryOperation::new(self, other)
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
        LazyBinaryOperation::new(self, other)
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
        println!("{:?}", a);
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
            LazyBinaryOperation::new(
                BasicArray::from_vec(vec![1, 2, 3, 4]),
                BasicArray::from_vec(vec![1, 2, 3, 4]),
            )
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
            LazyBinaryOperation::new(
                BasicArray::from_vec(vec![1, 2, 3, 4]),
                &BasicArray::from_vec(vec![1, 2, 3, 4]),
            )
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
            LazyBinaryOperation::new(
                &BasicArray::from_vec(vec![1, 2, 3, 4]),
                BasicArray::from_vec(vec![1, 2, 3, 4]),
            )
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
            LazyBinaryOperation::new(
                &BasicArray::from_vec(vec![1, 2, 3, 4]),
                &BasicArray::from_vec(vec![1, 2, 3, 4]),
            )
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
