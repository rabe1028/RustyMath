pub mod magma;

pub mod quasi_group;
pub mod semigroup;
pub mod unital_magma;

pub mod _loop;
pub mod inverse_semigroup;
pub mod monoid;

pub mod abelian_group;
pub mod group;

pub use magma::*;

pub use quasi_group::*;
pub use semigroup::*;
pub use unital_magma::*;

pub use _loop::*;
pub use inverse_semigroup::*;
pub use monoid::*;

pub use abelian_group::*;
pub use group::*;

// Commutative

pub mod commutative_monoid;
pub use commutative_monoid::*;
