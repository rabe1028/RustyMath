use crate::operator::*;
use crate::property::*;
use crate::set::*;
use crate::util::*;

use std::ops::*;

// F: FnOnce(A) -> Bの制約だと，Unconstrained Type Parameterで怒られる
impl<'a, A, B> Morphism for Box<dyn FnOnce(A) -> B + 'a> {
    type Domain = A;
    type Codomain = B;
}

// impl<A, B, F> Morphism<(A, B)> for F where F: FnOnce(A) -> B {
//     type Domain = A;
//     type Codomain = B;
// }

impl<'a, A> Endomorphism for Box<dyn FnOnce(A) -> A + 'a> {
    type Object = A;
}

// Endomorphism<A> == Endomorphism<(A)> == Endomorphism<()> ???
// Endomorphism<A>だと，Morphism<A>がないからエラー <(A)>も同じ
// -> エラーメッセージがComflicting Implementationなのが謎
// -> A == ()の時，上記全て()に変換されるみたい
// 　　コンパイル時にいらない括弧はないものにされるみたい
// Endomorphism<(A,A)>だと，Morphism<(A,A)>に接続されるからOKみたい
// impl<A, F> Endomorphism<(A, A)> for F where F: FnOnce(A) -> A {
//     type Object = A;
// }

impl<'a, A, B, C> BinaryOperator<Box<dyn FnOnce(B) -> C + 'a>, Box<dyn FnOnce(A) -> B + 'a>>
    for Compose
where
    A: 'a,
    B: 'a,
    C: 'a,
{
    type Output = Box<dyn FnOnce(A) -> C + 'a>;
    fn operate(
        lhs: Box<dyn FnOnce(B) -> C + 'a>,
        rhs: Box<dyn FnOnce(A) -> B + 'a>,
    ) -> Self::Output {
        Box::new(|x: A| lhs(rhs(x)))
    }
}

impl<'a, A> InternalBinaryOperator<Box<dyn FnOnce(A) -> A + 'a>> for Compose where A: 'a {}

impl<'a, A, B> LeftIdentity<Compose, Box<dyn FnOnce(A) -> A + 'a>> for Box<dyn FnOnce(A) -> B + 'a>
where
    Compose: BinaryOperator<Self, Box<dyn FnOnce(A) -> A + 'a>, Output = Self>,
{
    #[inline(always)]
    fn left_identity() -> Box<dyn FnOnce(A) -> A + 'a> {
        use std::convert::identity;
        Box::new(|x| identity(x))
    }
}

impl<'a, A, B> RightIdentity<Compose, Box<dyn FnOnce(B) -> B + 'a>> for Box<dyn FnOnce(A) -> B + 'a>
where
    Compose: BinaryOperator<Box<dyn FnOnce(B) -> B + 'a>, Self, Output = Self>,
{
    #[inline(always)]
    fn right_identity() -> Box<dyn FnOnce(B) -> B + 'a> {
        use std::convert::identity;
        Box::new(|x| identity(x))
    }
}

impl<'a, A> Identity<Compose> for Box<dyn FnOnce(A) -> A + 'a>
where
    Compose: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn identity() -> Self {
        use std::convert::identity;
        Box::new(|x| identity(x))
    }
}

impl<'a, A, B, C, D>
    Associativity<Compose, Box<dyn FnOnce(B) -> C + 'a>, Box<dyn FnOnce(A) -> B + 'a>>
    for Box<dyn FnOnce(C) -> D + 'a>
where
    Compose: BinaryOperator<Self, Box<dyn FnOnce(B) -> C + 'a>, Output = Box<dyn FnOnce(B) -> D + 'a>>
        + BinaryOperator<
            Box<dyn FnOnce(B) -> C + 'a>,
            Box<dyn FnOnce(A) -> B + 'a>,
            Output = Box<dyn FnOnce(A) -> C + 'a>,
        > + BinaryOperator<Self, Box<dyn FnOnce(A) -> C + 'a>, Output = Box<dyn FnOnce(A) -> D + 'a>>
        + BinaryOperator<
            Box<dyn FnOnce(B) -> D + 'a>,
            Box<dyn FnOnce(A) -> B + 'a>,
            Output = Box<dyn FnOnce(A) -> D + 'a>,
        >,
{
}

#[cfg(test)]
mod tests {
    use crate::set::function::*;
    use std::convert::identity;

    #[test]
    fn compose() {
        let a = Box::new(|x: i32| identity(x));
        let b = Box::new(|x: i32| identity(x) + 1);
        let c = Compose::operate(a, b);
        assert_eq!(c(1), 2);
    }

    #[test]
    fn compose_other() {
        let a = Box::new(|x: i32| x as i64);
        let b = Box::new(|x: usize| x as i32);
        // 何故かここでimpl Traitと型推測できているみたい
        let c = Compose::operate(a, b);
        assert_eq!(c(1), 1);
    }

    #[test]
    fn trait_impl_test() {
        type FnObj<A, B> = Box<dyn FnOnce(A) -> B>;
        // You Need To Add Type Annotation
        // Because if you remove type annotation,
        // variable type is become concrete type, not to become trait object
        let a = Box::new(|x: usize| identity(x));
        let a: FnObj<usize, usize> = a;
        Morphism::_morphism(&a);
        // Semigroupoid::<Compose, _, _>::_semigroupoid(&a);
        // Cannot use this type args
        //a._semigroupoid::<Compose, >();
        Semigroupoid::<Compose, FnObj<usize, usize>, FnObj<usize, usize>>::_semigroupoid(&a);
        //Category::_category(&a);
        fn test(x: usize) -> usize {
            x
        }
        let a = Box::new(test);
        let a: FnObj<usize, usize> = a;
        Morphism::_morphism(&a);
    }
}
