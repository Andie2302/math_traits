//! Tensoren vom Rang 0 bis 32 mit Achsenlaengen auf Typ-Ebene.
//!
//! `TensorN<T, D1, .., DN>` ist ein Alias auf [`Nd`] ueber verschachtelten
//! Arrays `[[..[T; DN]..; D2]; D1]`; die Achsenlaengen duerfen verschieden
//! sein. Alles liegt auf dem Stack (`no_std`, kein `alloc`).
//!
//! Alle Raenge teilen sich dieselben Implementierungen: [`Storage`] ist
//! rekursiv ueber die Array-Verschachtelung definiert, [`Tensor`] ist das
//! gemeinsame Supertrait.
//!
//! Achsen-Operationen, die die Form aendern ([`Nd::swap_adjacent`],
//! [`Nd::contract_adjacent`]), waehlen die Achse mit Peano-Zahlen
//! ([`U0`]..[`U31`]), da Rust (stabil) keine Rechnung mit const-Generics
//! erlaubt. Damit lassen sich alle Permutationen/Kontraktionen
//! zusammensetzen.

use crate::real::Real;
use crate::scalar::{Scalar, Signed};
use crate::traits::TrivialZero;
use core::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

mod ranks;
pub use ranks::*;

/// Hoechster unterstuetzter Rang.
pub const MAX_RANK: usize = 32;

// ---- Peano-Zahlen fuer Achsen-Position ------------------------------------

/// Typ-Ebene: Achse 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Z;
/// Typ-Ebene: Nachfolger von `N`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Succ<N>(core::marker::PhantomData<N>);

macro_rules! peano {
    ($prev:ident; $($name:ident),*) => {
        peano!(@ $prev; $($name),*);
    };
    (@ $prev:ty;) => {};
    (@ $prev:ty; $name:ident $(, $rest:ident)*) => {
        #[doc = "Achsenposition (Peano-Zahl)."]
        pub type $name = Succ<$prev>;
        peano!(@ $name; $($rest),*);
    };
}
/// Achsenposition 0.
pub type U0 = Z;
peano!(U0; U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16,
       U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31);

// ---- Storage: rekursiv ueber verschachtelte Arrays -------------------------

/// Speicherlayout eines Tensors: ein Skalar (Rang 0) oder ein Array von Speicher (Rang + 1).
///
/// Implementierungsdetail; man benutzt [`Tensor`] und [`Nd`].
pub trait Storage: Copy + PartialEq {
    type Scalar: Scalar;
    const RANK: usize;
    /// Gesamtzahl der Eintraege (Produkt aller Achsenlaengen).
    const LEN: usize;

    fn zero() -> Self;
    fn dim_of(axis: usize) -> usize;
    fn get_at(&self, idx: &[usize]) -> Option<&Self::Scalar>;
    fn get_mut_at(&mut self, idx: &[usize]) -> Option<&mut Self::Scalar>;
    fn from_fn_at(
        idx: &mut [usize],
        depth: usize,
        f: &mut impl FnMut(&[usize]) -> Self::Scalar,
    ) -> Self;
    fn map_with(self, f: &mut impl FnMut(Self::Scalar) -> Self::Scalar) -> Self;
    fn zip_with(
        self,
        o: Self,
        f: &mut impl FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar,
    ) -> Self;
    fn fold_with<A>(self, acc: A, f: &mut impl FnMut(A, Self::Scalar) -> A) -> A;
}

impl<T: Scalar> Storage for T {
    type Scalar = T;
    const RANK: usize = 0;
    const LEN: usize = 1;

    fn zero() -> Self {
        T::ZERO
    }
    fn dim_of(axis: usize) -> usize {
        panic!("Achse {axis} existiert nicht");
    }
    fn get_at(&self, idx: &[usize]) -> Option<&T> {
        idx.is_empty().then_some(self)
    }
    fn get_mut_at(&mut self, idx: &[usize]) -> Option<&mut T> {
        idx.is_empty().then_some(self)
    }
    fn from_fn_at(idx: &mut [usize], depth: usize, f: &mut impl FnMut(&[usize]) -> T) -> Self {
        f(&idx[..depth])
    }
    fn map_with(self, f: &mut impl FnMut(T) -> T) -> Self {
        f(self)
    }
    fn zip_with(self, o: Self, f: &mut impl FnMut(T, T) -> T) -> Self {
        f(self, o)
    }
    fn fold_with<A>(self, acc: A, f: &mut impl FnMut(A, T) -> A) -> A {
        f(acc, self)
    }
}

