use crate::axiom::*;
use crate::operator::*;
use std::ops::*;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Polynomial<Coeff>
where
    Coeff: Ring<Addition, Multiplication>,
    Addition: InternalBinaryOperator<Coeff>,
    Multiplication: InternalBinaryOperator<Coeff>,
{
    a: Vec<Coeff>,
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

forward_inter_binop! { Addition,
    (lhs, rhs) => {
        use std::cmp::min;
        let min_size = min(lhs.a.len(), rhs.a.len());

        let a = (0..min_size).map(|i|
                Addition::operate(lhs.a[i].clone(),rhs.a[i].clone())
            ).collect();

        Polynomial {a}
    }
}

forward_assign! {impl AddAssign, fn add_assign, Addition}

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

        Polynomial {a}
    }
}

forward_assign! {impl MulAssign, fn mul_assign, Multiplication}
