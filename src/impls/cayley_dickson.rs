//! Cayley-Dickson-Konstruktion: aus einer Algebra mit Konjugation `T` wird
//! `CayleyDickson<T>` = Paare `(a, b)` mit
//!
//! ```text
//! (a, b) + (c, d) = (a + c, b + d)
//! (a, b) · (c, d) = (a·c − d*·b,  d·a + b·c*)
//! (a, b)*         = (a*, −b)
//! ```
//!
//! Die *Signatur* ist generisch und gilt auf jeder Stufe. Die *Gesetze* gehen
//! aber Stufe für Stufe verloren, und genau das bildet die feine Basis ab:
//!
//! | Stufe | Typ | verliert | bleibt |
//! |---|---|---|---|
//! | ℂ | [`Complex`] | Ordnung | Körper |
//! | ℍ | [`Quaternion`] | Kommutativität | Schiefkörper |
//! | 𝕆 | [`Octonion`] | Assoziativität | alternativ, flexibel, Inverse |
//! | 𝕊 | [`Sedenion`] | Alternativität, Nullteilerfreiheit | flexibel, Inverse |
//!
//! Die Gesetze werden pro Stufe und Grundtyp deklariert (`f64`, `f32` und die
//! Ganzzahlen). Ganzzahlen rechnen exakt, darauf lassen sich die Gesetze
//! wirklich prüfen. Die Inversen gibt es nur über Körpern (`f64`, `f32`).

use crate::laws;
use crate::signature::{
    Additive, BinaryOp, HasConjugate, HasExp, HasIdentity, HasInverse, HasLn, HasPartialInverse,
    HasSinCos, HasSqrt, Multiplicative, op,
};

/// Ein Element `(re, im)` der Cayley-Dickson-Verdopplung von `T`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct CayleyDickson<T> {
    pub re: T,
    pub im: T,
}

