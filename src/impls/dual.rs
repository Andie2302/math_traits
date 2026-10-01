//! Duale Zahlen `a + b·ε` mit `ε² = 0`: Vorwärts-AutoDiff.
//!
//! Wertet man eine Funktion `f` bei `x + 1·ε` aus, erhält man
//! `f(x) + f'(x)·ε`. Die Ableitung fällt also automatisch mit ab. Das gilt
//! für jede Formel, die nur Ring-Operationen und Elementarfunktionen benutzt,
//! weil jede Elementarfunktion hier die Kettenregel umsetzt.
//!
//! Die Gesetze sind bedingt: `Dual<T>` ist ein kommutativer Ring, wenn `T`
//! einer ist, und erbt die Elementarfunktions-Gesetze von `T`. Deshalb
//! funktioniert auch `Dual<Dual<f64>>` für zweite Ableitungen.
//!
//! `Dual<T>` ist **kein Körper**: `ε` hat kein Inverses. Und es ist **nicht
//! geordnet**. Genau das sagt die Struktur ([`ElementaryRing`], nicht
//! [`RealField`](crate::structures::RealField)).

use crate::laws;
use crate::laws::InverseWhereDefined;
use crate::signature::{
    Additive, BinaryOp, HasExp, HasIdentity, HasInverse, HasLn, HasPartialInverse, HasSinCos,
    HasSqrt, Multiplicative, op,
};
use crate::structures::{CommutativeRing, ElementaryRing};

/// `re + eps·ε`
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Dual<T> {
    pub re: T,
    pub eps: T,
}

impl<T> Dual<T> {
    pub const fn new(re: T, eps: T) -> Self {
        Self { re, eps }
    }
}

impl<T: HasIdentity<Additive>> Dual<T> {
    /// Eine Konstante (Ableitung 0).
    pub fn constant(re: T) -> Self {
        Self::new(re, T::identity())
    }
}

impl<T: HasIdentity<Multiplicative>> Dual<T> {
    /// Die Variable, nach der abgeleitet wird (Ableitung 1).
    pub fn variable(re: T) -> Self {
        Self::new(re, T::identity())
    }
}

fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}

fn add<T: BinaryOp<Additive>>(a: &T, b: &T) -> T {
    op::<Additive, _>(a, b)
}

// --- Ring -------------------------------------------------------------------

impl<T: BinaryOp<Additive>> BinaryOp<Additive> for Dual<T> {
    fn op(&self, rhs: &Self) -> Self {
        Self::new(add(&self.re, &rhs.re), add(&self.eps, &rhs.eps))
    }
}

impl<T: HasIdentity<Additive>> HasIdentity<Additive> for Dual<T> {
    fn identity() -> Self {
        Self::new(T::identity(), T::identity())
    }
}

impl<T: HasInverse<Additive>> HasInverse<Additive> for Dual<T> {
    fn inverse(&self) -> Self {
        Self::new(self.re.inverse(), self.eps.inverse())
    }
}

impl<T: BinaryOp<Multiplicative> + BinaryOp<Additive>> BinaryOp<Multiplicative> for Dual<T> {
    /// `(a + bε)(c + dε) = ac + (ad + bc)ε`
    fn op(&self, rhs: &Self) -> Self {
        Self::new(
            mul(&self.re, &rhs.re),
            add(&mul(&self.re, &rhs.eps), &mul(&self.eps, &rhs.re)),
        )
    }
}

impl<T> HasIdentity<Multiplicative> for Dual<T>
where
    T: HasIdentity<Multiplicative> + BinaryOp<Additive> + HasIdentity<Additive>,
{
    fn identity() -> Self {
        Self::new(
            <T as HasIdentity<Multiplicative>>::identity(),
            <T as HasIdentity<Additive>>::identity(),
        )
    }
}

impl<T> HasPartialInverse<Multiplicative> for Dual<T>
where
    T: HasPartialInverse<Multiplicative> + HasInverse<Additive>,
{
    /// `(a + bε)⁻¹ = a⁻¹ − b·a⁻²·ε`, nur wenn `a` invertierbar ist.
    fn try_inverse(&self) -> Option<Self> {
        let inv = self.re.try_inverse()?;
        let eps = mul(&mul(&self.eps, &inv), &inv).inverse();
        Some(Self::new(inv, eps))
    }
}

// --- Elementarfunktionen: Kettenregel ----------------------------------------

/// `f(a + bε) = f(a) + b·f'(a)·ε`
fn chain<T: BinaryOp<Multiplicative>>(value: T, derivative: &T, eps: &T) -> Dual<T> {
    Dual::new(value, mul(eps, derivative))
}

impl<T: HasExp + BinaryOp<Multiplicative>> HasExp for Dual<T> {
    fn exp(&self) -> Self {
        let e = self.re.exp();
        let d = mul(&e, &self.eps);
        Self::new(e, d)
    }
}

impl<T: HasLn + HasPartialInverse<Multiplicative>> HasLn for Dual<T> {
    /// `ln(a + bε) = ln a + (b / a)·ε`
    fn ln(&self) -> Option<Self> {
        let value = self.re.ln()?;
        let inv = self.re.try_inverse()?;
        Some(chain(value, &inv, &self.eps))
    }
}

impl<T> HasSqrt for Dual<T>
where
    T: HasSqrt + HasPartialInverse<Multiplicative> + BinaryOp<Additive>,
{
    /// `sqrt(a + bε) = s + (b / 2s)·ε` mit `s = sqrt(a)`; `None` für `s = 0`.
    fn sqrt(&self) -> Option<Self> {
        let s = self.re.sqrt()?;
        let inv_2s = add(&s, &s).try_inverse()?;
        Some(chain(s, &inv_2s, &self.eps))
    }
}

impl<T: HasSinCos + BinaryOp<Multiplicative> + HasInverse<Additive>> HasSinCos for Dual<T> {
    fn sin(&self) -> Self {
        chain(self.re.sin(), &self.re.cos(), &self.eps)
    }
    fn cos(&self) -> Self {
        chain(self.re.cos(), &self.re.sin().inverse(), &self.eps)
    }
}

laws! {
    for[T: CommutativeRing] Dual<T> {
        Additive: associative, commutative, identity, inverse;
        Multiplicative: associative, alternative, flexible, commutative, identity;
        [Multiplicative, Additive]: distributive;
    }
    for[T: CommutativeRing + InverseWhereDefined<Multiplicative>] Dual<T> {
        Multiplicative: inverse_where_defined;
    }
    for[T: ElementaryRing + InverseWhereDefined<Multiplicative>] Dual<T> {
        Multiplicative: exp_inverts_ln, sqrt_squares;
        [Additive, Multiplicative]: exp_homomorphism;
        [Multiplicative, Additive]: trigonometric;
    }
}
