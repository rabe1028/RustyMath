use crate::axiom::*;
use crate::operator::*;
use crate::property::*;
use std::borrow::Cow;

pub trait HigherKind<U> {
    type Source; // X
    type Target; // Y
    // type Arrow; // X -> Y
    type MappedSource; // F(X)
    type MappedTarget; // F(Y)
    // type MappedArrow; // F(X -> Y)
}

impl<T, U> HigherKind<U> for Option<T> {
    type Source = T;
    type Target = U;
    // type Arrow = Box<Fn(T) -> U>;
    type MappedSource = Option<T>; // Self
    type MappedTarget = Option<U>;
    // type MappedArrow = Option<Self::Arrow>;
}

pub trait Functor<U>: HigherKind<U> {
    fn fmap<F>(f: F) -> Box<dyn Fn(Self::MappedSource) -> Self::MappedTarget>
    where
        F: FnOnce(Self::Source) -> Self::Target;

    fn map<F>(self, f: F) -> Self::MappedTarget
    where
        F: FnOnce(Self::Source) -> Self::Target;
}

impl<T, U> Functor<U> for Option<T> {
    fn fmap<F>(f: F) -> Box<dyn Fn(Self::MappedSource) -> Self::MappedTarget> {
        Box::new(|x: Self::MappedSource| x.map(f))
    }
    fn map<F>(self, f: F) -> Self::MappedTarget {
        self.map(f)
    }
}

pub trait Apply<A>: Functor<A> {
    // fs = F(A -> B)
    // F(A) -> F(A -> B) -> F(B)
    fn apply<F>(
        &self,
        fs: <Self as HigherKind<F>>::MappedTarget,
    ) -> <Self as HigherKind<A>>::MappedTarget
    where
        F: FnOnce(&Self::Source) -> Self::MappedTarget;
}

pub trait Pure<A>: HigherKind<A> {
    fn pure(value: A) -> <Self as HigherKind<A>>::MappedTarget;
}

