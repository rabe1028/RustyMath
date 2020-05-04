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

// struct LazyBinaryOperation<'a, Op, Left, Right, Output>
// where
//     Op: BinaryOperator<Left, Right, Output>,
//     Left: 'a + std::clone::Clone + Into<std::borrow::Cow<'a, Left>>,
//     Right: 'a + std::clone::Clone + Into<std::borrow::Cow<'a, Right>>,
// {
//     lhs: Left,
//     rhs: Right,
//     _op: PhantomData<&'a Op>,
//     _out: PhantomData<Output>,
// }

pub trait LazyOperation {
    type Output;
    fn eval(self) -> Self::Output;
}

#[derive(Debug, Eq, PartialEq)]
struct LazyBinaryOperationOO<Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
{
    lhs: Left,
    rhs: Right,
    _op: PhantomData<Op>,
    _out: PhantomData<Output>,
}

#[derive(Debug, Eq, PartialEq)]
struct LazyBinaryOperationBO<'a, Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
{
    lhs: &'a Left,
    rhs: Right,
    _op: PhantomData<Op>,
    _out: PhantomData<Output>,
}

#[derive(Debug, Eq, PartialEq)]
struct LazyBinaryOperationOB<'a, Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
{
    lhs: Left,
    rhs: &'a Right,
    _op: PhantomData<Op>,
    _out: PhantomData<Output>,
}

#[derive(Debug, Eq, PartialEq)]
struct LazyBinaryOperationBB<'a, Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
{
    lhs: &'a Left,
    rhs: &'a Right,
    _op: PhantomData<Op>,
    _out: PhantomData<Output>,
}

impl<'a, Op, Left, Right, Output> LazyOperation for LazyBinaryOperationOO<Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
    Left: 'a + Into<std::borrow::Cow<'a, Left>>,
    Right: 'a + Into<std::borrow::Cow<'a, Right>>,
{
    type Output = Output;
    fn eval(self) -> Self::Output {
        Op::operate(self.lhs, self.rhs)
    }
}

impl<'a, Op, Left, Right, Output> LazyOperation
    for LazyBinaryOperationBO<'a, Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
    &'a Left: Into<std::borrow::Cow<'a, Left>>,
    Right: 'a + Into<std::borrow::Cow<'a, Right>>,
{
    type Output = Output;
    fn eval(self) -> Self::Output {
        Op::operate(self.lhs, self.rhs)
    }
}

impl<'a, Op, Left, Right, Output> LazyOperation
    for LazyBinaryOperationOB<'a, Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
    Left: 'a + Into<std::borrow::Cow<'a, Left>>,
    &'a Right: Into<std::borrow::Cow<'a, Right>>,
{
    type Output = Output;
    fn eval(self) -> Output {
        Op::operate(self.lhs, self.rhs)
    }
}

impl<'a, Op, Left, Right, Output> LazyOperation
    for LazyBinaryOperationBB<'a, Op, Left, Right, Output>
where
    Op: BinaryOperator<Left, Right, Output>,
    Left: std::clone::Clone,
    Right: std::clone::Clone,
    &'a Left: Into<std::borrow::Cow<'a, Left>>,
    &'a Right: Into<std::borrow::Cow<'a, Right>>,
{
    type Output = Output;
    fn eval(self) -> Self::Output {
        Op::operate(self.lhs, self.rhs)
    }
}

impl<ElementType, Contravariant, Covariant> Add<BasicArray<ElementType, Contravariant, Covariant>>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
    type Output = LazyBinaryOperationOO<Addition, Self, Self, Self>;
    fn add(self, other: Self) -> Self::Output {
        LazyBinaryOperationOO {
            lhs: self,
            rhs: other,
            _op: PhantomData,
            _out: PhantomData,
        }
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    Add<&'a BasicArray<ElementType, Contravariant, Covariant>>
    for BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
    type Output = LazyBinaryOperationOB<'a, Addition, Self, Self, Self>;
    fn add(self, other: &'a Self) -> Self::Output {
        LazyBinaryOperationOB {
            lhs: self,
            rhs: other,
            _op: PhantomData,
            _out: PhantomData,
        }
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    Add<BasicArray<ElementType, Contravariant, Covariant>>
    for &'a BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
    type Output = LazyBinaryOperationBO<
        'a,
        Addition,
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    >;
    fn add(self, other: BasicArray<ElementType, Contravariant, Covariant>) -> Self::Output {
        LazyBinaryOperationBO {
            lhs: self,
            rhs: other,
            _op: PhantomData,
            _out: PhantomData,
        }
    }
}

impl<'a, ElementType, Contravariant, Covariant>
    Add<&'a BasicArray<ElementType, Contravariant, Covariant>>
    for &'a BasicArray<ElementType, Contravariant, Covariant>
where
    BasicArray<ElementType, Contravariant, Covariant>: Clone,
    ElementType: std::ops::Add<Output = ElementType> + Copy,
    Contravariant: HList + IndexShape + Add<Covariant>,
    Covariant: HList + IndexShape,
{
    type Output = LazyBinaryOperationBB<
        'a,
        Addition,
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
        BasicArray<ElementType, Contravariant, Covariant>,
    >;
    fn add(self, other: Self) -> Self::Output {
        LazyBinaryOperationBB {
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
        assert_eq!((a+b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_owned_borrowed() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            a + &b,
            LazyBinaryOperationOB {
                lhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((a+&b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_borrowed_owned() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            &a + b,
            LazyBinaryOperationBO {
                lhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((&a+b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }

    #[test]
    fn lazy_add_borrowed_borrowed() {
        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);

        assert_eq!(
            &a + &b,
            LazyBinaryOperationBB {
                lhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                rhs: &BasicArray::from_vec(vec![1, 2, 3, 4]),
                _op: PhantomData,
                _out: PhantomData,
            }
        );

        let a: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        let b: BasicVector<isize, U4> = BasicArray::from_vec(vec![1, 2, 3, 4]);
        assert_eq!((&a+&b).eval(), BasicArray::from_vec(vec![2, 4, 6, 8]));
    }
}
