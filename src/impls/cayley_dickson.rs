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
use crate::signature::HasRootsOfUnity;
use crate::signature::ScalarMul;
use crate::signature::{
    Additive, BinaryOp, HasConjugate, HasIdentity, HasInverse, HasPartialInverse, Multiplicative,
    op,
};
use crate::structures::{Module, Ring};
#[cfg(any(feature = "std", feature = "libm"))]
use {super::fmath::FMath, crate::signature::HasSqrt};

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

/// Skalare des Grundtyps wirken auf beide Hälften: `s·(a, b) = (s·a, s·b)`.
impl<S, T: ScalarMul<S>> ScalarMul<S> for CayleyDickson<T> {
    fn scale(&self, s: &S) -> Self {
        Self::new(self.re.scale(s), self.im.scale(s))
    }
}

laws! {
    for[S: Ring, T: Module<S>] CayleyDickson<T> {
        S: module;
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

/// Gaußsche Zahlen: Einheitswurzeln `1`, `−1`, `i` (Ordnung 1, 2, 4).
macro_rules! gaussian_roots {
    ($($b:ty),*) => {$(
        impl HasRootsOfUnity for Complex<$b> {
            fn primitive_root_of_unity(n: usize) -> Option<Self> {
                match n {
                    1 => Some(Complex::new(1, 0)),
                    2 => Some(Complex::new(-1, 0)),
                    4 => Some(Complex::new(0, 1)),
                    _ => None,
                }
            }
        }
        laws! { Complex<$b> { Multiplicative: primitive_root_of_unity; } }
    )*};
}

gaussian_roots!(i8, i16, i32, i64, i128, isize);
cayley_dickson_field_laws!(f32, f64);

/// `exp`, `ln`, `sin`, `cos` auf ℂ = `CayleyDickson<T>` über einem **reellen**
/// Grundtyp `T` (Gesetz [`SelfConjugate`]). Das Gesetz im Bound verhindert,
/// dass die Formeln auf Quaternionen angewendet werden, wo sie falsch wären.
/// Generisch über `T`, deshalb funktioniert auch `Complex<Dual<f64>>`
/// (AutoDiff durch komplexe Funktionen).
mod generic_elementary {
    use super::Complex;
    use crate::laws::SelfConjugate;
    use crate::signature::{
        Additive, BinaryOp, HasAtan2, HasExp, HasIdentity, HasInverse, HasLn, HasPartialInverse,
        HasSinCos, Multiplicative, op,
    };

    fn add<T: BinaryOp<Additive>>(a: &T, b: &T) -> T {
        op::<Additive, _>(a, b)
    }
    fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
        op::<Multiplicative, _>(a, b)
    }
    /// `1/2` im Grundtyp, `None` wenn `2` nicht invertierbar ist.
    fn half<T: HasPartialInverse<Multiplicative> + BinaryOp<Additive>>() -> Option<T> {
        let one = <T as HasIdentity<Multiplicative>>::identity();
        add(&one, &one).try_inverse()
    }

    impl<T> HasExp for Complex<T>
    where
        T: SelfConjugate<Multiplicative> + HasExp + HasSinCos + BinaryOp<Multiplicative>,
    {
        /// `exp(a + bi) = eᵃ (cos b + i sin b)`
        fn exp(&self) -> Self {
            let r = self.re.exp();
            Complex::new(mul(&r, &self.im.cos()), mul(&r, &self.im.sin()))
        }
    }

    impl<T> HasLn for Complex<T>
    where
        T: SelfConjugate<Multiplicative>
            + HasLn
            + HasAtan2
            + BinaryOp<Multiplicative>
            + BinaryOp<Additive>
            + HasPartialInverse<Multiplicative>,
    {
        /// `ln z = ½·ln(a² + b²) + i·atan2(b, a)`, `None` für `z = 0`.
        fn ln(&self) -> Option<Self> {
            let r2 = add(&mul(&self.re, &self.re), &mul(&self.im, &self.im));
            let ln_r = mul(&r2.ln()?, &half::<T>()?);
            Some(Complex::new(ln_r, self.im.atan2(&self.re)))
        }
    }

    impl<T> HasSinCos for Complex<T>
    where
        T: SelfConjugate<Multiplicative>
            + HasExp
            + HasSinCos
            + BinaryOp<Multiplicative>
            + BinaryOp<Additive>
            + HasInverse<Additive>
            + HasPartialInverse<Multiplicative>,
    {
        /// `sin(a + bi) = sin a·cosh b + i·cos a·sinh b`
        fn sin(&self) -> Self {
            let (ch, sh) = cosh_sinh(&self.im);
            Complex::new(mul(&self.re.sin(), &ch), mul(&self.re.cos(), &sh))
        }
        /// `cos(a + bi) = cos a·cosh b − i·sin a·sinh b`
        fn cos(&self) -> Self {
            let (ch, sh) = cosh_sinh(&self.im);
            Complex::new(mul(&self.re.cos(), &ch), mul(&self.re.sin(), &sh).inverse())
        }
    }

    /// `(cosh b, sinh b) = ((eᵇ + e⁻ᵇ)/2, (eᵇ − e⁻ᵇ)/2)`
    fn cosh_sinh<T>(b: &T) -> (T, T)
    where
        T: HasExp
            + BinaryOp<Multiplicative>
            + BinaryOp<Additive>
            + HasInverse<Additive>
            + HasPartialInverse<Multiplicative>,
    {
        let h = half::<T>().expect("2 ist invertierbar");
        let (ep, em) = (b.exp(), b.inverse().exp());
        (mul(&add(&ep, &em), &h), mul(&add(&ep, &em.inverse()), &h))
    }
}

/// Wurzel und Einheitswurzeln auf ℂ für `f32`/`f64` (Hauptzweige), dazu die
/// Gesetze der Elementarfunktionen.
#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! complex_elementary {
    ($($b:ty),*) => {$(
        impl HasSqrt for Complex<$b> {
            /// Hauptwurzel mit `Re ≥ 0`. Auf ℂ überall definiert.
            fn sqrt(&self) -> Option<Self> {
                let r = <$b as FMath>::m_hypot(self.re, self.im);
                let re = <$b as FMath>::m_sqrt((r + self.re) / 2.0);
                let im = <$b as FMath>::m_sqrt((r - self.re) / 2.0);
                Some(Complex::new(re, if self.im < 0.0 { -im } else { im }))
            }
        }
        impl HasRootsOfUnity for Complex<$b> {
            /// `ω = e^(2πi/n)`
            fn primitive_root_of_unity(n: usize) -> Option<Self> {
                if n == 0 {
                    return None;
                }
                let phi = 2.0 * core::f64::consts::PI as $b / n as $b;
                Some(Complex::new(<$b as FMath>::m_cos(phi), <$b as FMath>::m_sin(phi)))
            }
        }
        laws! {
            Complex<$b> {
                Multiplicative: exp_inverts_ln, sqrt_squares, inverse_where_defined, primitive_root_of_unity;
                [Additive, Multiplicative]: exp_homomorphism;
                [Multiplicative, Additive]: trigonometric;
            }
        }
    )*};
}

#[cfg(any(feature = "std", feature = "libm"))]
complex_elementary!(f32, f64);