impl<S: Storage, const N: usize> Storage for [S; N] {
    type Scalar = S::Scalar;
    const RANK: usize = S::RANK + 1;
    const LEN: usize = N * S::LEN;

    fn zero() -> Self {
        [S::zero(); N]
    }
    fn dim_of(axis: usize) -> usize {
        if axis == 0 { N } else { S::dim_of(axis - 1) }
    }
    fn get_at(&self, idx: &[usize]) -> Option<&Self::Scalar> {
        let (&i, rest) = idx.split_first()?;
        self.get(i)?.get_at(rest)
    }
    fn get_mut_at(&mut self, idx: &[usize]) -> Option<&mut Self::Scalar> {
        let (&i, rest) = idx.split_first()?;
        self.get_mut(i)?.get_mut_at(rest)
    }
    fn from_fn_at(
        idx: &mut [usize],
        depth: usize,
        f: &mut impl FnMut(&[usize]) -> Self::Scalar,
    ) -> Self {
        core::array::from_fn(|i| {
            idx[depth] = i;
            S::from_fn_at(idx, depth + 1, f)
        })
    }
    fn map_with(self, f: &mut impl FnMut(Self::Scalar) -> Self::Scalar) -> Self {
        self.map(|s| s.map_with(f))
    }
    fn zip_with(
        self,
        o: Self,
        f: &mut impl FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar,
    ) -> Self {
        core::array::from_fn(|i| self[i].zip_with(o[i], f))
    }
    fn fold_with<A>(self, mut acc: A, f: &mut impl FnMut(A, Self::Scalar) -> A) -> A {
        for s in self {
            acc = s.fold_with(acc, f);
        }
        acc
    }
}

// ---- Nd: der eine Tensor-Typ hinter allen Aliasen --------------------------

/// Tensor ueber dem Speicherlayout `S`. Benutze die Aliase `Tensor0` .. `Tensor32`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nd<S>(pub S);

/// Gemeinsames Supertrait aller Tensoren `Tensor0` .. `Tensor32`.
///
/// `+`, `-` und Skalar-Multiplikation gibt es immer; `Neg` nur fuer
/// vorzeichenbehaftete Skalare.
pub trait Tensor: TrivialZero + Copy + PartialEq + Add<Output = Self> + Sub<Output = Self> {
    type Scalar: Scalar;
    /// Anzahl der Achsen (0..=32).
    const RANK: usize;
    /// Gesamtzahl der Eintraege (Produkt der Achsenlaengen; Rang 0: 1).
    const LEN: usize;

    /// Laenge der Achse `axis < RANK`; panikt sonst.
    fn dim(axis: usize) -> usize;
    /// Schreibt die Achsenlaengen nach `out[..RANK]`; panikt, wenn `out` zu kurz ist.
    fn shape_into(out: &mut [usize]) {
        for (axis, o) in out[..Self::RANK].iter_mut().enumerate() {
            *o = Self::dim(axis);
        }
    }
    /// Eintrag am Mehrfachindex (`idx.len() == RANK`), sonst `None`.
    fn get(&self, idx: &[usize]) -> Option<&Self::Scalar>;
    fn get_mut(&mut self, idx: &[usize]) -> Option<&mut Self::Scalar>;
    /// Baut den Tensor aus `f(mehrfachindex)`.
    fn from_fn(f: impl FnMut(&[usize]) -> Self::Scalar) -> Self;
    fn map(self, f: impl FnMut(Self::Scalar) -> Self::Scalar) -> Self;
    fn zip_map(
        self,
        other: Self,
        f: impl FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar,
    ) -> Self;
    fn fold<A>(self, init: A, f: impl FnMut(A, Self::Scalar) -> A) -> A;

    /// Alle Eintraege mit `s` multiplizieren.
    fn scale(self, s: Self::Scalar) -> Self {
        self.map(|x| x * s)
    }
    /// Summe aller Eintraege.
    fn sum(self) -> Self::Scalar {
        self.fold(Self::Scalar::ZERO, |a, x| a + x)
    }
    /// Frobenius-Skalarprodukt: Summe der Produkte gleicher Eintraege.
    fn dot(self, other: Self) -> Self::Scalar {
        self.zip_map(other, |a, b| a * b).sum()
    }
}

