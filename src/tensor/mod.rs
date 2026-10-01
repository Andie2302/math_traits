//! Tensoren fester Größe bis Stufe 4, plus ein dynamischer Tensor (`alloc`).
//!
//! | Typ | Alias | Speicher |
//! |---|---|---|
//! | [`Tensor1<T, N>`] | [`Vector<T, N>`] | `[T; N]` |
//! | [`Tensor2<T, M, N>`] | [`Matrix<T, M, N>`], [`SquareMatrix<T, N>`] | `[[T; N]; M]` |
//! | [`Tensor3<T, A, B, C>`] | | `[[[T; C]; B]; A]` |
//! | [`Tensor4<T, A, B, C, D>`] | | `[[[[T; D]; C]; B]; A]` |
//!
//! Alle Tensoren sind zeilenweise (row-major) gespeichert. Mit
//! [`TensorShape::as_slice`] und [`TensorShape::SHAPE`] lassen sie sich direkt
//! an [`fft_nd`](crate::fft::fft_nd) übergeben.
//!
//! Struktur, die die Gesetze garantieren:
//! * jeder Tensor ist ein [`Module`](crate::structures::Module) über `T`
//!   mit Frobenius-Skalarprodukt `⟨A, B⟩ = Σ Aᵢ*·Bᵢ`,
//! * quadratische Matrizen bilden einen [`Ring`](crate::structures::Ring) und
//!   über kommutativem `T` eine [`AssociativeAlgebra`](crate::structures::AssociativeAlgebra),
//! * Multiplikation zwischen verschiedenen Stufen: [`Contract`] und [`Outer`].
//!
//! [`Contract`]: crate::signature::Contract
//! [`Outer`]: crate::signature::Outer

mod contract;
#[cfg(feature = "alloc")]
pub mod dynamic;
mod lie;
pub mod matrix;

use crate::laws;
use crate::laws::{ConjugateAdditive, SelfConjugate};
use crate::signature::{
    Additive, BinaryOp, HasConjugate, HasIdentity, HasInverse, InnerProduct, LessEq,
    Multiplicative, ScalarMul, op,
};
use crate::structures::{AbelianGroup, CommutativeRing, Involution, OrderedField, Ring};

/// Vektor (Stufe 1).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Tensor1<T, const N: usize>(pub [T; N]);

/// Matrix (Stufe 2), `M` Zeilen, `N` Spalten.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Tensor2<T, const M: usize, const N: usize>(pub [[T; N]; M]);

/// Tensor der Stufe 3.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Tensor3<T, const A: usize, const B: usize, const C: usize>(pub [[[T; C]; B]; A]);

/// Tensor der Stufe 4.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Tensor4<T, const A: usize, const B: usize, const C: usize, const D: usize>(
    pub [[[[T; D]; C]; B]; A],
);

pub type Vector<T, const N: usize> = Tensor1<T, N>;
pub type Matrix<T, const M: usize, const N: usize> = Tensor2<T, M, N>;
pub type SquareMatrix<T, const N: usize> = Tensor2<T, N, N>;

/// Gemeinsame Sicht auf alle Tensoren fester Größe: flach, zeilenweise.
pub trait TensorShape: Sized {
    type Elem;
    /// Die Form, z. B. `[M, N]` für eine Matrix.
    const SHAPE: &'static [usize];
    fn as_slice(&self) -> &[Self::Elem];
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
    /// Baut einen Tensor aus seinem flachen Index.
    fn from_flat_fn(f: impl FnMut(usize) -> Self::Elem) -> Self;
}

impl<T, const N: usize> TensorShape for Tensor1<T, N> {
    type Elem = T;
    const SHAPE: &'static [usize] = &[N];
    fn as_slice(&self) -> &[T] {
        &self.0
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }
    fn from_flat_fn(f: impl FnMut(usize) -> T) -> Self {
        Tensor1(core::array::from_fn(f))
    }
}

impl<T, const M: usize, const N: usize> TensorShape for Tensor2<T, M, N> {
    type Elem = T;
    const SHAPE: &'static [usize] = &[M, N];
    fn as_slice(&self) -> &[T] {
        self.0.as_flattened()
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_flattened_mut()
    }
    fn from_flat_fn(mut f: impl FnMut(usize) -> T) -> Self {
        Tensor2(core::array::from_fn(|i| {
            core::array::from_fn(|j| f(i * N + j))
        }))
    }
}

