//! Kontraktion (`A · B`) und Tensorprodukt (`A ⊗ B`) zwischen allen Stufen,
//! deren Ergebnis höchstens Stufe 4 hat.
//!
//! Kontraktion: letzter Index von `A` mit erstem Index von `B`. Flach
//! betrachtet ist `A` eine `(p × k)`-Matrix und `B` eine `(k × q)`-Matrix,
//! das Ergebnis eine `(p × q)`-Matrix. Daher genügt eine einzige Formel.

use super::{Tensor1, Tensor2, Tensor3, Tensor4, TensorShape};
use crate::laws;
use crate::signature::{Additive, BinaryOp, Contract, HasIdentity, Multiplicative, Outer, op};
use crate::structures::{CommutativeRing, Ring};

/// `Σₗ a[i·k + l] · b[l·q + j]` für den flachen Ergebnisindex `idx = i·q + j`.
fn contract_at<T>(a: &[T], b: &[T], k: usize, q: usize, idx: usize) -> T
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
{
    let (i, j) = (idx / q, idx % q);
    (0..k).fold(T::identity(), |acc, l| {
        op::<Additive, _>(&acc, &op::<Multiplicative, _>(&a[i * k + l], &b[l * q + j]))
    })
}

/// `a[idx / q] · b[idx % q]`
fn outer_at<T: BinaryOp<Multiplicative>>(a: &[T], b: &[T], q: usize, idx: usize) -> T {
    op::<Multiplicative, _>(&a[idx / q], &b[idx % q])
}

macro_rules! contract {
    ($([$($g:ident),*] $l:ty, $r:ty => $o:ty, k = $k:ident, q = [$($q:ident),*];)*) => {$(
        impl<T, $(const $g: usize),*> Contract<$r> for $l
        where
            T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
        {
            type Output = $o;
            fn contract(&self, rhs: &$r) -> $o {
                let (a, b) = (self.as_slice(), rhs.as_slice());
                <$o>::from_flat_fn(|idx| contract_at(a, b, $k, 1 $(* $q)*, idx))
            }
        }
    )*};
}

contract! {
    [M, K] Tensor2<T, M, K>, Tensor1<T, K> => Tensor1<T, M>, k = K, q = [];
    [K, N] Tensor1<T, K>, Tensor2<T, K, N> => Tensor1<T, N>, k = K, q = [N];
    [M, K, N] Tensor2<T, M, K>, Tensor2<T, K, N> => Tensor2<T, M, N>, k = K, q = [N];
    [K, B, C] Tensor1<T, K>, Tensor3<T, K, B, C> => Tensor2<T, B, C>, k = K, q = [B, C];
    [A, B, K] Tensor3<T, A, B, K>, Tensor1<T, K> => Tensor2<T, A, B>, k = K, q = [];
    [M, K, B, C] Tensor2<T, M, K>, Tensor3<T, K, B, C> => Tensor3<T, M, B, C>, k = K, q = [B, C];
    [A, B, K, N] Tensor3<T, A, B, K>, Tensor2<T, K, N> => Tensor3<T, A, B, N>, k = K, q = [N];
    [K, B, C, D] Tensor1<T, K>, Tensor4<T, K, B, C, D> => Tensor3<T, B, C, D>, k = K, q = [B, C, D];
    [A, B, C, K] Tensor4<T, A, B, C, K>, Tensor1<T, K> => Tensor3<T, A, B, C>, k = K, q = [];
    [A, B, K, C, D] Tensor3<T, A, B, K>, Tensor3<T, K, C, D> => Tensor4<T, A, B, C, D>, k = K, q = [C, D];
    [M, K, B, C, D] Tensor2<T, M, K>, Tensor4<T, K, B, C, D> => Tensor4<T, M, B, C, D>, k = K, q = [B, C, D];
    [A, B, C, K, N] Tensor4<T, A, B, C, K>, Tensor2<T, K, N> => Tensor4<T, A, B, C, N>, k = K, q = [N];
}

