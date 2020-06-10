use crate::operator::*;

pub trait Totality<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}

pub trait Morphism {
    // Self : Domain -> Codomain
    // Ex, Int : () -> ()
    type Domain; // Source
    type Codomain; // Target
    fn _morphism(&self) {}
}

pub trait Endomorphism
where
    Self: Morphism<
        Domain = <Self as Endomorphism>::Object,
        Codomain = <Self as Endomorphism>::Object,
    >,
{
    type Object;
    fn _endomorphism(&self) {}
}

pub type Domain<A> = <A as Morphism>::Domain;
pub type Codomain<A> = <A as Morphism>::Codomain;

// f : c -> d, g : b -> c, h : c -> a
// f . g . h
// f = Self, g = MHs, h = Rhs
// Mhs = Middle Hand Side
// Rhs = Right Hand Side
pub trait Associativity<Op, Mhs, Rhs>
where
    Self: Sized + Morphism,
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
    fn check_associativity(x: Self, y: Mhs, z: Rhs) -> bool
    where
        Self: PartialEq + Clone,
        Mhs: PartialEq + Clone,
        Rhs: PartialEq + Clone,
        Target<Op, Self, Target<Op, Mhs, Rhs>>: Sized + PartialEq + Clone,
    {
        Op::operate(x.clone(), Op::operate(y.clone(), z.clone()))
            == Op::operate(Op::operate(x, y), z)
    }
}

pub trait LeftIdentity<Op, Rhs = Self>
where
    Self: Sized + Morphism,
    Rhs: Sized + Endomorphism<Object = Domain<Self>>,
    Op: BinaryOperator<Self, Rhs, Output = Self>,
{
    fn left_identity() -> Rhs;
    #[inline(always)]
    fn is_left_identity(other: &Rhs) -> bool
    where
        Self: PartialEq + Clone,
        Rhs: PartialEq + Clone,
    {
        *other == Self::left_identity()
    }
}

// impl<Op, T> LeftIdentity<Op> for T
// where
//     T: Identity<Op>,
//     Op: InternalBinaryOperator<T>,
// {
//     #[inline(always)]
//     fn left_identity() -> Self {
//         T::identity()
//     }
// }

pub trait RightIdentity<Op, Lhs = Self>
where
    Self: Sized + Morphism,
    Lhs: Sized + Endomorphism<Object = Codomain<Self>>,
    Op: BinaryOperator<Lhs, Self, Output = Self>,
{
    fn right_identity() -> Lhs;
    #[inline(always)]
    fn is_right_identity(other: &Lhs) -> bool
    where
        Self: PartialEq + Clone,
        Lhs: PartialEq + Clone,
    {
        *other == Self::right_identity()
    }
}

// impl<Op, T> RightIdentity<Op> for T
// where
//     T: Identity<Op>,
//     Op: InternalBinaryOperator<T>,
// {
//     #[inline(always)]
//     fn right_identity() -> T {
//         T::identity()
//     }
// }

// Identityの制約
// InternalBinaryOperatorが定義されているはず．
// Ex
// f(Self) * id_x = f
// id_y * f(Self) = f
pub trait Identity<Op>: LeftIdentity<Op, Self> + RightIdentity<Op, Self>
where
    Self: Sized + Endomorphism,
    Op: InternalBinaryOperator<Self>,
{
    fn identity() -> Self;
    #[inline(always)]
    fn is_identity(&self) -> bool
    where
        Self: PartialEq + Clone,
    {
        *self == Self::identity()
    }
    #[inline(always)]
    fn check_identity(x: Self) -> bool
    where
        Self: PartialEq + Clone,
    {
        let id = Self::identity();
        let left = Op::operate(x.clone(), id.clone());
        let right = Op::operate(id.clone(), x.clone());
        (left == x) && (right == x)
    }
}
pub trait Invertivility<T>
where
    Self: Sized + Clone + Morphism,
{
    type Inverse: Morphism<
        Domain = <Self as Morphism>::Codomain,
        Codomain = <Self as Morphism>::Domain,
    >;
    fn inverse(&self) -> Self::Inverse;
    // fn inverse(&self) -> Self {
    //     Self::identity().inv_op(self.clone())
    // }
    // fn inv_op(self, other: Self) -> Self {
    //     T::operate(self, other.inverse())
    // }
}

pub type Inverse<Op, A> = <A as Invertivility<Op>>::Inverse;

pub trait Commutativity<T>
where
    Self: Sized + PartialEq + Clone,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_commutativity(x: Self, y: Self) -> bool {
        T::operate(x.clone(), y.clone()) == T::operate(y, x)
    }
}

pub trait RightDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_right_distributivity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(Add::operate(x.clone(), y.clone()), z.clone())
            == Add::operate(
                Mul::operate(x.clone(), z.clone()),
                Mul::operate(y.clone(), z.clone()),
            )
    }
}

pub trait LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_left_distributivity(x: Self, y: Self, z: Self) -> bool {
        Mul::operate(x.clone(), Add::operate(y.clone(), z.clone()))
            == Add::operate(Mul::operate(x.clone(), y), Mul::operate(x.clone(), z))
    }
}

pub trait Distributivity<Add, Mul>:
    RightDistributivity<Add, Mul> + LeftDistributivity<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Mul: InternalBinaryOperator<Self>,
    Add: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_distributivity(x: Self, y: Self, z: Self) -> bool {
        Self::check_left_distributivity(x.clone(), y.clone(), z.clone())
            && Self::check_right_distributivity(x, y, z)
    }
}

impl<T, Add, Mul> Distributivity<Add, Mul> for T
where
    T: LeftDistributivity<Add, Mul> + RightDistributivity<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
}

pub trait Absorbency<Add, Mul>
where
    Self: Sized + PartialEq + Clone,
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_absorbency(x: Self, y: Self) -> bool {
        let add_xy = Add::operate(x.clone(), y.clone());
        let mul_xy = Mul::operate(x.clone(), y.clone());

        Mul::operate(x.clone(), add_xy) == x && Add::operate(x.clone(), mul_xy) == x
    }
}

pub trait Divisibility<Add, Mul>
where
    Self: Sized + PartialEq + Copy,
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
}

pub trait LeftCancellative<T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<Self>,
{
}
pub trait RightCancellative<T>
where
    Self: Sized + Clone,
    T: InternalBinaryOperator<Self>,
{
}
pub trait Cancellative<T>: LeftCancellative<T> + RightCancellative<T>
where
    Self: Sized,
    T: InternalBinaryOperator<Self>,
{
}

impl<Op, T> Cancellative<Op> for T
where
    T: LeftCancellative<Op> + RightCancellative<Op>,
    Op: InternalBinaryOperator<T>,
{
}

pub trait Mediality<T>
where
    Self: Sized + PartialEq + Copy,
    T: InternalBinaryOperator<Self>,
{
    #[inline(always)]
    fn check_mediality(a: Self, b: Self, c: Self, d: Self) -> bool {
        let ab = T::operate(a.clone(), b.clone());
        let cd = T::operate(c.clone(), d.clone());
        let ac = T::operate(a, c);
        let bd = T::operate(b, d);

        T::operate(ab, cd) == T::operate(ac, bd)
    }
}

pub trait NoZeroDivisor {}

pub trait IntegrallyClosed {}

pub trait UniqueFactorizable {}

pub trait UniquePrimeFactorizable: UniqueFactorizable {}
