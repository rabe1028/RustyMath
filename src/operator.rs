pub trait BinaryOperator<A, B> {
    type Output;
    fn operate(lhs: A, rhs: B) -> Self::Output;
}

// for copyable element(primitive types)
#[macro_export]
macro_rules! forward_ref_binop {
    ($op:ty,$lhs:ty, $rhs:ty,$out:ty) => {
        impl<'a> BinaryOperator<&'a $lhs, $rhs> for $op {
            type Output = <$op as BinaryOperator<$lhs, $rhs>>::Output;

            #[inline]
            fn operate(lhs: &'a $lhs, rhs: $rhs) -> Self::Output {
                <$op as BinaryOperator<$lhs, $rhs>>::operate(*lhs, rhs)
            }
        }

        impl<'a> BinaryOperator<$lhs, &'a $rhs> for $op {
            type Output = <$op as BinaryOperator<$lhs, $rhs>>::Output;

            #[inline]
            fn operate(lhs: $lhs, rhs: &'a $rhs) -> Self::Output {
                <$op as BinaryOperator<$lhs, $rhs>>::operate(lhs, *rhs)
            }
        }

        impl<'a> BinaryOperator<&'a $lhs, &'a $rhs> for $op {
            type Output = <$op as BinaryOperator<$lhs, $rhs>>::Output;

            #[inline]
            fn operate(lhs: &'a $lhs, rhs: &'a $rhs) -> Self::Output {
                <$op as BinaryOperator<$lhs, $rhs>>::operate(*lhs, *rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! forward_binop {
    ($op:ty,$lhs:ty, $rhs:ty,$out:ty, ($l:ident, $r:ident) => $x: expr) => {
        impl BinaryOperator<$lhs, $rhs> for $op {
            type Output = $out;
            fn operate($l: $lhs, $r: $rhs) -> Self::Output {
                $x
            }
        }

        forward_ref_binop! {$op, $lhs, $rhs, $out}
    };
}

pub trait InternalBinaryOperator<T>: BinaryOperator<T, T, Output = T> {}

// impl<Op, T> InternalBinaryOperator<T> for Op where Op: BinaryOperator<T, T, Output = T> {}

#[macro_export]
macro_rules! forward_internal_binop {
    ($op:ty, $t:ty,  ($lhs:ident, $rhs:ident) => $x: expr) => {
        forward_binop! {$op, $t, $t, $t, ($lhs, $rhs) => $x}
    };
    (
        $op:ty,
        $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)?
        $(where $( $wlt:tt : $wclt:tt $(+ $wdlt:tt )* ),+ )?
        ,
        ($lhs:ident, $rhs:ident) => $x: expr
    ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?
            BinaryOperator<
                $name $(< $( $lt ),+ >)? ,
                $name $(< $( $lt ),+ >)? ,
            >
        for $op
        $(where $($wlt: $wclt $(+ $wdlt)*),+)?
        {
            type Output = $name $(< $( $lt ),+ >)? ;
            fn operate($l: $name $(< $( $lt ),+ >)?, $r: $name $(< $( $lt ),+ >)?) -> Self::Output {
                $x
            }
        }
    };
}

pub trait ExternalBinaryOperator<S, T>: BinaryOperator<S, T, Output = T> {}

impl<Op, S, T> ExternalBinaryOperator<S, T> for Op where Op: BinaryOperator<S, T, Output = T> {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Addition {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Multiplication {}

// for vector operation
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InnerProduct {}

// for hadamard product operation
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HadamardProduct {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GreatestCommonDivisor<Add, Mul> {
    Add(std::marker::PhantomData<Add>),
    Mul(std::marker::PhantomData<Mul>),
}