/// Vektor · Vektor ohne Konjugation: `Σ aᵢ·bᵢ` (Skalar).
impl<T, const N: usize> Contract<Tensor1<T, N>> for Tensor1<T, N>
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
{
    type Output = T;
    fn contract(&self, rhs: &Tensor1<T, N>) -> T {
        contract_at(&self.0, &rhs.0, N, 1, 0)
    }
}

macro_rules! outer {
    ($([$($g:ident),*] $l:ty, $r:ty => $o:ty, q = [$($q:ident),*];)*) => {$(
        impl<T: BinaryOp<Multiplicative>, $(const $g: usize),*> Outer<$r> for $l {
            type Output = $o;
            fn outer(&self, rhs: &$r) -> $o {
                let (a, b) = (self.as_slice(), rhs.as_slice());
                <$o>::from_flat_fn(|idx| outer_at(a, b, 1 $(* $q)*, idx))
            }
        }
    )*};
}

outer! {
    [M, N] Tensor1<T, M>, Tensor1<T, N> => Tensor2<T, M, N>, q = [N];
    [A, B, C] Tensor1<T, A>, Tensor2<T, B, C> => Tensor3<T, A, B, C>, q = [B, C];
    [A, B, C] Tensor2<T, A, B>, Tensor1<T, C> => Tensor3<T, A, B, C>, q = [C];
    [A, B, C, D] Tensor1<T, A>, Tensor3<T, B, C, D> => Tensor4<T, A, B, C, D>, q = [B, C, D];
    [A, B, C, D] Tensor3<T, A, B, C>, Tensor1<T, D> => Tensor4<T, A, B, C, D>, q = [D];
    [A, B, C, D] Tensor2<T, A, B>, Tensor2<T, C, D> => Tensor4<T, A, B, C, D>, q = [C, D];
}

// --- Quadratische Matrizen bilden einen Ring --------------------------------------

impl<T, const N: usize> BinaryOp<Multiplicative> for Tensor2<T, N, N>
where
    T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
{
    fn op(&self, rhs: &Self) -> Self {
        self.contract(rhs)
    }
}

/// Einheitsmatrix.
impl<T, const N: usize> HasIdentity<Multiplicative> for Tensor2<T, N, N>
where
    T: HasIdentity<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
{
    fn identity() -> Self {
        Tensor2::from_flat_fn(|idx| {
            if idx / N == idx % N {
                <T as HasIdentity<Multiplicative>>::identity()
            } else {
                <T as HasIdentity<Additive>>::identity()
            }
        })
    }
}

laws! {
    for[T: Ring, const N: usize] Tensor2<T, N, N> {
        Multiplicative: associative, identity;
        [Multiplicative, Additive]: distributive;
    }
    for[T: CommutativeRing, const N: usize] Tensor2<T, N, N> {
        [Multiplicative, T]: op_homogeneous;
    }

    // Matrix · Matrix und Matrix · Vektor: bilinear und assoziativ
    for[T: Ring, const M: usize, const K: usize] Tensor2<T, M, K> {
        Tensor1<T, K>: contract_bilinear;
    }
    for[T: Ring, const M: usize, const K: usize, const N: usize] Tensor2<T, M, K> {
        Tensor2<T, K, N>: contract_bilinear;
        [Tensor2<T, K, N>, Tensor1<T, N>]: contract_associative;
    }
    for[T: CommutativeRing, const M: usize, const K: usize] Tensor2<T, M, K> {
        [Tensor1<T, K>, T]: contract_homogeneous;
    }
    for[T: CommutativeRing, const M: usize, const K: usize, const N: usize] Tensor2<T, M, K> {
        [Tensor2<T, K, N>, T]: contract_homogeneous;
    }
    for[T: Ring, const M: usize, const K: usize, const N: usize, const P: usize] Tensor2<T, M, K> {
        [Tensor2<T, K, N>, Tensor2<T, N, P>]: contract_associative;
    }

    // Tensorprodukt: bilinear und assoziativ
    for[T: Ring, const A: usize, const B: usize] Tensor1<T, A> {
        Tensor1<T, B>: outer_bilinear;
    }
    for[T: Ring, const A: usize, const B: usize, const C: usize] Tensor1<T, A> {
        [Tensor1<T, B>, Tensor1<T, C>]: outer_associative;
    }
}
