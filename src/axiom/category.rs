use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

type ComposeTriple<Op, A, B, C> = Target<Op, A, Target<Op, B, C>>;

// Hom(x, x) : x is Object  has identity
pub trait Category<Op, G, H, HomAA, HomBB, HomCC, HomDD>: Semigroupoid<Op, G, H>
// + Identity<Op>
where
    G: Clone + Morphism<Domain = Codomain<Self>>,
    H: Clone + Morphism<Domain = Codomain<G>>,
    Target<Op, G, Self>: Morphism<Domain = Domain<Self>, Codomain = Codomain<G>>,
    Target<Op, H, G>: Morphism<Domain = Domain<G>, Codomain = Codomain<H>>,
    ComposeTriple<Op, H, G, Self>: Morphism<Domain = Domain<Self>, Codomain = Codomain<H>>,
    ComposeTriple<Op, H, G, Self>: Sized + PartialEq + Clone,
    HomAA: Endomorphism<Object = Domain<Self>> + Identity<Op>,
    HomBB: Endomorphism<Object = Domain<G>> + Identity<Op>,
    HomCC: Endomorphism<Object = Domain<H>> + Identity<Op>,
    HomDD: Endomorphism<Object = Codomain<H>> + Identity<Op>,
    // from Associativity
    Op: BinaryOperator<G, Self>
        + BinaryOperator<H, G>
        + BinaryOperator<H, Target<Op, G, Self>>
        + BinaryOperator<Target<Op, H, G>, Self, Output = ComposeTriple<Op, H, G, Self>>,
    // from Category
    Op: BinaryOperator<Self, HomAA, Output = Self>
        + BinaryOperator<HomBB, Self, Output = Self>
        + BinaryOperator<G, HomBB, Output = G>
        + BinaryOperator<HomCC, G, Output = G>
        + BinaryOperator<H, HomCC, Output = H>
        + BinaryOperator<HomDD, H, Output = H>
        + BinaryOperator<Target<Op, G, Self>, HomAA, Output = Target<Op, G, Self>>
        + BinaryOperator<HomCC, Target<Op, G, Self>, Output = Target<Op, G, Self>>
        + BinaryOperator<Target<Op, H, G>, HomBB, Output = Target<Op, H, G>>
        + BinaryOperator<HomDD, Target<Op, H, G>, Output = Target<Op, H, G>>
        + BinaryOperator<ComposeTriple<Op, H, G, Self>, HomAA, Output = ComposeTriple<Op, H, G, Self>>
        + BinaryOperator<HomDD, ComposeTriple<Op, H, G, Self>, Output = ComposeTriple<Op, H, G, Self>>
        + InternalBinaryOperator<HomAA>
        + InternalBinaryOperator<HomBB>
        + InternalBinaryOperator<HomCC>
        + InternalBinaryOperator<HomDD>
{
}

impl<Op, F, G, H, HomAA, HomBB, HomCC, HomDD> Category<Op, G, H, HomAA, HomBB, HomCC, HomDD> for F
where
    F: Semigroupoid<Op, G, H>,
    G: Clone + Morphism<Domain = Codomain<Self>>,
    H: Clone + Morphism<Domain = Codomain<G>>,
    Target<Op, G, Self>: Morphism<Domain = Domain<Self>, Codomain = Codomain<G>>,
    Target<Op, H, G>: Morphism<Domain = Domain<G>, Codomain = Codomain<H>>,
    ComposeTriple<Op, H, G, Self>: Morphism<Domain = Domain<Self>, Codomain = Codomain<H>>,
    ComposeTriple<Op, H, G, Self>: Sized + PartialEq + Clone,
    HomAA: Endomorphism<Object = Domain<Self>> + Identity<Op>,
    HomBB: Endomorphism<Object = Domain<G>> + Identity<Op>,
    HomCC: Endomorphism<Object = Domain<H>> + Identity<Op>,
    HomDD: Endomorphism<Object = Codomain<H>> + Identity<Op>,
    // from Associativity
    Op: BinaryOperator<G, Self>
        + BinaryOperator<H, G>
        + BinaryOperator<H, Target<Op, G, Self>>
        + BinaryOperator<Target<Op, H, G>, Self, Output = ComposeTriple<Op, H, G, Self>>,
    // from Category
    Op: BinaryOperator<Self, HomAA, Output = Self>
        + BinaryOperator<HomBB, Self, Output = Self>
        + BinaryOperator<G, HomBB, Output = G>
        + BinaryOperator<HomCC, G, Output = G>
        + BinaryOperator<H, HomCC, Output = H>
        + BinaryOperator<HomDD, H, Output = H>
        + BinaryOperator<Target<Op, G, Self>, HomAA, Output = Target<Op, G, Self>>
        + BinaryOperator<HomCC, Target<Op, G, Self>, Output = Target<Op, G, Self>>
        + BinaryOperator<Target<Op, H, G>, HomBB, Output = Target<Op, H, G>>
        + BinaryOperator<HomDD, Target<Op, H, G>, Output = Target<Op, H, G>>
        + BinaryOperator<ComposeTriple<Op, H, G, Self>, HomAA, Output = ComposeTriple<Op, H, G, Self>>
        + BinaryOperator<HomDD, ComposeTriple<Op, H, G, Self>, Output = ComposeTriple<Op, H, G, Self>>
        + InternalBinaryOperator<HomAA>
        + InternalBinaryOperator<HomBB>
        + InternalBinaryOperator<HomCC>
        + InternalBinaryOperator<HomDD>
{
}
