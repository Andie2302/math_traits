//! Vektoren `[T; N]` mit komponentenweiser Addition und Skalarmultiplikation.
//!
//! Die Gesetze sind *bedingt*: Sie gelten genau dann, wenn `T` die passenden
//! Gesetze erfüllt. Das zeigt die generische Form von `laws!`.

use crate::laws;
use crate::laws::{ConjugateAdditive, SelfConjugate};
use crate::signature::{
    Additive, BinaryOp, HasConjugate, HasIdentity, HasInverse, InnerProduct, LessEq,
    Multiplicative, ScalarMul, op,
};
use crate::structures::{AbelianGroup, CommutativeRing, Involution, OrderedField, Ring};

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

/// `⟨v, w⟩ = Σ vᵢ* · wᵢ`
impl<T, const N: usize> InnerProduct<T> for [T; N]
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive> + HasConjugate,
{
    fn inner(&self, other: &Self) -> T {
        self.iter().zip(other).fold(T::identity(), |acc, (v, w)| {
            op::<Additive, _>(&acc, &op::<Multiplicative, _>(&v.conj(), w))
        })
    }
}

laws! {
    for[T: AbelianGroup<Additive>, const N: usize] [T; N] {
        Additive: associative, commutative, identity, inverse;
    }
    for[T: Ring, const N: usize] [T; N] {
        T: module;
    }
    for[T: CommutativeRing + Involution<Multiplicative> + ConjugateAdditive<Additive>, const N: usize] [T; N] {
        T: inner_product;
    }
    // Über reellen geordneten Skalaren ist das Skalarprodukt positiv definit.
    for[T: OrderedField + SelfConjugate<Multiplicative> + ConjugateAdditive<Additive>, const N: usize] [T; N] {
        T: inner_definite;
        [T, LessEq]: inner_non_negative;
    }
}
