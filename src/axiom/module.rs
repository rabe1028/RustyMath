use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

/*
pub trait Module<Add, Mul>: AbelianGroup<Add>
where
    Add: InternalBinaryOperator<Self>
        + InternalBinaryOperator<<Self as Module<Add, Mul>>::CoefficientType>,
    Mul: ExternalBinaryOperator<<Self as Module<Add, Mul>>::CoefficientType, Self>
        + InternalBinaryOperator<<Self as Module<Add, Mul>>::CoefficientType>,
{
    // Coefficient Ring is Scalar
    type CoefficientType: UnitalRing<Add, Mul>;
}
*/

pub trait Module<Coeff, Add, Mul>: AbelianGroup<Add>
where
    Add: InternalBinaryOperator<Self> + InternalBinaryOperator<Coeff>,
    Mul: ExternalBinaryOperator<Coeff, Self> + InternalBinaryOperator<Coeff>,
    Coeff: UnitalRing<Add, Mul>,
{
}
