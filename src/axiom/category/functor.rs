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
    // U is Codomain
    // X , Y: Object
    // f: X -> Y
    // F: Functor
    // fmap: f -> F(f)
    fn fmap<'a, F>(f: F) -> Box<dyn FnOnce(Self::MappedSource) -> Self::MappedTarget + 'a>
    where
        F: FnOnce(Self::Source) -> Self::Target + 'a;

    // pure: X -> F(X)
    fn pure(value: Self::Source) -> Self;

    // apply: F(X) -> X -> Y -> F(Y)
    // fn apply<F>(
    //     &self,
    //     f: <Self as HigherKind<F>>::MappedTarget,
    // ) -> <Self as HigherKind<U>>::MappedTarget
    // where
    //     F: Fn(&<Self as HigherKind<U>>::Source) -> <Self as HigherKind<U>>::Target,
    //     Self: HigherKind<F>;

    fn map<F>(self, f: F) -> Self::MappedTarget
    where
        F: FnOnce(Self::Source) -> Self::Target;
}

impl<T, U> Functor<U> for Option<T> {
    fn fmap<'a, F>(f: F) -> Box<dyn FnOnce(Self::MappedSource) -> Self::MappedTarget + 'a>
    where
        F: FnOnce(T) -> U + 'a,
    {
        Box::new(|x: Self::MappedSource| x.map(f))
    }

    fn pure(value: T) -> Self {
        Option::Some(value)
    }

    // fn apply<F>(
    //     &self,
    //     f: <Self as HigherKind<F>>::MappedTarget,
    // ) -> <Self as HigherKind<U>>::MappedTarget
    // where
    //     F: Fn(&<Self as HigherKind<U>>::Source) -> <Self as HigherKind<U>>::Target,
    //     Self: HigherKind<F>,
    // {
    //     match self {
    //         &Some(ref value) => match f {
    //             Some(fs) => Some(fs(value)),
    //             None => None,
    //         },
    //         &None => None,
    //     }
    // }

    fn map<F>(self, f: F) -> Self::MappedTarget
    where
        F: FnOnce(Self::Source) -> Self::Target,
    {
        self.map(f)
    }
}

// pub trait Apply<A>: Functor<A> {
//     // fs = F(A -> B)
//     // Apply : F(A) -> F(A -> B) -> F(B)
//     fn apply<F>(
//         &self,
//         fs: <Self as HigherKind<F>>::MappedTarget,
//     ) -> <Self as HigherKind<A>>::MappedTarget
//     where
//         F: FnOnce(&<Self as HigherKind<A>>::Source) -> <Self as HigherKind<A>>::MappedTarget,
//         Self: HigherKind<F>;
// }

// pub trait Pure<A>: HigherKind<A> {
//     // x is Object,
//     // Pure : x -> F(x)
//     fn pure(value: A) -> <Self as HigherKind<A>>::MappedTarget;
// }

// ApplicativeFunctor = lax monoidal functor


pub trait Apply<A>: HigherKind<A> {
    fn ap<F>(&self, fs: <Self as HigherKind<F>>::MappedTarget) -> <Self as HigherKind<A>>::MappedTarget
    where
        F: Fn(&<Self as HigherKind<A>>::Source) -> <Self as HigherKind<A>>::Target,
        Self: HigherKind<F>;
}

impl<A, B> Apply<B> for Option<A> {
    fn ap<F>(&self, fs: <Self as HigherKind<F>>::MappedTarget) -> <Self as HigherKind<B>>::MappedTarget
    where
        F: Fn(&A) -> B,
    {
        match self {
            &Some(ref value) => match fs {
                Some(f) => Some(f(value)),
                None => None,
            },
            &None => None,
        }
    }
}