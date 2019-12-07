use crate::axiom::*;
use crate::operator::*;
use crate::property::*;

impl BinaryOperator<Vec<isize>> for Addition {
    fn operate(lhs: Vec<isize>, rhs: Vec<isize>) -> Vec<isize> {
        lhs
    }
}
impl Totality<Addition> for Vec<isize> {}