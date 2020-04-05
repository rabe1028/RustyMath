use std::borrow::Cow;

pub trait HigherKind<U> {
    type Source; // X
    type Target; // Y
    type Arrow; // X -> Y
    type MappedSource; // F(X)
    type MappedTarget; // F(Y)
    type MappedArrow; // F(X -> Y)
}

impl<T, U> HigherKind<U> for Option<T> {
    type Source = T;
    type Target = U;
    type Arrow = Box<Fn(T) -> U>;
    type MappedSource = Option<T>;
    type MappedTarget = Option<U>;
    type MappedArrow = Option<Self::Arrow>;
}

trait Functor<U>: HigherKind<U> {
    fn fmap<F>(f: F) -> Box<dyn Fn(Self::MappedSource) -> Self::MappedTarget>
    where
        F: FnOnce(Self::Source) -> Self::Target;

    fn map<F>(self, f: F) -> Self::MappedTarget
    where
        F: FnOnce(Self::Source) -> Self::Target;
}

trait ApplicativeFunctor<U>: Functor<U> {
    fn apply(f: Self::MappedArrow) -> Box<Fn(Self::MappedSource) -> Self::MappedTarget>;
}

use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

struct TestCategory {}

/*
trait TestBinaryOperator<T> {
    fn operate<A, B, C, FAB, FBC, FAC>(lhs: Box<FAB>, rhs: Box<FBC>) -> Box<Fn(A) -> C >
    where
        FAB: Fn(A) -> B,
        FBC: Fn(B) -> C;

    fn _operate<'a, A, B, C>(lhs: impl Fn(A) -> B + 'a, rhs: impl Fn(B) -> C + 'a) -> Box< Fn(A) -> C > {
            Box::new(|a: A| rhs(lhs(a)))
    }
}

impl TestBinaryOperator<TestCategory> for Composition {
    fn operate<A, B, C, FAB, FBC, FAC>(lhs: Box<FAB>, rhs: Box<FBC>) -> Box<Fn(A) -> C>
    where
        FAB: Fn(A) -> B,
        FBC: Fn(B) -> C,
    {
        Box::new(move |a| rhs(lhs(a)))
    }
}
*/

// function compose operator
enum Composition {}
impl Composition {
    fn operate<'a: 'c, 'b: 'c, 'c, A, B, C>(
        lhs: impl Fn(A) -> B + 'a,
        rhs: impl Fn(B) -> C + 'b,
    ) -> impl Fn(A) -> C + 'c {
        move |a| rhs(lhs(a))
    }
}


/*
enum TestComposition {}

impl<A,B,C,X,Y,Z> BinaryOperator<impl Fn(A) -> B,impl Fn(B) -> C, impl Fn(A) -> C> for TestComposition 
where X: Fn(A) -> B, Y: Fn(B) -> C, Z: Fn(A) -> C,
{
    fn operate<'a: 'c, 'b: 'c, 'c>(
        lhs: impl Into<Cow<'a, X>>, 
        rhs: impl Into<Cow<'b, Y>>,
    ) -> Z + 'c {

    }
}
*/

/*
impl<A, B, C> GeneralBinaryOperator<Box<Fn(A) -> B>, Box<Fn(B) -> C>, Box<Fn(A) -> C>>
    for Composition
{
    fn operate<'a : 'c, 'b: 'c, 'c>(lhs: Box<Fn(A) -> B + 'a>, rhs: Box<Fn(B) -> C + 'b>) -> Box<Fn(A) -> C + 'c> {
        Box::new(move |a| rhs(lhs(a)))
    }
}
*/

/*
impl Composition {
    fn operate<'a, 'b, A, B, C>(
        lhs: &'a dyn Fn(A) -> B,
        rhs: &'b dyn Fn(B) -> C,
    ) -> impl Fn(A) -> C + 'b
    where
        'a: 'b,
    {
        move |a: A| rhs(lhs(a))
    }

    fn test<'a: 'c, 'b: 'c, 'c, A, B, C>(
        lhs: &'a Fn(A) -> B,
        rhs: &'b Fn(B) -> C,
    ) -> impl Fn(A) -> C + 'c {
        move |a: A| rhs(lhs(a))
    }

    fn impl_operate<'a: 'c, 'b: 'c, 'c, A, B, C>(
        lhs: impl Fn(A) -> B + 'a,
        rhs: impl Fn(B) -> C + 'b,
    ) -> impl Fn(A) -> C + 'c {
        move |a: A| rhs(lhs(a))
    }
}
*/

trait Domain<T> {}

impl Domain<i32> for TestCategory {}
impl Domain<bool> for TestCategory {}