impl<S: Storage> TrivialZero for Nd<S> {
    fn zero() -> Self {
        Nd(S::zero())
    }
    fn is_zero(&self) -> bool {
        self.0
            .fold_with(true, &mut |a, x| a && x == S::Scalar::ZERO)
    }
}

impl<S: Storage> Tensor for Nd<S> {
    type Scalar = S::Scalar;
    const RANK: usize = S::RANK;
    const LEN: usize = S::LEN;

    fn dim(axis: usize) -> usize {
        S::dim_of(axis)
    }
    fn get(&self, idx: &[usize]) -> Option<&S::Scalar> {
        self.0.get_at(idx)
    }
    fn get_mut(&mut self, idx: &[usize]) -> Option<&mut S::Scalar> {
        self.0.get_mut_at(idx)
    }
    fn from_fn(mut f: impl FnMut(&[usize]) -> S::Scalar) -> Self {
        let mut idx = [0usize; MAX_RANK];
        Nd(S::from_fn_at(&mut idx, 0, &mut f))
    }
    fn map(self, mut f: impl FnMut(S::Scalar) -> S::Scalar) -> Self {
        Nd(self.0.map_with(&mut f))
    }
    fn zip_map(self, o: Self, mut f: impl FnMut(S::Scalar, S::Scalar) -> S::Scalar) -> Self {
        Nd(self.0.zip_with(o.0, &mut f))
    }
    fn fold<A>(self, init: A, mut f: impl FnMut(A, S::Scalar) -> A) -> A {
        self.0.fold_with(init, &mut f)
    }
}

impl<S: Storage> Add for Nd<S> {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        self.zip_map(o, |a, b| a + b)
    }
}
impl<S: Storage> Sub for Nd<S> {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        self.zip_map(o, |a, b| a - b)
    }
}
impl<S: Storage> Neg for Nd<S>
where
    S::Scalar: Signed,
{
    type Output = Self;
    fn neg(self) -> Self {
        self.map(|a| -a)
    }
}
impl<S: Storage> core::ops::AddAssign for Nd<S> {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}
impl<S: Storage> core::ops::SubAssign for Nd<S> {
    fn sub_assign(&mut self, o: Self) {
        *self = *self - o;
    }
}

/// Skalar-Multiplikation `tensor * skalar` (je Skalartyp, da `Mul<S::Scalar>`
/// sich mit dem Matrixprodukt nicht generisch vertraegt).
macro_rules! scalar_mul {
    ($($t:ty),*) => {$(
        impl<S: Storage<Scalar = $t>> Mul<$t> for Nd<S> {
            type Output = Self;
            fn mul(self, s: $t) -> Self {
                self.scale(s)
            }
        }
    )*};
}
scalar_mul!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
#[cfg(feature = "f16")]
scalar_mul!(f16);
#[cfg(feature = "f128")]
scalar_mul!(f128);

/// `t[i]` liefert die i-te Scheibe (Rang - 1) bzw. bei Rang 1 den Eintrag.
impl<S, const N: usize> Index<usize> for Nd<[S; N]> {
    type Output = S;
    fn index(&self, i: usize) -> &S {
        &self.0[i]
    }
}
impl<S, const N: usize> IndexMut<usize> for Nd<[S; N]> {
    fn index_mut(&mut self, i: usize) -> &mut S {
        &mut self.0[i]
    }
}

// ---- Achsen vertauschen / kontrahieren ---------------------------------------

/// Vertauscht die Achsen `D` und `D + 1`.
pub trait SwapAt<D>: Storage {
    type Out: Storage<Scalar = Self::Scalar>;
    fn swap_at(self) -> Self::Out;
}
impl<S: Storage, const A: usize, const B: usize> SwapAt<Z> for [[S; B]; A] {
    type Out = [[S; A]; B];
    fn swap_at(self) -> Self::Out {
        core::array::from_fn(|b| core::array::from_fn(|a| self[a][b]))
    }
}
impl<S: SwapAt<D>, D, const M: usize> SwapAt<Succ<D>> for [S; M] {
    type Out = [S::Out; M];
    fn swap_at(self) -> Self::Out {
        self.map(SwapAt::<D>::swap_at)
    }
}

