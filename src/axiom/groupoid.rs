use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

type ComposeTriple<Op, A, B, C> = Target<Op, A, Target<Op, B, C>>;

pub trait Groupoid<Op, G, H, HomAA, HomBB, HomCC, HomDD>:
    Category<Op, G, H, HomAA, HomBB, HomCC, HomDD> + Invertivility<Op>
// ab
where
    G: Clone + Morphism<Domain = Codomain<Self>> + Invertivility<Op>, //bc
    H: Clone + Morphism<Domain = Codomain<G>> + Invertivility<Op>,    //cd
    Target<Op, G, Self>:
        Morphism<Domain = Domain<Self>, Codomain = Codomain<G>> + Invertivility<Op>,
    Target<Op, H, G>: Morphism<Domain = Domain<G>, Codomain = Codomain<H>> + Invertivility<Op>,
    ComposeTriple<Op, H, G, Self>:
        Morphism<Domain = Domain<Self>, Codomain = Codomain<H>> + Invertivility<Op>,
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
        + InternalBinaryOperator<HomDD>,
    // from groupoid
    Op: BinaryOperator<Inverse<Op, Self>, Self, Output = HomAA>
        + BinaryOperator<Self, Inverse<Op, Self>, Output = HomBB>
        + BinaryOperator<Inverse<Op, G>, Self, Output = HomBB>
        + BinaryOperator<G, Inverse<Op, G>, Output = HomCC>
        + BinaryOperator<Inverse<Op, H>, H, Output = HomCC>
        + BinaryOperator<H, Inverse<Op, H>, Output = HomDD>
        + BinaryOperator<Inverse<Op, Target<Op, G, Self>>, Target<Op, G, Self>, Output = HomAA>
        + BinaryOperator<Target<Op, G, Self>, Inverse<Op, Target<Op, G, Self>>, Output = HomCC>
        + BinaryOperator<Inverse<Op, Target<Op, H, G>>, Target<Op, H, G>, Output = HomBB>
        + BinaryOperator<Target<Op, H, G>, Inverse<Op, Target<Op, H, G>>, Output = HomDD>
        + BinaryOperator<
            Inverse<Op, ComposeTriple<Op, H, G, Self>>,
            ComposeTriple<Op, H, G, Self>,
            Output = HomAA,
        > + BinaryOperator<
            ComposeTriple<Op, H, G, Self>,
            Inverse<Op, ComposeTriple<Op, H, G, Self>>,
            Output = HomDD,
        >,
{
}

impl<Op, F, G, H, HomAA, HomBB, HomCC, HomDD> Groupoid<Op, G, H, HomAA, HomBB, HomCC, HomDD> for F
where
    F: Category<Op, G, H, HomAA, HomBB, HomCC, HomDD> + Invertivility<Op>, //ab
    G: Clone + Morphism<Domain = Codomain<Self>> + Invertivility<Op>,      //bc
    H: Clone + Morphism<Domain = Codomain<G>> + Invertivility<Op>,         //cd
    Target<Op, G, Self>:
        Morphism<Domain = Domain<Self>, Codomain = Codomain<G>> + Invertivility<Op>,
    Target<Op, H, G>: Morphism<Domain = Domain<G>, Codomain = Codomain<H>> + Invertivility<Op>,
    ComposeTriple<Op, H, G, Self>:
        Morphism<Domain = Domain<Self>, Codomain = Codomain<H>> + Invertivility<Op>,
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
        + InternalBinaryOperator<HomDD>,
    // from groupoid
    Op: BinaryOperator<Inverse<Op, Self>, Self, Output = HomAA>
        + BinaryOperator<Self, Inverse<Op, Self>, Output = HomBB>
        + BinaryOperator<Inverse<Op, G>, Self, Output = HomBB>
        + BinaryOperator<G, Inverse<Op, G>, Output = HomCC>
        + BinaryOperator<Inverse<Op, H>, H, Output = HomCC>
        + BinaryOperator<H, Inverse<Op, H>, Output = HomDD>
        + BinaryOperator<Inverse<Op, Target<Op, G, Self>>, Target<Op, G, Self>, Output = HomAA>
        + BinaryOperator<Target<Op, G, Self>, Inverse<Op, Target<Op, G, Self>>, Output = HomCC>
        + BinaryOperator<Inverse<Op, Target<Op, H, G>>, Target<Op, H, G>, Output = HomBB>
        + BinaryOperator<Target<Op, H, G>, Inverse<Op, Target<Op, H, G>>, Output = HomDD>
        + BinaryOperator<
            Inverse<Op, ComposeTriple<Op, H, G, Self>>,
            ComposeTriple<Op, H, G, Self>,
            Output = HomAA,
        > + BinaryOperator<
            ComposeTriple<Op, H, G, Self>,
            Inverse<Op, ComposeTriple<Op, H, G, Self>>,
            Output = HomDD,
        >,
{
}
