use crate::axiom::*;
use crate::operator::*;
use crate::property::*;
use std::cmp::*;
use std::ops::*;

#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct Polynomial<Coeff>
where
    Coeff: Ring<Addition, Multiplication>,
    Addition: InternalBinaryOperator<Coeff>,
    Multiplication: InternalBinaryOperator<Coeff>,
{
    a: Vec<Coeff>,
}

impl<T> Morphism for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    type Domain = ();
    type Codomain = ();
}

impl<T> Endomorphism for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    type Object = ();
}

impl<T> Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    #[inline(always)]
    pub fn from_vec(a: Vec<T>) -> Self {
        if a.len() == 0 {
            let mut r = Self { a };
            r.a.push(T::zero());
            r
        } else {
            let mut r = Self { a };
            r.a.reverse();
            r
        }
    }

    #[inline(always)]
    pub fn new_monomial_with_degree(deg: usize) -> Self
    where
        T: Semiring<Addition, Multiplication>,
    {
        let mut v = vec![T::zero(); deg + 1];
        v[deg] = T::one();
        Self::from_vec(v)
    }

    #[inline(always)]
    pub fn reduce(mut self) -> Self {
        if self.a[self.degree()] == T::zero() && self.degree() > 0 {
            self.a.pop();
            self.reduce()
        } else {
            self
        }
    }

    #[inline(always)]
    pub fn degree(&self) -> usize {
        assert!(self.a.len() > 0);
        self.a.len() - 1
    }

    #[inline(always)]
    pub fn coeff(&self, i: usize) -> &T {
        &self.a[i]
    }

    #[inline(always)]
    pub fn leading_coefficient(&self) -> &T {
        &self.a[self.degree()]
    }

    #[inline(always)]
    pub fn is_monic(&self) -> bool
    where
        T: Monoid<Multiplication>,
    {
        self.a[self.degree() - 1] == T::one()
    }
}

macro_rules! forward_one_ref_binop {
    ($op:ty) => {
        impl<'a, T> BinaryOperator<&'a Polynomial<T>, Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn operate(lhs: &'a Polynomial<T>, rhs: Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(lhs, &rhs)
            }
        }

        impl<'a, T> BinaryOperator<Polynomial<T>, &'a Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn operate(lhs: Polynomial<T>, rhs: &'a Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(&lhs, rhs)
            }
        }

        impl<T> BinaryOperator<Polynomial<T>, Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn operate(lhs: Polynomial<T>, rhs: Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(&lhs, &rhs)
            }
        }
    };
}

macro_rules! forward_inter_binop {
    ($op:ty, ($l:ident, $r:ident) => $x: expr) => {
        impl<T> InternalBinaryOperator<Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
        }

        impl<'a, T> BinaryOperator<&'a Polynomial<T>, &'a Polynomial<T>> for $op
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;
            fn operate($l: &'a Polynomial<T>, $r: &'a Polynomial<T>) -> Self::Output {
                $x
            }
        }

        forward_one_ref_binop! {$op}
    };
}

macro_rules! impl_helper {
    (impl $type:ident  $(< $( $lt:tt ),+ >)? ) => {
        impl<T> $type $(< $( $lt ),+ >)? for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {}
    };
    (
        impl $type:ident $(< $( $lt:tt ),+ >)?,
        $(#[$attr:meta])* fn $name:ident $args:tt -> $ret:ty $body:block
    ) => {
        impl<T> $type $(< $( $lt ),+ >)? for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            $(#[$attr])* fn $name $args -> $ret $body
        }
    };
}

macro_rules! forward_assign {
    (impl $type:ident, fn $name:ident, $op:ty) => {
        impl<T> $type<Polynomial<T>> for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            fn $name(&mut self, other: Polynomial<T>) {
                *self = <$op as BinaryOperator<&Polynomial<T>, Polynomial<T>>>::operate(self, other)
            }
        }

        impl<'a, T> $type<&'a Polynomial<T>> for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            fn $name(&mut self, other: &'a Polynomial<T>) {
                *self =
                    <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(self, other)
            }
        }
    };
}

