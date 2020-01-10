use std::ops::Add;
use std::ops::Sub;

use frunk::hlist::HList;
use frunk::*;
use typenum::uint::Unsigned;
use typenum::*;

pub mod typelength;

pub use typelength::*;

pub mod hsliceable;

pub use hsliceable::*;

pub mod hinsertable;

pub use hinsertable::*;

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
    Tail: HList + IndexShape,
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
