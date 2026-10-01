//! Vektoren `[T; N]` mit komponentenweiser Addition und Skalarmultiplikation.
//!
//! Die Gesetze sind *bedingt*: Sie gelten genau dann, wenn `T` die passenden
//! Gesetze erfüllt. Das zeigt die generische Form von `laws!`.

use crate::laws;
use crate::signature::{
    Additive, BinaryOp, HasIdentity, HasInverse, Multiplicative, ScalarMul, op,
};
use crate::structures::{AbelianGroup, Ring};

impl<T: BinaryOp<Additive>, const N: usize> BinaryOp<Additive> for [T; N] {
    fn op(&self, rhs: &Self) -> Self {
        core::array::from_fn(|i| op::<Additive, _>(&self[i], &rhs[i]))
    }
}

impl<T: HasIdentity<Additive>, const N: usize> HasIdentity<Additive> for [T; N] {
    fn identity() -> Self {
        core::array::from_fn(|_| T::identity())
    }
}

impl<T: HasInverse<Additive>, const N: usize> HasInverse<Additive> for [T; N] {
    fn inverse(&self) -> Self {
        core::array::from_fn(|i| self[i].inverse())
    }
}

/// `s · (v₁, …, vₙ) = (s·v₁, …, s·vₙ)`
impl<T: BinaryOp<Multiplicative>, const N: usize> ScalarMul<T> for [T; N] {
    fn scale(&self, s: &T) -> Self {
        core::array::from_fn(|i| op::<Multiplicative, _>(s, &self[i]))
    }
}

laws! {
    for[T: AbelianGroup<Additive>, const N: usize] [T; N] {
        Additive: associative, commutative, identity, inverse;
    }
    for[T: Ring, const N: usize] [T; N] {
        T: module;
    }
}
