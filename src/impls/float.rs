//! Gleitkommazahlen als **Modell** des geordneten Körpers `ℝ`.
//!
//! Streng genommen erfüllt `f64` kaum ein Gesetz:
//! * Rundung verletzt Assoziativität und Distributivität.
//! * `NaN` verletzt Reflexivität und Totalität von `≤`.
//! * Überlauf auf `±∞` verletzt die Inversen.
//!
//! Trotzdem *meint* jedes numerische Programm mit `f64` die reellen Zahlen.
//! Ohne diese Deklaration wären Bisektion, Newton und Gauß nicht schreibbar.
//! Deshalb gilt hier der Vertrag:
//!
//! > Für `f64`/`f32` gelten die deklarierten Gesetze **bis auf Rundung** und
//! > nur für endliche Werte (kein `NaN`, kein `±∞`).
//!
//! Die Prüffunktionen `holds` vergleichen exakt und schlagen für Gleitkomma
//! deshalb gelegentlich fehl. Das ist erwartet. Wer exakte Gesetze braucht,
//! nimmt einen exakten Typ (Ganzzahlen, `GF(p)`, rationale Zahlen).

use crate::laws;
use crate::signature::HasRootsOfUnity;
use crate::signature::ScalarMul;
use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasConjugate, HasIdentity, HasInverse, HasPartialInverse,
    LessEq, Multiplicative,
};
#[cfg(any(feature = "std", feature = "libm"))]
use {
    super::fmath::FMath,
    crate::signature::{HasExp, HasLn, HasSinCos, HasSqrt},
};

macro_rules! float {
    ($($t:ty),*) => {$(
        impl BinaryOp<Additive> for $t {
            fn op(&self, rhs: &Self) -> Self { self + rhs }
        }
        impl HasIdentity<Additive> for $t {
            fn identity() -> Self { 0.0 }
        }
        impl HasInverse<Additive> for $t {
            fn inverse(&self) -> Self { -self }
        }
        impl BinaryOp<Multiplicative> for $t {
            fn op(&self, rhs: &Self) -> Self { self * rhs }
        }
        impl HasIdentity<Multiplicative> for $t {
            fn identity() -> Self { 1.0 }
        }
        impl HasPartialInverse<Multiplicative> for $t {
            fn try_inverse(&self) -> Option<Self> { (*self != 0.0).then(|| 1.0 / self) }
        }
        impl BinaryRelation<LessEq> for $t {
            fn relates(&self, other: &Self) -> bool { self <= other }
        }
        impl HasConjugate for $t {
            fn conj(&self) -> Self { *self }
        }
        /// ℝ als Modul über sich selbst: `s · x = s·x`.
        impl ScalarMul<$t> for $t {
            fn scale(&self, s: &$t) -> Self { s * self }
        }
        impl HasRootsOfUnity for $t {
            /// In ℝ gibt es nur `1` und `−1`.
            fn primitive_root_of_unity(n: usize) -> Option<Self> {
                match n {
                    1 => Some(1.0),
                    2 => Some(-1.0),
                    _ => None,
                }
            }
        }
        laws! {
            $t {
                Additive: associative, commutative, identity, inverse, cancellative, conjugate_additive;
                Multiplicative: associative, commutative, identity, conjugation, self_conjugate;
                [Multiplicative, Additive]: distributive, zero_divisor_free, inverse_except_zero, nontrivial;
                LessEq: total_order;
                $t: module;
                [Additive, LessEq]: monotone;
                [Multiplicative, Additive, LessEq]: positive_product;
                Multiplicative: inverse_where_defined, primitive_root_of_unity;
            }
        }
    )*};
}

float!(f32, f64);

/// Elementarfunktionen, nur mit `std` oder `libm`.
#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! float_elementary {
    ($($t:ty),*) => {$(
        impl HasSqrt for $t {
            fn sqrt(&self) -> Option<Self> { (*self >= 0.0).then(|| self.m_sqrt()) }
        }
        impl HasExp for $t {
            fn exp(&self) -> Self { self.m_exp() }
        }
        impl HasLn for $t {
            fn ln(&self) -> Option<Self> { (*self > 0.0).then(|| self.m_ln()) }
        }
        impl HasSinCos for $t {
            fn sin(&self) -> Self { self.m_sin() }
            fn cos(&self) -> Self { self.m_cos() }
        }
        impl crate::signature::HasAtan2 for $t {
            fn atan2(&self, x: &Self) -> Self { self.m_atan2(*x) }
        }

        laws! {
            $t {
                Multiplicative: exp_inverts_ln, sqrt_squares, atan2_inverts_sin_cos;
                [Additive, Multiplicative]: exp_homomorphism;
                [Additive, LessEq]: sqrt_of_non_negative;
                [Multiplicative, Additive]: trigonometric;
            }
        }
    )*};
}

#[cfg(any(feature = "std", feature = "libm"))]
float_elementary!(f32, f64);
