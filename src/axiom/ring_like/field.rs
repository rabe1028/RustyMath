use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

pub trait Field<'a, Add, Mul>: AbelianGroup<'a, Mul> + EuclidianDomain<'a, Add, Mul>
where
    Add: InternalBinaryOperator<'a, Self>,
    Mul: InternalBinaryOperator<'a, Self>,
{
    #[inline(always)]
    fn reciprocal(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(<Self as Invertivility<'a, Mul>>::inverse(self))
        }
    }
}

impl<'a, Add, Mul, T> Field<'a, Add, Mul> for T
where
    T: AbelianGroup<'a, Mul> + EuclidianDomain<'a, Add, Mul>,
    Add: InternalBinaryOperator<'a,T>,
    Mul: InternalBinaryOperator<'a,T>,
{
}
