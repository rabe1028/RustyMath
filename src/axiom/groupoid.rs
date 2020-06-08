use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

type ComposeTriple<Op, A, B, C> = Target<Op, A, Target<Op, B, C>>;

pub trait Groupoid<Op, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD>:
    Category<Op, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD> + Invertivility<Op>
where
    Mhs: Sized + Morphism<Codomain = Domain<Self>> + Invertivility<Op>,
    Rhs: Sized + Morphism<Codomain = Domain<Mhs>> + Invertivility<Op>,
    Target<Op, Self, Mhs>:
        Sized + Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>> + Invertivility<Op>,
    Target<Op, Mhs, Rhs>:
        Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>> + Invertivility<Op>,
    ComposeTriple<Op, Self, Mhs, Rhs>:
        Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>> + Invertivility<Op>,
    HomAA: Sized
        + Endomorphism<Object = Domain<Rhs>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomAA>,
    HomBB: Sized
        + Endomorphism<Object = Domain<Mhs>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomBB>,
    HomCC: Sized
        + Endomorphism<Object = Domain<Self>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomCC>,
    HomDD: Sized
        + Endomorphism<Object = Codomain<Self>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomDD>,
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
    // from groupoid
    Op: BinaryOperator<Inverse<Op, Rhs>, Rhs, Output = HomAA>
        + BinaryOperator<Rhs, Inverse<Op, Rhs>, Output = HomBB>
        + BinaryOperator<Inverse<Op, Mhs>, Mhs, Output = HomBB>
        + BinaryOperator<Mhs, Inverse<Op, Mhs>, Output = HomCC>
        + BinaryOperator<Inverse<Op, Self>, Self, Output = HomCC>
        + BinaryOperator<Self, Inverse<Op, Self>, Output = HomDD>
        + BinaryOperator<Inverse<Op, Target<Op, Mhs, Rhs>>, Target<Op, Mhs, Rhs>, Output = HomAA>
        + BinaryOperator<Target<Op, Mhs, Rhs>, Inverse<Op, Target<Op, Mhs, Rhs>>, Output = HomCC>
        + BinaryOperator<Inverse<Op, Target<Op, Self, Mhs>>, Target<Op, Self, Mhs>, Output = HomBB>
        + BinaryOperator<Target<Op, Self, Mhs>, Inverse<Op, Target<Op, Self, Mhs>>, Output = HomDD>
        + BinaryOperator<
            Inverse<Op, ComposeTriple<Op, Self, Mhs, Rhs>>,
            ComposeTriple<Op, Self, Mhs, Rhs>,
            Output = HomAA,
        > + BinaryOperator<
            ComposeTriple<Op, Self, Mhs, Rhs>,
            Inverse<Op, ComposeTriple<Op, Self, Mhs, Rhs>>,
            Output = HomDD,
        >,
{
}

impl<Op, Lhs, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD>
    Groupoid<Op, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD> for Lhs
where
    Lhs: Category<Op, Mhs, Rhs, HomAA, HomBB, HomCC, HomDD> + Invertivility<Op>,
    Mhs: Sized + Morphism<Codomain = Domain<Self>> + Invertivility<Op>,
    Rhs: Sized + Morphism<Codomain = Domain<Mhs>> + Invertivility<Op>,
    Target<Op, Self, Mhs>:
        Sized + Morphism<Domain = Domain<Mhs>, Codomain = Codomain<Self>> + Invertivility<Op>,
    Target<Op, Mhs, Rhs>:
        Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Mhs>> + Invertivility<Op>,
    ComposeTriple<Op, Self, Mhs, Rhs>:
        Sized + Morphism<Domain = Domain<Rhs>, Codomain = Codomain<Self>> + Invertivility<Op>,
    HomAA: Sized
        + Endomorphism<Object = Domain<Rhs>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomAA>,
    HomBB: Sized
        + Endomorphism<Object = Domain<Mhs>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomBB>,
    HomCC: Sized
        + Endomorphism<Object = Domain<Self>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomCC>,
    HomDD: Sized
        + Endomorphism<Object = Codomain<Self>>
        + Identity<Op>
        + Invertivility<Op, Inverse = HomDD>,
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
    // from groupoid
    Op: BinaryOperator<Inverse<Op, Rhs>, Rhs, Output = HomAA>
        + BinaryOperator<Rhs, Inverse<Op, Rhs>, Output = HomBB>
        + BinaryOperator<Inverse<Op, Mhs>, Mhs, Output = HomBB>
        + BinaryOperator<Mhs, Inverse<Op, Mhs>, Output = HomCC>
        + BinaryOperator<Inverse<Op, Self>, Self, Output = HomCC>
        + BinaryOperator<Self, Inverse<Op, Self>, Output = HomDD>
        + BinaryOperator<Inverse<Op, Target<Op, Mhs, Rhs>>, Target<Op, Mhs, Rhs>, Output = HomAA>
        + BinaryOperator<Target<Op, Mhs, Rhs>, Inverse<Op, Target<Op, Mhs, Rhs>>, Output = HomCC>
        + BinaryOperator<Inverse<Op, Target<Op, Self, Mhs>>, Target<Op, Self, Mhs>, Output = HomBB>
        + BinaryOperator<Target<Op, Self, Mhs>, Inverse<Op, Target<Op, Self, Mhs>>, Output = HomDD>
        + BinaryOperator<
            Inverse<Op, ComposeTriple<Op, Self, Mhs, Rhs>>,
            ComposeTriple<Op, Self, Mhs, Rhs>,
            Output = HomAA,
        > + BinaryOperator<
            ComposeTriple<Op, Self, Mhs, Rhs>,
            Inverse<Op, ComposeTriple<Op, Self, Mhs, Rhs>>,
            Output = HomDD,
        >,
{
}
