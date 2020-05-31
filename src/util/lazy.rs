use crate::operator::*;
use std::marker::PhantomData;

pub trait LazyOperation {
    type Output;
    fn eval(self) -> Self::Output;
}

pub trait InputSanitizer {
    type InputShape;
    fn sanitize(self) -> Self::InputShape;
}

pub type Sanitize<A> = <A as InputSanitizer>::InputShape;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    lhs: Left,
    rhs: Right,
    _op: PhantomData<Op>,
}

impl<Op, Left, Right> LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    pub fn new(lhs: Left, rhs: Right) -> Self {
        LazyBinaryOperation {
            lhs,
            rhs,
            _op: PhantomData,
        }
    }
}

impl<'a, Op, Left, Right> LazyOperation for LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    type Output = <Op as BinaryOperator<Sanitize<Left>, Sanitize<Right>>>::Output;
    fn eval(self) -> Self::Output {
        Op::operate(self.lhs.sanitize(), self.rhs.sanitize())
    }
}

impl<Op, Left, Right> InputSanitizer for LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
{
    type InputShape = <Op as BinaryOperator<Sanitize<Left>, Sanitize<Right>>>::Output;
    fn sanitize(self) -> Self::InputShape {
        self.eval()
    }
}

impl<Op, Left, Right, Rhs> std::ops::Add<Rhs> for LazyBinaryOperation<Op, Left, Right>
where
    Left: InputSanitizer,
    Right: InputSanitizer,
    Op: BinaryOperator<Sanitize<Left>, Sanitize<Right>>,
    Self: InputSanitizer,                    //Left
    Rhs: std::clone::Clone + InputSanitizer, //Right
    Addition: std::clone::Clone + BinaryOperator<Sanitize<Self>, Sanitize<Rhs>>,
{
    type Output = LazyBinaryOperation<Addition, Self, Rhs>;
    fn add(self, other: Rhs) -> Self::Output {
        LazyBinaryOperation::new(self, other)
    }
}
