use crate::operator::*;
use crate::property::*;

pub trait Semigroupoid<Op, G, H>: Associativity<Op, G, H>
where
    G: Clone + Morphism<Domain = Codomain<Self>>,
    H: Clone + Morphism<Domain = Codomain<G>>,
    Target<Op, G, Self>: Morphism<Domain = Domain<Self>, Codomain = Codomain<G>>,
    Target<Op, H, G>: Morphism<Domain = Domain<G>, Codomain = Codomain<H>>,
    Target<Op, H, Target<Op, G, Self>>: Morphism<Domain = Domain<Self>, Codomain = Codomain<H>>,
    Target<Op, H, Target<Op, G, Self>>: Sized + PartialEq + Clone,
    Op: BinaryOperator<G, Self>
        + BinaryOperator<H, G>
        + BinaryOperator<H, Target<Op, G, Self>>
        + BinaryOperator<Target<Op, H, G>, Self, Output = Target<Op, H, Target<Op, G, Self>>>,
{
}

impl<Op, F, G, H> Semigroupoid<Op, G, H> for F
where
    F: Associativity<Op, G, H>,
    G: Clone + Morphism<Domain = Codomain<Self>>,
    H: Clone + Morphism<Domain = Codomain<G>>,
    Target<Op, G, Self>: Morphism<Domain = Domain<Self>, Codomain = Codomain<G>>,
    Target<Op, H, G>: Morphism<Domain = Domain<G>, Codomain = Codomain<H>>,
    Target<Op, H, Target<Op, G, Self>>: Morphism<Domain = Domain<Self>, Codomain = Codomain<H>>,
    Target<Op, H, Target<Op, G, Self>>: Sized + PartialEq + Clone,
    Op: BinaryOperator<G, Self>
        + BinaryOperator<H, G>
        + BinaryOperator<H, Target<Op, G, Self>>
        + BinaryOperator<Target<Op, H, G>, Self, Output = Target<Op, H, Target<Op, G, Self>>>,
{
}
