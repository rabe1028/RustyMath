use std::ops::Add;

use frunk::hlist::HList;
use frunk::*;
use typenum::uint::Unsigned;
use typenum::*;

pub trait TypeLength {
    type Length: Unsigned; // HList length (type int)
}

impl TypeLength for HNil {
    type Length = U0;
}

impl<Head, Tail> TypeLength for HCons<Head, Tail>
where
    Head: Unsigned,
    Tail: HList + TypeLength,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Length = <<Tail as TypeLength>::Length as Add<U1>>::Output;
}

pub trait IndexShape {
    // all usize hlist
    type Shape: HList;
    //type Capacity;

    fn get_index(index: Self::Shape) -> (usize, usize);
    fn get_capacity() -> usize;
}

impl IndexShape for HNil {
    type Shape = HNil;
    //type Capacity = U1;

    fn get_index(_: Self::Shape) -> (usize, usize) {
        (0, 1)
    }

    fn get_capacity() -> usize {
        1
    }
}

impl<Head, Tail> IndexShape for HCons<Head, Tail>
where
    Head: Unsigned,
    //Head: std::ops::Mul<<Tail as IndexShape>::Capacity>,
    Tail: HList + IndexShape,
    //<Tail as IndexShape>::Capacity: Unsigned,
{
    type Shape = HCons<usize, <Tail as IndexShape>::Shape>;
    //type Capacity = Prod<Head, <Tail as IndexShape>::Capacity>;

    fn get_index(index: Self::Shape) -> (usize, usize) {
        let (h, tail) = index.pop();
        let (offset, width) = Tail::get_index(tail);
        assert!(h < Head::to_usize());
        (h * width + offset, Head::to_usize())
    }

    fn get_capacity() -> usize {
        Head::to_usize() * Tail::get_capacity()
    }
}

pub trait HAppendable<RHS> {
    type Appended: HList;

    fn append(self, other: RHS) -> Self::Appended;
}

impl<T: HList> HAppendable<T> for HNil {
    type Appended = T;

    fn append(self, rhs: T) -> Self::Appended {
        rhs
    }
}

impl<H1, T1, RHS> HAppendable<RHS> for HCons<H1, T1>
where
    T1: HList + HAppendable<RHS>,
    RHS: HList,
{
    type Appended = HCons<H1, <T1 as HAppendable<RHS>>::Appended>;

    fn append(self, rhs: RHS) -> Self::Appended {
        HCons {
            head: self.head,
            tail: self.tail.append(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::*;

    #[test]
    fn append() {
        let h1 = hlist![1, "hi"];
        let h2 = hlist!["hoge", 11];

        assert_eq!(h1.append(h2), hlist![1, "hi", "hoge", 11]);
    }

    #[test]
    fn type_length() {
        assert_type_eq!(<Hlist!(U1, U1, U1) as TypeLength>::Length, U3);
    }
}
