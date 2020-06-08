use crate::operator::*;
use crate::property::*;
use crate::set::*;
use crate::util::*;

use std::ops::*;

// F: FnOnce(A) -> Bの制約だと，Unconstrained Type Parameterで怒られる
impl<A, B> Morphism for Box<dyn FnOnce(A) -> B> {
    type Domain = A;
    type Codomain = B;
}

impl<A> Endomorphism for Box<dyn FnOnce(A) -> A> {
    type Object = A;
}

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

// impl<Lhs, Mhs, Rhs> Associativity<Mhs, Rhs> for Lhs
