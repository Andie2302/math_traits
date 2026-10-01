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
use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasConjugate, HasExp, HasIdentity, HasInverse, HasLn,
    HasPartialInverse, HasSinCos, HasSqrt, LessEq, Multiplicative,
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
        impl HasSqrt for $t {
            fn sqrt(&self) -> Option<Self> { (*self >= 0.0).then(|| <$t>::sqrt(*self)) }
        }
        impl HasExp for $t {
            fn exp(&self) -> Self { <$t>::exp(*self) }
        }
        impl HasLn for $t {
            fn ln(&self) -> Option<Self> { (*self > 0.0).then(|| <$t>::ln(*self)) }
        }
        impl HasSinCos for $t {
            fn sin(&self) -> Self { <$t>::sin(*self) }
            fn cos(&self) -> Self { <$t>::cos(*self) }
        }

        laws! {
            $t {
                Additive: associative, commutative, identity, inverse, cancellative;
                Multiplicative: associative, commutative, identity, conjugation;
                [Multiplicative, Additive]: distributive, zero_divisor_free, inverse_except_zero, nontrivial;
                LessEq: total_order;
                [Additive, LessEq]: monotone;
                [Multiplicative, Additive, LessEq]: positive_product;
                Multiplicative: exp_inverts_ln, sqrt_squares, inverse_where_defined;
                [Additive, Multiplicative]: exp_homomorphism;
                [Additive, LessEq]: sqrt_of_non_negative;
                [Multiplicative, Additive]: trigonometric;
            }
        }
    )*};
}

float!(f32, f64);
