use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<Op, Mhs, Rhs>: Associativity<Op, Mhs, Rhs>
where
    Mhs: Sized + Morphism<Codomain = Domain<Self>>,
    Rhs: Sized + Morphism<Codomain = Domain<Mhs>>,
    Target<Op, Self, Mhs>: Sized + Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>>,
    Target<Op, Mhs, Rhs>: Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>>,
    Target<Op, Self, Target<Op, Mhs, Rhs>>:
        Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    Op: BinaryOperator<Self, Mhs>
        + BinaryOperator<Mhs, Rhs>
        + BinaryOperator<Self, Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, Rhs, Output = Target<Op, Self, Target<Op, Mhs, Rhs>>>,
{
}

impl<Op, Lhs, Mhs, Rhs> Semigroupoid<Op, Mhs, Rhs> for Lhs
where
    Lhs: Associativity<Op, Mhs, Rhs>,
    Mhs: Sized + Morphism<Codomain = Domain<Self>>,
    Rhs: Sized + Morphism<Codomain = Domain<Mhs>>,
    Target<Op, Self, Mhs>: Sized + Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>>,
    Target<Op, Mhs, Rhs>: Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>>,
    Target<Op, Self, Target<Op, Mhs, Rhs>>:
        Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    Op: BinaryOperator<Self, Mhs>
        + BinaryOperator<Mhs, Rhs>
        + BinaryOperator<Self, Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, Rhs, Output = Target<Op, Self, Target<Op, Mhs, Rhs>>>,
{
}
