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

impl<'a, A> Endomorphism for Box<dyn FnOnce(A) -> A + 'a> {
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

impl<'a, A, B, C, D>
    Category<
        Compose,
        Box<dyn FnOnce(B) -> C + 'a>,
        Box<dyn FnOnce(A) -> B + 'a>,
        Box<dyn FnOnce(A) -> A + 'a>,
        Box<dyn FnOnce(B) -> B + 'a>,
        Box<dyn FnOnce(C) -> C + 'a>,
        Box<dyn FnOnce(D) -> D + 'a>,
    > for Box<dyn FnOnce(C) -> D + 'a>
where
    Self: Semigroupoid<Compose, Box<dyn FnOnce(B) -> C + 'a>, Box<dyn FnOnce(A) -> B + 'a>>,
    Box<dyn FnOnce(A) -> B + 'a>: Morphism<Domain = A, Codomain = B>,
    Box<dyn FnOnce(B) -> C + 'a>: Morphism<Domain = B, Codomain = C>,
    Box<dyn FnOnce(C) -> D + 'a>: Morphism<Domain = C, Codomain = D>,
    Box<dyn FnOnce(A) -> A + 'a>: Endomorphism<Object = A> + Identity<Compose>,
    Box<dyn FnOnce(B) -> B + 'a>: Endomorphism<Object = B> + Identity<Compose>,
    Box<dyn FnOnce(C) -> C + 'a>: Endomorphism<Object = C> + Identity<Compose>,
    Box<dyn FnOnce(D) -> D + 'a>: Endomorphism<Object = D> + Identity<Compose>,
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
    Compose: BinaryOperator<
            Box<dyn FnOnce(A) -> B + 'a>,
            Box<dyn FnOnce(A) -> A + 'a>,
            Output = Box<dyn FnOnce(A) -> B + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(B) -> B + 'a>,
            Box<dyn FnOnce(A) -> B + 'a>,
            Output = Box<dyn FnOnce(A) -> B + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(B) -> C + 'a>,
            Box<dyn FnOnce(B) -> B + 'a>,
            Output = Box<dyn FnOnce(B) -> C + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(C) -> C + 'a>,
            Box<dyn FnOnce(B) -> C + 'a>,
            Output = Box<dyn FnOnce(B) -> C + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(C) -> D + 'a>,
            Box<dyn FnOnce(C) -> C + 'a>,
            Output = Box<dyn FnOnce(C) -> D + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(D) -> D + 'a>,
            Box<dyn FnOnce(C) -> D + 'a>,
            Output = Box<dyn FnOnce(C) -> D + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(A) -> C + 'a>,
            Box<dyn FnOnce(A) -> A + 'a>,
            Output = Box<dyn FnOnce(A) -> C + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(C) -> C + 'a>,
            Box<dyn FnOnce(A) -> C + 'a>,
            Output = Box<dyn FnOnce(A) -> C + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(B) -> D + 'a>,
            Box<dyn FnOnce(B) -> B + 'a>,
            Output = Box<dyn FnOnce(B) -> D + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(D) -> D + 'a>,
            Box<dyn FnOnce(B) -> D + 'a>,
            Output = Box<dyn FnOnce(B) -> D + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(A) -> D + 'a>,
            Box<dyn FnOnce(A) -> A + 'a>,
            Output = Box<dyn FnOnce(A) -> D + 'a>,
        > + BinaryOperator<
            Box<dyn FnOnce(D) -> D + 'a>,
            Box<dyn FnOnce(A) -> D + 'a>,
            Output = Box<dyn FnOnce(A) -> D + 'a>,
        >,
    Compose: InternalBinaryOperator<Box<dyn FnOnce(A) -> A + 'a>>
        + InternalBinaryOperator<Box<dyn FnOnce(B) -> B + 'a>>
        + InternalBinaryOperator<Box<dyn FnOnce(C) -> C + 'a>>
        + InternalBinaryOperator<Box<dyn FnOnce(D) -> D + 'a>>,
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
        let c = Compose::operate(a, b);
        assert_eq!(c(1), 1);
    }

    #[test]
    fn category_test() {
        let a = Box::new(|x: usize| identity(x));
        a._category()
    }
}
