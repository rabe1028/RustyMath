use std::marker::PhantomData;
use typenum::*;

pub struct IntegerQuotientRing<N: Unsigned> {
    value: u64,
    _phantom: PhantomData<N>,
}

