//! Beispiel-Impls für Standardtypen.
//!
//! * Ganzzahlen mit **Wrapping**-Arithmetik bilden den Ring `ℤ/2ⁿ`. Mit
//!   `min`/`max` bilden sie einen Verband, mit `<=` eine Totalordnung.
//!   Monotonie von `+` gilt wegen des Überlaufs *nicht* und wird deshalb auch
//!   nicht deklariert. Division mit Rest ist euklidisch (`gcd`).
//! * `bool` mit `xor`/`and` ist der Körper `GF(2)`, mit `and`/`or` ein
//!   Verband.
//! * `f64`/`f32`: siehe [`float`].
//! * Cayley-Dickson (komplexe Zahlen bis Sedenionen): siehe [`cayley_dickson`].
//! * Vektoren `[T; N]`: siehe [`vector`].

pub mod cayley_dickson;
pub mod dual;
pub mod float;
#[cfg(any(feature = "std", feature = "libm"))]
pub(crate) mod fmath;
pub mod vector;

use crate::laws;
use crate::signature::HasRootsOfUnity;
use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasConjugate, HasDivRem, HasEuclideanSize, HasIdentity,
    HasInverse, HasPartialInverse, Join, LessEq, Meet, Multiplicative,
};

macro_rules! wrapping_int {
    ($($t:ty),*) => {$(
        impl BinaryOp<Additive> for $t {
            fn op(&self, rhs: &Self) -> Self { self.wrapping_add(*rhs) }
        }
        impl HasIdentity<Additive> for $t {
            fn identity() -> Self { 0 }
        }
        impl HasInverse<Additive> for $t {
            fn inverse(&self) -> Self { self.wrapping_neg() }
        }
        impl BinaryOp<Multiplicative> for $t {
            fn op(&self, rhs: &Self) -> Self { self.wrapping_mul(*rhs) }
        }
        impl HasIdentity<Multiplicative> for $t {
            fn identity() -> Self { 1 }
        }
        impl BinaryOp<Meet> for $t {
            fn op(&self, rhs: &Self) -> Self { (*self).min(*rhs) }
        }
        impl BinaryOp<Join> for $t {
            fn op(&self, rhs: &Self) -> Self { (*self).max(*rhs) }
        }
        impl BinaryRelation<LessEq> for $t {
            fn relates(&self, other: &Self) -> bool { self <= other }
        }
        impl HasConjugate for $t {
            fn conj(&self) -> Self { *self }
        }
        impl HasRootsOfUnity for $t {
            /// In `ℤ/2ⁿ` gibt es nur `1` und `−1`.
            fn primitive_root_of_unity(n: usize) -> Option<Self> {
                match n {
                    1 => Some(1),
                    2 => Some((0 as $t).wrapping_sub(1)),
                    _ => None,
                }
            }
        }
        impl HasDivRem for $t {
            fn div_rem(&self, d: &Self) -> Option<(Self, Self)> {
                (*d != 0).then(|| (self.wrapping_div(*d), self.wrapping_rem(*d)))
            }
        }

        laws! {
            $t {
                Additive: associative, commutative, identity, inverse, cancellative, conjugate_additive;
                Multiplicative: associative, commutative, identity, conjugation, self_conjugate, primitive_root_of_unity;
                [Multiplicative, Additive]: distributive, euclidean;
                Meet: associative, commutative, idempotent;
                Join: associative, commutative, idempotent;
                [Meet, Join]: absorption;
                [Join, Meet]: absorption;
                LessEq: total_order;
            }
        }
    )*};
}

wrapping_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! euclidean_size {
    (signed: $($s:ty),*; unsigned: $($u:ty),*) => {
        $(impl HasEuclideanSize for $s {
            fn euclidean_size(&self) -> u128 { self.unsigned_abs() as u128 }
        })*
        $(impl HasEuclideanSize for $u {
            fn euclidean_size(&self) -> u128 { *self as u128 }
        })*
    };
}

euclidean_size!(signed: i8, i16, i32, i64, i128, isize; unsigned: u8, u16, u32, u64, u128, usize);

impl BinaryOp<Additive> for bool {
    fn op(&self, rhs: &Self) -> Self {
        self ^ rhs
    }
}
impl HasIdentity<Additive> for bool {
    fn identity() -> Self {
        false
    }
}
impl HasInverse<Additive> for bool {
    fn inverse(&self) -> Self {
        *self
    }
}
impl BinaryOp<Multiplicative> for bool {
    fn op(&self, rhs: &Self) -> Self {
        self & rhs
    }
}
impl HasIdentity<Multiplicative> for bool {
    fn identity() -> Self {
        true
    }
}
impl HasPartialInverse<Multiplicative> for bool {
    fn try_inverse(&self) -> Option<Self> {
        self.then_some(true)
    }
}
impl BinaryOp<Meet> for bool {
    fn op(&self, rhs: &Self) -> Self {
        self & rhs
    }
}
impl BinaryOp<Join> for bool {
    fn op(&self, rhs: &Self) -> Self {
        self | rhs
    }
}
impl BinaryRelation<LessEq> for bool {
    fn relates(&self, other: &Self) -> bool {
        !self | other
    }
}

laws! {
    bool {
        Additive: associative, commutative, identity, inverse, cancellative;
        Multiplicative: associative, commutative, identity, idempotent, inverse_where_defined;
        [Multiplicative, Additive]: distributive, zero_divisor_free, inverse_except_zero, nontrivial;
        Meet: associative, commutative, idempotent;
        Join: associative, commutative, idempotent;
        [Meet, Join]: absorption;
        [Join, Meet]: absorption;
        LessEq: total_order;
    }
}
