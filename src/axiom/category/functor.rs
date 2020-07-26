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

pub trait Functor<Cod>: HigherKind<Cod> {
    // U is Codomain
    // X , Y: Object
    // f: X -> Y
    // F: Functor
    // fmap: f -> F(f)
    fn fmap<'a, F>(f: F) -> Box<dyn FnOnce(Self::MappedSource) -> Self::MappedTarget + 'a>
    where
        F: FnOnce(Self::Source) -> Self::Target + 'a;

    fn map<F>(self, f: F) -> Self::MappedTarget
    where
        F: FnOnce(Self::Source) -> Self::Target;
}

// Option<Dom> is F(X)
// F is Option?
impl<Dom, Cod> Functor<Cod> for Option<Dom> {
    fn fmap<'a, F>(f: F) -> Box<dyn FnOnce(Self::MappedSource) -> Self::MappedTarget + 'a>
    where
        F: FnOnce(Dom) -> Cod + 'a,
    {
        Box::new(|x: Self::MappedSource| x.map(f))
    }

    fn map<F>(self, f: F) -> Self::MappedTarget
    where
        F: FnOnce(Self::Source) -> Self::Target,
    {
        self.map(f)
    }
}

// ApplicativeFunctor = lax monoidal functor

pub trait Apply<A>: HigherKind<A> {
    // fs = F(A -> B)
    // Apply : F(A) -> F(A -> B) -> F(B)
    fn ap<F>(
        self,
        fs: <Self as HigherKind<F>>::MappedTarget,
    ) -> <Self as HigherKind<A>>::MappedTarget
    where
        F: Fn(<Self as HigherKind<A>>::Source) -> <Self as HigherKind<A>>::Target,
        Self: HigherKind<F>;
}

impl<A, B> Apply<B> for Option<A> {
    fn ap<F>(
        self,
        fs: <Self as HigherKind<F>>::MappedTarget,
    ) -> <Self as HigherKind<B>>::MappedTarget
    where
        F: Fn(A) -> B,
    {
        fs.and_then(|ref f| self.map(f))
    }
}

pub trait Pure<A>: HigherKind<A> {
    // x is Object,
    // Pure : x -> F(x)
    fn pure(value: A) -> <Self as HigherKind<A>>::MappedTarget;
}

impl<A> Pure<A> for Option<A> {
    fn pure(value: A) -> Self {
        Option::Some(value)
    }
}