macro_rules! forward_binop_impl {
    (impl $type:ident, fn $name:ident, $op:ty) => {
        impl<T> $type<Polynomial<T>> for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn $name(self, rhs: Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(&self, &rhs)
            }
        }

        impl<'a, T> $type<Polynomial<T>> for &'a Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn $name(self, rhs: Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(self, &rhs)
            }
        }

        impl<'a, T> $type<&'a Polynomial<T>> for Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;

            #[inline]
            fn $name(self, rhs: &'a Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(&self, rhs)
            }
        }

        impl<'a, T> $type<&'a Polynomial<T>> for &'a Polynomial<T>
        where
            T: Ring<Addition, Multiplication> + Clone,
            Addition: InternalBinaryOperator<T>,
            Multiplication: InternalBinaryOperator<T>,
        {
            type Output = Polynomial<T>;
            fn $name(self, rhs: &'a Polynomial<T>) -> Self::Output {
                <$op as BinaryOperator<&Polynomial<T>, &Polynomial<T>>>::operate(self, rhs)
            }
        }
    };
}

forward_inter_binop! { Addition,
    (lhs, rhs) => {
        use std::cmp::min;
        let min_size = min(lhs.a.len(), rhs.a.len());

        let (mut lhs, rhs) = if lhs.degree() < rhs.degree() {
            (rhs.clone(), &lhs)
        } else {
            (lhs.clone(), &rhs)
        };

        for i in 0..min_size {
            lhs.a[i] = lhs.a[i].clone().add(rhs.a[i].clone())
        }

        lhs.reduce()
    }
}

forward_binop_impl! {impl Add, fn add, Addition}

forward_assign! {impl AddAssign, fn add_assign, Addition}

impl_helper! {impl Totality<Addition>}
impl_helper! {impl Associativity<Addition, Self, Self>}
impl_helper! {impl Commutativity<Addition>}

// impl_helper! {impl LeftIdentity<Addition>,
//     #[inline(always)]
//     fn left_identity() -> Self {
//         Self::identity()
//     }
// }

// impl_helper! {impl RightIdentity<Addition>,
//     #[inline(always)]
//     fn right_identity() -> Self {
//         Self::identity()
//     }
// }

impl_helper! {impl Identity<Addition>,
    #[inline(always)]
    fn identity() -> Self {
        Polynomial { a: vec![T::zero()] }
    }
}

impl<T> Invertivility<Addition> for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    type Inverse = Self;
    #[inline(always)]
    fn inverse(&self) -> Self {
        Polynomial {
            a: self.a.iter().map(|i| i.inverse()).collect()
        }
    }
}

// impl_helper! {impl Invertivility<Addition>,
//     #[inline(always)]
//     fn inverse(&self) -> Self {
//         Polynomial {
//             a: self.a.iter().map(|i| i.inverse()).collect()
//         }
//     }
// }

//  Convolution like Mul
forward_inter_binop! { Multiplication,
    (lhs, rhs) => {
        let out_len = lhs.a.len() + rhs.a.len();
        let mut a = vec![T::identity(); out_len];

        for l in 0..lhs.a.len() {
            for r in 0..rhs.a.len() {
                a[l+r] = Addition::operate(
                    Multiplication::operate(
                        lhs.a[l].clone(), rhs.a[r].clone()
                    ),
                    a[l+r].clone(),
                )
            }
        }

        Polynomial {a}.reduce()
    }
}

forward_binop_impl! {impl Mul, fn mul, Multiplication}

forward_assign! {impl MulAssign, fn mul_assign, Multiplication}

impl_helper! {impl Totality<Multiplication>}
impl_helper! {impl Associativity<Multiplication, Self, Self>}

// impl_helper! {impl Identity<Multiplication>,
//     #[inline(always)]
//     fn identity() -> Self {
//         Polynomial {
//             a: vec![ T::one() ]
//         }
//     }
// }

// impl<T> LeftIdentity<Multiplication> for Polynomial<T>
// where
//     T: Ring<Addition, Multiplication> + Monoid<Multiplication> + Clone,
//     Addition: InternalBinaryOperator<T>,
//     Multiplication: InternalBinaryOperator<T>,
// {
//     #[inline(always)]
//     fn left_identity() -> Self {
//         Polynomial { a: vec![T::one()] }
//     }
// }

// impl<T> RightIdentity<Multiplication> for Polynomial<T>
// where
//     T: Ring<Addition, Multiplication> + Monoid<Multiplication> + Clone,
//     Addition: InternalBinaryOperator<T>,
//     Multiplication: InternalBinaryOperator<T>,
// {
//     #[inline(always)]
//     fn right_identity() -> Self {
//         Polynomial { a: vec![T::one()] }
//     }
// }

impl<T> Identity<Multiplication> for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Monoid<Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    #[inline(always)]
    fn identity() -> Self {
        Polynomial { a: vec![T::one()] }
    }
}

// impl_helper! {impl Commutativity<Multiplication>}

impl<T> Commutativity<Multiplication> for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Commutativity<Multiplication> + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
}

