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
    //Head: Unsigned,
    Tail: HList + TypeLength,
    <Tail as TypeLength>::Length: Unsigned + Add<U1>,
    <<Tail as TypeLength>::Length as Add<U1>>::Output: Unsigned,
{
    type Length = <<Tail as TypeLength>::Length as Add<U1>>::Output;
}

#[cfg(test)]
mod tests {
    use crate::util::*;

    use typenum::*;

    #[test]
    fn type_length() {
        assert_type_eq!(<Hlist!(U1, U1, U1) as TypeLength>::Length, U3);
    }
}
