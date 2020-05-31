use std::borrow::Cow;
use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

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