impl<T, const A: usize, const B: usize, const C: usize> TensorShape for Tensor3<T, A, B, C> {
    type Elem = T;
    const SHAPE: &'static [usize] = &[A, B, C];
    fn as_slice(&self) -> &[T] {
        self.0.as_flattened().as_flattened()
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_flattened_mut().as_flattened_mut()
    }
    fn from_flat_fn(mut f: impl FnMut(usize) -> T) -> Self {
        Tensor3(core::array::from_fn(|i| {
            core::array::from_fn(|j| core::array::from_fn(|k| f((i * B + j) * C + k)))
        }))
    }
}

impl<T, const A: usize, const B: usize, const C: usize, const D: usize> TensorShape
    for Tensor4<T, A, B, C, D>
{
    type Elem = T;
    const SHAPE: &'static [usize] = &[A, B, C, D];
    fn as_slice(&self) -> &[T] {
        self.0.as_flattened().as_flattened().as_flattened()
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.0
            .as_flattened_mut()
            .as_flattened_mut()
            .as_flattened_mut()
    }
    fn from_flat_fn(mut f: impl FnMut(usize) -> T) -> Self {
        Tensor4(core::array::from_fn(|i| {
            core::array::from_fn(|j| {
                core::array::from_fn(|k| core::array::from_fn(|l| f(((i * B + j) * C + k) * D + l)))
            })
        }))
    }
}

/// Elementweise Struktur (Addition, Skalare, Frobenius-Skalarprodukt) und ihre
/// Gesetze, für alle vier Stufen gleich.
macro_rules! elementwise {
    ($($name:ident<$($d:ident),+>);* $(;)?) => {$(
        impl<T: BinaryOp<Additive>, $(const $d: usize),+> BinaryOp<Additive> for $name<T, $($d),+> {
            fn op(&self, rhs: &Self) -> Self {
                let (a, b) = (self.as_slice(), rhs.as_slice());
                Self::from_flat_fn(|i| op::<Additive, _>(&a[i], &b[i]))
            }
        }
        impl<T: HasIdentity<Additive>, $(const $d: usize),+> HasIdentity<Additive> for $name<T, $($d),+> {
            fn identity() -> Self {
                Self::from_flat_fn(|_| T::identity())
            }
        }
        impl<T: HasInverse<Additive>, $(const $d: usize),+> HasInverse<Additive> for $name<T, $($d),+> {
            fn inverse(&self) -> Self {
                let a = self.as_slice();
                Self::from_flat_fn(|i| a[i].inverse())
            }
        }
        /// `s · A`, elementweise von links.
        impl<T: BinaryOp<Multiplicative>, $(const $d: usize),+> ScalarMul<T> for $name<T, $($d),+> {
            fn scale(&self, s: &T) -> Self {
                let a = self.as_slice();
                Self::from_flat_fn(|i| op::<Multiplicative, _>(s, &a[i]))
            }
        }
        /// Frobenius: `⟨A, B⟩ = Σ Aᵢ*·Bᵢ`
        impl<T, $(const $d: usize),+> InnerProduct<T> for $name<T, $($d),+>
        where
            T: BinaryOp<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive> + HasConjugate,
        {
            fn inner(&self, other: &Self) -> T {
                self.as_slice().iter().zip(other.as_slice()).fold(T::identity(), |acc, (a, b)| {
                    op::<Additive, _>(&acc, &op::<Multiplicative, _>(&a.conj(), b))
                })
            }
        }

        laws! {
            for[T: AbelianGroup<Additive>, $(const $d: usize),+] $name<T, $($d),+> {
                Additive: associative, commutative, identity, inverse;
            }
            for[T: Ring, $(const $d: usize),+] $name<T, $($d),+> {
                T: module;
            }
            for[T: CommutativeRing + Involution<Multiplicative> + ConjugateAdditive<Additive>, $(const $d: usize),+] $name<T, $($d),+> {
                T: inner_product;
            }
            for[T: OrderedField + SelfConjugate<Multiplicative> + ConjugateAdditive<Additive>, $(const $d: usize),+] $name<T, $($d),+> {
                T: inner_definite;
                [T, LessEq]: inner_non_negative;
            }
        }
    )*};
}

elementwise! {
    Tensor1<N>;
    Tensor2<M, N>;
    Tensor3<A, B, C>;
    Tensor4<A, B, C, D>;
}