/// Kontrahiert (summiert ueber die Diagonale) die gleich langen Achsen `D` und `D + 1`.
pub trait ContractAt<D>: Storage {
    type Out: Storage<Scalar = Self::Scalar>;
    fn contract_at(self) -> Self::Out;
}
impl<S: Storage, const N: usize> ContractAt<Z> for [[S; N]; N] {
    type Out = S;
    fn contract_at(self) -> S {
        let mut acc = S::zero();
        for (i, row) in self.iter().enumerate() {
            acc = acc.zip_with(row[i], &mut |a, b| a + b);
        }
        acc
    }
}
impl<S: ContractAt<D>, D, const M: usize> ContractAt<Succ<D>> for [S; M] {
    type Out = [S::Out; M];
    fn contract_at(self) -> Self::Out {
        self.map(ContractAt::<D>::contract_at)
    }
}

impl<S: Storage> Nd<S> {
    /// Vertauscht die benachbarten Achsen `D` und `D + 1`, z.B.
    /// `t.swap_adjacent::<U0>()` auf `Tensor3<T, A, B, C>` ergibt `Tensor3<T, B, A, C>`.
    pub fn swap_adjacent<D>(self) -> Nd<<S as SwapAt<D>>::Out>
    where
        S: SwapAt<D>,
    {
        Nd(self.0.swap_at())
    }

    /// Kontrahiert die benachbarten, gleich langen Achsen `D` und `D + 1`
    /// (Rang sinkt um 2). Rang 2 -> Rang 0 ist die Spur.
    pub fn contract_adjacent<D>(self) -> Nd<<S as ContractAt<D>>::Out>
    where
        S: ContractAt<D>,
    {
        Nd(self.0.contract_at())
    }
}

// ---- Rang 2: Transponieren, Spur, Matrixprodukt -------------------------------

impl<T: Scalar, const R: usize, const C: usize> Tensor2<T, R, C> {
    /// Transponierte Matrix.
    pub fn transpose(self) -> Tensor2<T, C, R> {
        self.swap_adjacent::<U0>()
    }
}
impl<T: Scalar, const N: usize> Tensor2<T, N, N> {
    /// Spur (Summe der Diagonale).
    pub fn trace(self) -> T {
        self.contract_adjacent::<U0>().0
    }
    /// Einheitsmatrix.
    pub fn identity() -> Self {
        Self::from_fn(|i| if i[0] == i[1] { T::ONE } else { T::ZERO })
    }
}

/// Matrixprodukt `(R x K) * (K x C) -> (R x C)`.
impl<T: Scalar, const R: usize, const K: usize, const C: usize> Mul<Tensor2<T, K, C>>
    for Tensor2<T, R, K>
{
    type Output = Tensor2<T, R, C>;
    fn mul(self, o: Tensor2<T, K, C>) -> Self::Output {
        Nd(core::array::from_fn(|r| {
            core::array::from_fn(|c| {
                let mut acc = T::ZERO;
                for k in 0..K {
                    acc = acc + self.0[r][k] * o.0[k][c];
                }
                acc
            })
        }))
    }
}
/// Matrix-Vektor-Produkt `(R x K) * (K) -> (R)`.
impl<T: Scalar, const R: usize, const K: usize> Mul<Tensor1<T, K>> for Tensor2<T, R, K> {
    type Output = Tensor1<T, R>;
    fn mul(self, v: Tensor1<T, K>) -> Self::Output {
        Nd(core::array::from_fn(|r| {
            let mut acc = T::ZERO;
            for k in 0..K {
                acc = acc + self.0[r][k] * v.0[k];
            }
            acc
        }))
    }
}

// ---- Tensor0 <-> Real, Tensor1 <-> Cayley-Dickson ----------------------------

impl<T: Scalar> From<Real<T>> for Tensor0<T> {
    fn from(r: Real<T>) -> Self {
        Nd(r.0)
    }
}
impl<T: Scalar> From<Tensor0<T>> for Real<T> {
    fn from(t: Tensor0<T>) -> Self {
        Real(t.0)
    }
}
impl<T: Scalar> From<T> for Tensor0<T> {
    fn from(t: T) -> Self {
        Nd(t)
    }
}

macro_rules! cd_vector {
    ($($alias:ident = $n:literal),*) => {$(
        impl<T: Signed> From<crate::$alias<T>> for Tensor1<T, $n> {
            fn from(x: crate::$alias<T>) -> Self {
                Nd(<[T; $n]>::from(x))
            }
        }
        impl<T: Signed> From<Tensor1<T, $n>> for crate::$alias<T> {
            fn from(t: Tensor1<T, $n>) -> Self {
                t.0.into()
            }
        }
    )*};
}
cd_vector!(
    Complex = 2,
    Quaternion = 4,
    Octonion = 8,
    Sedenion = 16,
    Trigintaduonion = 32
);
