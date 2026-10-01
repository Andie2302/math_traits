//! Beispiel-Impls für Standardtypen.
//!
//! * Ganzzahlen mit **Wrapping**-Arithmetik bilden den Ring `ℤ/2ⁿ`. Mit
//!   `min`/`max` bilden sie einen Verband, mit `<=` eine Totalordnung.
//!   Monotonie von `+` gilt wegen des Überlaufs *nicht* und wird deshalb auch
//!   nicht deklariert.
//! * `bool` mit `xor`/`and` ist der Körper `GF(2)` (hier ein Boolescher
//!   Integritätsbereich), mit `and`/`or` ein Verband.
//! * `f64` bekommt bewusst keine Gesetze: Gleitkomma-Addition ist nicht
//!   assoziativ.

use crate::laws;
use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasIdentity, HasInverse, Join, LessEq, Meet,
    Multiplicative,
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

        laws! {
            $t {
                Additive: associative, commutative, identity, inverse, cancellative;
                Multiplicative: associative, commutative, identity;
                [Multiplicative, Additive]: distributive;
                Meet: associative, commutative, idempotent;
                Join: associative, commutative, idempotent;
                [Meet, Join]: absorption;
                [Join, Meet]: absorption;
                LessEq: total_order;
            }
        }
    )*};
}

wrapping_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

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
        Multiplicative: associative, commutative, identity, idempotent;
        [Multiplicative, Additive]: distributive, zero_divisor_free;
        Meet: associative, commutative, idempotent;
        Join: associative, commutative, idempotent;
        [Meet, Join]: absorption;
        [Join, Meet]: absorption;
        LessEq: total_order;
    }
}
