use std::ops::Add;
use std::ops::Sub;

use frunk::hlist::HList;
use frunk::*;
use typenum::uint::Unsigned;
use typenum::*;

use crate::util::*;

pub trait HInsertable<I, A>: TypeLength
where
    I: Unsigned + IsLessOrEqual<<Self as TypeLength>::Length, Output = B1>,
{
    type Inserted: HList;
    fn insert(self, index: I, item: A) -> Self::Inserted;
}

impl<A> HInsertable<U0, A> for HNil {
    type Inserted = HCons<A, HNil>;

    fn insert(self, _: U0, item: A) -> Self::Inserted {
        hlist!(item)
    }
}

impl<A, Head, Tail> HInsertable<U0, A> for HCons<Head, Tail>
where
    Tail: HList + TypeLength,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    U0: IsLessOrEqual<<Self as TypeLength>::Length, Output = B1>,
{
    type Inserted = HCons<A, Self>;

    fn insert(self, _: U0, item: A) -> Self::Inserted {
        HCons {
            head: item,
            tail: self,
        }
    }
}

impl<A, Head, Tail> HInsertable<U1, A> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HInsertable<Sub1<U1>, A>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    U1: IsLessOrEqual<<Self as TypeLength>::Length, Output = B1>,
    Sub1<U1>: IsLessOrEqual<<Tail as TypeLength>::Length, Output = B1>,
{
    type Inserted = HCons<Head, <Tail as HInsertable<U0, A>>::Inserted>;

    fn insert(self, index: U1, item: A) -> Self::Inserted {
        HCons {
            head: self.head,
            tail: self.tail.insert(index - B1::new(), item),
        }
    }
}

impl<A, U: Unsigned, B: Bit, Head, Tail> HInsertable<UInt<UInt<U, B>, B0>, A> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HInsertable<Sub1<UInt<UInt<U, B>, B0>>, A>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<U, B>, B0>: Sub<B1> + IsLessOrEqual<<Self as TypeLength>::Length, Output = B1>,
    Sub1<UInt<UInt<U, B>, B0>>: Unsigned + IsLessOrEqual<<Tail as TypeLength>::Length, Output = B1>,
{
    type Inserted = HCons<Head, <Tail as HInsertable<Sub1<UInt<UInt<U, B>, B0>>, A>>::Inserted>;

    fn insert(self, index: UInt<UInt<U, B>, B0>, item: A) -> Self::Inserted {
        HCons {
            head: self.head,
            tail: self.tail.insert(index - B1::new(), item),
        }
    }
}

impl<A, U: Unsigned, B: Bit, Head, Tail> HInsertable<UInt<UInt<U, B>, B1>, A> for HCons<Head, Tail>
where
    Tail: HList + TypeLength + HInsertable<Sub1<UInt<UInt<U, B>, B1>>, A>,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
    UInt<UInt<U, B>, B1>: Sub<B1> + IsLessOrEqual<<Self as TypeLength>::Length, Output = B1>,
    Sub1<UInt<UInt<U, B>, B1>>: Unsigned + IsLessOrEqual<<Tail as TypeLength>::Length, Output = B1>,
{
    type Inserted = HCons<Head, <Tail as HInsertable<Sub1<UInt<UInt<U, B>, B1>>, A>>::Inserted>;

    fn insert(self, index: UInt<UInt<U, B>, B1>, item: A) -> Self::Inserted {
        HCons {
            head: self.head,
            tail: self.tail.insert(index - B1::new(), item),
        }
    }
}
