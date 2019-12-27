use crate::axiom::*;
use crate::operator::*;

/*
pub trait VectorSpace<Add, Mul>: Module<Add, Mul>
where
    Add: InternalBinaryOperator<Self>
        + InternalBinaryOperator<<Self as Module<Add, Mul>>::CoefficientType>
        + InternalBinaryOperator<<Self as VectorSpace<Add, Mul>>::CoefficientType>,
    Mul: ExternalBinaryOperator<<Self as Module<Add, Mul>>::CoefficientType, Self>
        + InternalBinaryOperator<<Self as Module<Add, Mul>>::CoefficientType>
        + ExternalBinaryOperator<<Self as VectorSpace<Add, Mul>>::CoeffcientType, Self>
        + InternalBinaryOperator<<Self as VectorSpace<Add, Mul>>::CoefficneryType>,
{
    // Coefficient Field
    type CoefficientType: Field<Add, Mul> = <Self as Module<Add, Mul>>::CoefficientType;
}
*/

/*
設計変更
CoefficientTypeを関連型にすると，RealVectorSpaceとComplexVectorSpaceの区別ができなくなる
Add,Mulの依存が多すぎ
*/

pub trait VectorSpace<Coeff, Add, Mul>: Module<Coeff, Add, Mul>
where
    Add: InternalBinaryOperator<Self> + InternalBinaryOperator<Coeff>,
    Mul: ExternalBinaryOperator<Coeff, Self> + InternalBinaryOperator<Coeff>,
    Coeff: Field<Add, Mul>,
{
}
