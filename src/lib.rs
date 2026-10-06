//! Cayley-Dickson-Konstruktion als Traits, `no_std` und ohne `alloc`.
//!
//! Die Stufen sind `Real<T>` (Level 0), `Complex<T>`, `Quaternion<T>`,
//! `Octonion<T>`, `Sedenion<T>` und `Trigintaduonion<T>` (Level 5). Jede
//! Stufe ist `Cd<Vorgaenger>`, ein Paar aus zwei Elementen der Vorstufe mit
//!
//! ```text
//! (a, b) * (c, d) = (a*c - conj(d)*b,  d*a + b*conj(c))
//! conj((a, b))    = (conj(a), -b)
//! ```
//!
//! # Welche Typen gehen wo?
//!
//! * `Real<T>` gibt es fuer alle `u8..u128`, `usize`, `i8..i128`, `isize`,
//!   `f32`, `f64` (und mit Feature `f16` / `f128` auf Nightly auch dafuer).
//! * `Complex<T>` und hoeher brauchen `Neg`, also nur vorzeichenbehaftete
//!   Ganzzahlen und Floats. Unsigned-Typen bleiben bei `Real<uN>`.
//! * `/` gibt es fuer `Real<T>` (bei Ganzzahlen ganzzahlig, panikt bei 0)
//!   und fuer `Complex`, `Quaternion`, `Octonion` mit Float-Skalaren
//!   (`Div` = Multiplikation mit dem Inversen von rechts). Ab `Sedenion`
//!   existiert keine Division, nur [`CayleyDickson::inverse`].
//!
//! # Axiome pro Stufe
//!
//! | Stufe          | kommutativ | assoziativ | alternativ | flexibel | Nullteiler |
//! |----------------|:---:|:---:|:---:|:---:|:---:|
//! | Real           | ja  | ja  | ja  | ja  | nein |
//! | Complex        | ja  | ja  | ja  | ja  | nein |
//! | Quaternion     | nein| ja  | ja  | ja  | nein |
//! | Octonion       | nein| nein| ja  | ja  | nein |
//! | Sedenion       | nein| nein| nein| ja  | ja   |
//! | Trigintaduonion| nein| nein| nein| ja  | ja   |
//!
//! Alle Stufen sind zusaetzlich potenzassoziativ. Die Axiome sind als
//! Marker-Traits ([`Commutative`], [`Associative`], [`Alternative`],
//! [`Flexible`], [`PowerAssociative`], [`MultiplicativeNorm`],
//! [`DivisionAlgebra`]) und als Konstanten in [`CayleyDickson`] verfuegbar.
//!
//! # Overflow
//!
//! Die Operatoren folgen dem Verhalten des Skalartyps: Ganzzahlen panikken
//! in Debug-Builds bei Ueberlauf, in Release wrappen sie.
#![no_std]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]
#![forbid(unsafe_code)]

mod algebra;
mod doubled;
mod real;
mod scalar;
mod tensor;
mod traits;

pub use algebra::{Complex, Octonion, Quaternion, Sedenion, Trigintaduonion};
pub use doubled::Cd;
pub use real::Real;
pub use scalar::{Field, Scalar, Signed};
pub use tensor::*;
pub use traits::{
    Alternative, Associative, CayleyDickson, Commutative, DivisionAlgebra, Flexible,
    MultiplicativeNorm, NonTrivialZero, PowerAssociative, TrivialZero,
};