impl<T> CayleyDickson<T> {
    pub const fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

/// Komplexe Zahlen über `T` (Dimension 2).
pub type Complex<T> = CayleyDickson<T>;
/// Quaternionen über `T` (Dimension 4).
pub type Quaternion<T> = CayleyDickson<Complex<T>>;
/// Oktonionen über `T` (Dimension 8).
pub type Octonion<T> = CayleyDickson<Quaternion<T>>;
/// Sedenionen über `T` (Dimension 16).
pub type Sedenion<T> = CayleyDickson<Octonion<T>>;

impl<T: BinaryOp<Additive>> BinaryOp<Additive> for CayleyDickson<T> {
    fn op(&self, rhs: &Self) -> Self {
        Self::new(
            op::<Additive, _>(&self.re, &rhs.re),
            op::<Additive, _>(&self.im, &rhs.im),
        )
    }
}

impl<T: HasIdentity<Additive>> HasIdentity<Additive> for CayleyDickson<T> {
    fn identity() -> Self {
        Self::new(T::identity(), T::identity())
    }
}

impl<T: HasInverse<Additive>> HasInverse<Additive> for CayleyDickson<T> {
    fn inverse(&self) -> Self {
        Self::new(self.re.inverse(), self.im.inverse())
    }
}

impl<T: HasConjugate + HasInverse<Additive>> HasConjugate for CayleyDickson<T> {
    fn conj(&self) -> Self {
        Self::new(self.re.conj(), self.im.inverse())
    }
}

impl<T> BinaryOp<Multiplicative> for CayleyDickson<T>
where
    T: BinaryOp<Multiplicative> + HasInverse<Additive> + HasConjugate,
{
    /// `(a, b) · (c, d) = (a·c − d*·b,  d·a + b·c*)`
    fn op(&self, rhs: &Self) -> Self {
        let (a, b, c, d) = (&self.re, &self.im, &rhs.re, &rhs.im);
        let mul = op::<Multiplicative, T>;
        let add = op::<Additive, T>;
        Self::new(
            add(&mul(a, c), &mul(&d.conj(), b).inverse()),
            add(&mul(d, a), &mul(b, &c.conj())),
        )
    }
}

impl<T> HasIdentity<Multiplicative> for CayleyDickson<T>
where
    T: HasIdentity<Multiplicative> + HasInverse<Additive> + HasConjugate,
{
    fn identity() -> Self {
        Self::new(
            <T as HasIdentity<Multiplicative>>::identity(),
            <T as HasIdentity<Additive>>::identity(),
        )
    }
}

impl<T> HasPartialInverse<Multiplicative> for CayleyDickson<T>
where
    T: HasPartialInverse<Multiplicative> + HasInverse<Additive> + HasConjugate,
{
    /// `x⁻¹ = x* · n⁻¹` mit der reellen Norm `n = x · x*`.
    fn try_inverse(&self) -> Option<Self> {
        let conj = self.conj();
        let norm = op::<Multiplicative, _>(self, &conj).re;
        let inv = norm.try_inverse()?;
        let real = Self::new(inv, <T as HasIdentity<Additive>>::identity());
        Some(op::<Multiplicative, _>(&conj, &real))
    }
}

/// Deklariert die Gesetze aller vier Stufen über einem Grundtyp.
macro_rules! cayley_dickson_laws {
    ($($b:ty),*) => {$(
        laws! {
            Complex<$b> {
                Additive: associative, commutative, identity, inverse, conjugate_additive;
                Multiplicative: associative, alternative, flexible, commutative, identity, conjugation;
                [Multiplicative, Additive]: distributive;
            }
            Quaternion<$b> {
                Additive: associative, commutative, identity, inverse, conjugate_additive;
                Multiplicative: associative, alternative, flexible, identity, conjugation;
                [Multiplicative, Additive]: distributive;
            }
            Octonion<$b> {
                Additive: associative, commutative, identity, inverse, conjugate_additive;
                Multiplicative: alternative, flexible, identity, conjugation;
                [Multiplicative, Additive]: distributive;
            }
            Sedenion<$b> {
                Additive: associative, commutative, identity, inverse, conjugate_additive;
                Multiplicative: flexible, identity, conjugation;
                [Multiplicative, Additive]: distributive;
            }
        }
    )*};
}

/// Zusätzliche Gesetze, wenn der Grundtyp ein (geordneter) Körper ist.
macro_rules! cayley_dickson_field_laws {
    ($($b:ty),*) => {$(
        laws! {
            Complex<$b> { [Multiplicative, Additive]: inverse_except_zero, nontrivial, zero_divisor_free; }
            Quaternion<$b> { [Multiplicative, Additive]: inverse_except_zero, nontrivial, zero_divisor_free; }
            Octonion<$b> { [Multiplicative, Additive]: inverse_except_zero, nontrivial, zero_divisor_free; }
            // Sedenionen: Inverse ja, aber Nullteiler!
            Sedenion<$b> { [Multiplicative, Additive]: inverse_except_zero, nontrivial; }
        }
    )*};
}

cayley_dickson_laws!(f32, f64, i8, i16, i32, i64, i128, isize);
cayley_dickson_field_laws!(f32, f64);

/// Elementarfunktionen auf ℂ für `f32`/`f64` (Hauptzweige).
macro_rules! complex_elementary {
    ($($b:ty),*) => {$(
        impl HasExp for Complex<$b> {
            /// `exp(a + bi) = eᵃ (cos b + i sin b)`
            fn exp(&self) -> Self {
                let r = <$b>::exp(self.re);
                Complex::new(r * <$b>::cos(self.im), r * <$b>::sin(self.im))
            }
        }
        impl HasLn for Complex<$b> {
            /// `ln z = ln|z| + i·arg z` mit `arg z ∈ (−π, π]`, `None` für `z = 0`.
            fn ln(&self) -> Option<Self> {
                let r = <$b>::hypot(self.re, self.im);
                (r > 0.0).then(|| Complex::new(<$b>::ln(r), <$b>::atan2(self.im, self.re)))
            }
        }
        impl HasSqrt for Complex<$b> {
            /// Hauptwurzel mit `Re ≥ 0`. Auf ℂ überall definiert.
            fn sqrt(&self) -> Option<Self> {
                let r = <$b>::hypot(self.re, self.im);
                let re = <$b>::sqrt((r + self.re) / 2.0);
                let im = <$b>::sqrt((r - self.re) / 2.0);
                Some(Complex::new(re, if self.im < 0.0 { -im } else { im }))
            }
        }
        impl HasSinCos for Complex<$b> {
            fn sin(&self) -> Self {
                Complex::new(
                    <$b>::sin(self.re) * <$b>::cosh(self.im),
                    <$b>::cos(self.re) * <$b>::sinh(self.im),
                )
            }
            fn cos(&self) -> Self {
                Complex::new(
                    <$b>::cos(self.re) * <$b>::cosh(self.im),
                    -<$b>::sin(self.re) * <$b>::sinh(self.im),
                )
            }
        }
        laws! {
            Complex<$b> {
                Multiplicative: exp_inverts_ln, sqrt_squares, inverse_where_defined;
                [Additive, Multiplicative]: exp_homomorphism;
                [Multiplicative, Additive]: trigonometric;
            }
        }
    )*};
}

complex_elementary!(f32, f64);
