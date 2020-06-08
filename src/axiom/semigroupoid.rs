use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<Op, Mhs, Rhs>: Associativity<Op, Mhs, Rhs>
where
    Mhs: Clone + Morphism<Codomain = Domain<Self>>,
    Rhs: Clone + Morphism<Codomain = Domain<Mhs>>,
    Target<Op, Self, Mhs>: Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>>,
    Target<Op, Mhs, Rhs>: Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>>,
    Target<Op, Self, Target<Op, Mhs, Rhs>>:
        Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    Target<Op, Self, Target<Op, Mhs, Rhs>>: Sized + PartialEq + Clone,
    Op: BinaryOperator<Self, Mhs>
        + BinaryOperator<Mhs, Rhs>
        + BinaryOperator<Self, Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, Rhs, Output = Target<Op, Self, Target<Op, Mhs, Rhs>>>,
{
}

impl<Op, Lhs, Mhs, Rhs> Semigroupoid<Op, Mhs, Rhs> for Lhs
where
    Lhs: Associativity<Op, Mhs, Rhs>,
    Mhs: Clone + Morphism<Codomain = Domain<Self>>,
    Rhs: Clone + Morphism<Codomain = Domain<Mhs>>,
    Target<Op, Self, Mhs>: Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>>,
    Target<Op, Mhs, Rhs>: Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>>,
    Target<Op, Self, Target<Op, Mhs, Rhs>>:
        Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    Target<Op, Self, Target<Op, Mhs, Rhs>>: Sized + PartialEq + Clone,
    Op: BinaryOperator<Self, Mhs>
        + BinaryOperator<Mhs, Rhs>
        + BinaryOperator<Self, Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, Rhs, Output = Target<Op, Self, Target<Op, Mhs, Rhs>>>,
{
}
