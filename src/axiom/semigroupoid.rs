use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<Op>: Associativity<Op>
where
    Rhs: Sized + Morphism<Codomain = Domain<Self>>,
    for <Rhs> Target<Op, Self, Rhs>: Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    Op: BinaryOperator<Self, Rhs>
{
    fn _semigroupoid(&self) {}
}

impl<Op, Lhs, Rhs> Semigroupoid<Op, Rhs> for Lhs
where
    Lhs: Associativity<Op>,
    Rhs: Sized + Morphism<Codomain = Domain<Self>>,
    Target<Op, Self, Rhs>: Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    Op: BinaryOperator<Self, Rhs>
{
}
