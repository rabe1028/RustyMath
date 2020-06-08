use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

type ComposeTriple<Op, A, B, C> = Target<Op, A, Target<Op, B, C>>;

// Hom(x, x) : x is Object  has identity
pub trait Category<Op, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD>: Semigroupoid<Op, Mhs, Rhs>
// + Identity<Op>
where
    Mhs: Clone + Morphism<Codomain = Domain<Self>>,
    Rhs: Clone + Morphism<Codomain = Domain<Mhs>>,
    Target<Op, Self, Mhs>: Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>>,
    Target<Op, Mhs, Rhs>: Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>>,
    ComposeTriple<Op, Self, Mhs, Rhs>: Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    ComposeTriple<Op, Self, Mhs, Rhs>: Sized + PartialEq + Clone,
    HomAA: Endomorphism<Object = Domain<Rhs>> + Identity<Op>,
    HomBB: Endomorphism<Object = Domain<Mhs>> + Identity<Op>,
    HomCC: Endomorphism<Object = Domain<Self>> + Identity<Op>,
    HomDD: Endomorphism<Object = Codomain<Self>> + Identity<Op>,
    // from Associativity
    Op: BinaryOperator<Self, Mhs>
        + BinaryOperator<Mhs, Rhs>
        + BinaryOperator<Self, Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, Rhs, Output = Target<Op, Self, Target<Op, Mhs, Rhs>>>,
    // from Category
    Op: BinaryOperator<Rhs, HomAA, Output = Rhs>
        + BinaryOperator<HomBB, Rhs, Output = Rhs>
        + BinaryOperator<Mhs, HomBB, Output = Mhs>
        + BinaryOperator<HomCC, Mhs, Output = Mhs>
        + BinaryOperator<Self, HomCC, Output = Self>
        + BinaryOperator<HomDD, Self, Output = Self>
        + BinaryOperator<Target<Op, Mhs, Rhs>, HomAA, Output = Target<Op, Mhs, Rhs>>
        + BinaryOperator<HomCC, Target<Op, Mhs, Rhs>, Output = Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, HomBB, Output = Target<Op, Self, Mhs>>
        + BinaryOperator<HomDD, Target<Op, Self, Mhs>, Output = Target<Op, Self, Mhs>>
        + BinaryOperator<
            ComposeTriple<Op, Self, Mhs, Rhs>,
            HomAA,
            Output = ComposeTriple<Op, Self, Mhs, Rhs>,
        > + BinaryOperator<
            HomDD,
            ComposeTriple<Op, Self, Mhs, Rhs>,
            Output = ComposeTriple<Op, Self, Mhs, Rhs>,
        > + InternalBinaryOperator<HomAA>
        + InternalBinaryOperator<HomBB>
        + InternalBinaryOperator<HomCC>
        + InternalBinaryOperator<HomDD>,
{
}

impl<Op, Lhs, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD>
    Category<Op, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD> for Lhs
where
    Lhs: Semigroupoid<Op, Mhs, Rhs>,
    Mhs: Clone + Morphism<Codomain = Domain<Self>>,
    Rhs: Clone + Morphism<Codomain = Domain<Mhs>>,
    Target<Op, Self, Mhs>: Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>>,
    Target<Op, Mhs, Rhs>: Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>>,
    ComposeTriple<Op, Self, Mhs, Rhs>: Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>>,
    ComposeTriple<Op, Self, Mhs, Rhs>: Sized + PartialEq + Clone,
    HomAA: Endomorphism<Object = Domain<Rhs>> + Identity<Op>,
    HomBB: Endomorphism<Object = Domain<Mhs>> + Identity<Op>,
    HomCC: Endomorphism<Object = Domain<Self>> + Identity<Op>,
    HomDD: Endomorphism<Object = Codomain<Self>> + Identity<Op>,
    // from Associativity
    Op: BinaryOperator<Self, Mhs>
        + BinaryOperator<Mhs, Rhs>
        + BinaryOperator<Self, Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, Rhs, Output = Target<Op, Self, Target<Op, Mhs, Rhs>>>,
    // from Category
    Op: BinaryOperator<Rhs, HomAA, Output = Rhs>
        + BinaryOperator<HomBB, Rhs, Output = Rhs>
        + BinaryOperator<Mhs, HomBB, Output = Mhs>
        + BinaryOperator<HomCC, Mhs, Output = Mhs>
        + BinaryOperator<Self, HomCC, Output = Self>
        + BinaryOperator<HomDD, Self, Output = Self>
        + BinaryOperator<Target<Op, Mhs, Rhs>, HomAA, Output = Target<Op, Mhs, Rhs>>
        + BinaryOperator<HomCC, Target<Op, Mhs, Rhs>, Output = Target<Op, Mhs, Rhs>>
        + BinaryOperator<Target<Op, Self, Mhs>, HomBB, Output = Target<Op, Self, Mhs>>
        + BinaryOperator<HomDD, Target<Op, Self, Mhs>, Output = Target<Op, Self, Mhs>>
        + BinaryOperator<
            ComposeTriple<Op, Self, Mhs, Rhs>,
            HomAA,
            Output = ComposeTriple<Op, Self, Mhs, Rhs>,
        > + BinaryOperator<
            HomDD,
            ComposeTriple<Op, Self, Mhs, Rhs>,
            Output = ComposeTriple<Op, Self, Mhs, Rhs>,
        > + InternalBinaryOperator<HomAA>
        + InternalBinaryOperator<HomBB>
        + InternalBinaryOperator<HomCC>
        + InternalBinaryOperator<HomDD>,
{
}
