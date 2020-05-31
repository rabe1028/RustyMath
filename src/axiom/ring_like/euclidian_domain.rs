use crate::axiom::*;
use crate::operator::*;


pub trait EuclidianDomain<Add, Mul>: PrincipalIdealDomain<Add, Mul> + std::cmp::PartialOrd
// + AbelianGroup<GreatestCommonDivisor<Add, Mul>>
where
    Add: InternalBinaryOperator<Self>,
    Mul: InternalBinaryOperator<Self>,
{
    fn div(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self {
        let t = self.div(other).mul(other.clone());
        self.clone().sub(t)
    }
    fn divrem(&self, other: &Self) -> (Self, Self) {
        (self.div(other), self.rem(other))
    }
}

impl<Add, Mul, T> BinaryOperator<T, T> for GreatestCommonDivisor<Add, Mul>
where
    T: EuclidianDomain<Add, Mul>,
    Add: InternalBinaryOperator<T>,
    Mul: InternalBinaryOperator<T>,
{
    type Output = T;
    fn operate(lhs: T, rhs: T) -> Self::Output {
        if lhs >= rhs {
            return GreatestCommonDivisor::operate(rhs, lhs);
        }
        if rhs.is_zero() {
            rhs
        } else {
            let r = lhs.rem(&rhs);
            GreatestCommonDivisor::operate(rhs, r)
        }
    }
}

// impl<Add, Mul, T> EuclidianDomain<Add, Mul> for T
// where
//     T: PrincipalIdealDomain<Add, Mul> + AbelianGroup<GreatestCommonDivisor>,
//     Add: InternalBinaryOperator<T>,
//     Mul: InternalBinaryOperator<T>,
//     GreatestCommonDivisor: InternalBinaryOperator<T>,
// {
// }