impl_helper! {impl RightDistributivity<Addition, Multiplication>}
impl_helper! {impl LeftDistributivity<Addition, Multiplication>}

// 多項式に距離を定義
// 距離は整列順序集合になるから，
// それを比較する

impl<T> Ord for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Eq + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.degree().cmp(&other.degree())
    }
}

impl<T> PartialOrd for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Eq + Clone,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> std::fmt::Debug for Polynomial<T>
where
    T: Ring<Addition, Multiplication> + Eq + Clone + std::fmt::Debug,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "f(x) = ");
        for i in (0..=self.degree()).rev() {
            if i == 0 {
                write!(f, "{:?} ", self.a[i]);
            } else {
                write!(f, "{:?} x^{:?} + ", self.a[i], i);
            }
        }
        Ok(())
    }
}

impl_helper! {impl NoZeroDivisor}
impl_helper! {impl IntegrallyClosed}
impl_helper! {impl UniqueFactorizable}
impl_helper! {impl UniquePrimeFactorizable}

impl<T> Ring<Addition, Multiplication> for Polynomial<T>
where
    T: EuclidianDomain<Addition, Multiplication> + Eq + Clone + std::fmt::Debug,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn sub(self, other: Self) -> Self {
        let inv: Self = other.inverse();
        <Self as Ring<_, _>>::add(self, inv)
    }
}

impl<T> EuclidianDomain<Addition, Multiplication> for Polynomial<T>
where
    T: EuclidianDomain<Addition, Multiplication> + Eq + Clone + std::fmt::Debug,
    Addition: InternalBinaryOperator<T>,
    Multiplication: InternalBinaryOperator<T>,
{
    fn div(&self, other: &Self) -> Self {
        self.divrem(other).0
    }

    fn rem(&self, other: &Self) -> Self {
        self.divrem(other).1
    }

    fn divrem(&self, other: &Self) -> (Self, Self) {
        // other.leading_coefficient() != 0 <==> other == 0
        assert!(!other.is_zero());

        let mut r = self.clone();
        let mut q = Polynomial::from_vec(vec![T::zero(); r.degree() - other.degree() + 1]);

        // println!("degree of q : {:?}", q.degree());
        for i in 0..=q.degree() {
            // qの上の次数から探索する
            let q_ind = q.degree() - i;
            // r.degree() - i はrの最高次数 - i
            q.a[q_ind] = r.a[r.degree() - i].clone().div(other.leading_coefficient());
            let t = q.a[q_ind].clone();

            // println!("{:?} / {:?} : Coeff t = {:?} \n
            //     lc(r) = {:?}, lc(g) = {:?}",
            //     self, other, t,
            //     r.a[r.degree() - i], other.leading_coefficient()
            // );

            // r <- r - q.a[ind]* x^(deg(r) - deg(other)) * other
            for k in 0..=other.degree() {
                let r_ind = r.degree() - i - k;
                r.a[r_ind] = r.a[r_ind]
                    .clone()
                    .sub(other.a[other.degree() - k].clone().mul(t.clone()))
            }
        }

        (q.reduce(), r.reduce())
    }
}

#[cfg(test)]
mod tests {
    use crate::set::polynomial::*;

    #[test]
    fn construct() {
        let _ = Polynomial::from_vec(vec![1, 0]);
    }

    #[test]
    fn add() {
        let a = Polynomial::from_vec(vec![1, 2, 1]);
        let b = Polynomial::from_vec(vec![1, 1]);

        assert_eq!(a + b, Polynomial::from_vec(vec![1, 3, 2]));
    }

    #[test]
    fn mul() {
        let a = Polynomial::from_vec(vec![1, 0]);
        let b = Polynomial::from_vec(vec![1, 1]);
        assert_eq!(a * b, Polynomial::from_vec(vec![1, 1, 0]))
    }

    #[test]
    fn divmod() {
        let a = Polynomial::from_vec(vec![1, 2, 1]);
        let b = Polynomial::from_vec(vec![1, 1]);

        assert_eq!(&b * &b, a);

        assert_eq!(a.div(&b), b);

        let d = Polynomial::from_vec(vec![3, 2, 1]);
        let c = &a * &d + &b;

        assert_eq!(c, Polynomial::from_vec(vec![3, 8, 8, 5, 2]));

        let (c1, c2) = c.divrem(&a);
        assert_eq!((c1.clone(), c2.clone()), (d, b));
        assert_eq!(c1 * &a + c2, Polynomial::from_vec(vec![3, 8, 8, 5, 2]));
    }
}
