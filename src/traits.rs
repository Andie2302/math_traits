use crate::scalar::Scalar;
use core::ops::{Add, Div, Mul, Sub};

/// Das triviale Nullelement existiert auf jeder Cayley-Dickson-Stufe.
pub trait TrivialZero: Sized {
    /// Das Nullelement (alle Koeffizienten null).
    fn zero() -> Self;
    /// `true`, wenn alle Koeffizienten null sind.
    fn is_zero(&self) -> bool;
}

/// Stufen, in denen nichttriviale Nullteiler auftreten (ab `Sedenion`):
/// es gibt `a != 0`, `b != 0` mit `a * b == 0`.
pub trait NonTrivialZero: TrivialZero + CayleyDickson {
    /// Ein konkretes Paar `(a, b)` mit `a != 0`, `b != 0` und `a * b == 0`.
    fn zero_divisor_pair() -> (Self, Self);

    /// `true`, wenn `a` und `b` beide ungleich null sind, aber `a * b == 0`.
    fn is_nontrivial_zero_product(a: &Self, b: &Self) -> bool {
        !a.is_zero() && !b.is_zero() && (*a * *b).is_zero()
    }
}

/// Gemeinsames Trait aller Stufen `Real<T>` bis `Trigintaduonion<T>`.
///
/// `-` ist immer vorhanden. Negation (`Neg`) und Division (`Div`) haengen
/// vom Skalartyp bzw. der Stufe ab und sind deshalb separate Bounds.
pub trait CayleyDickson:
    TrivialZero
    + Copy
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
{
    /// Koeffiziententyp.
    type Scalar: Scalar;

    /// Anzahl der Verdopplungen (Real = 0, Complex = 1, ..., Trigintaduonion = 5).
    const LEVEL: usize;
    /// Dimension ueber dem Skalar: `2^LEVEL`.
    const DIM: usize;

    /// `a*b == b*a` fuer alle a, b (Level <= 1).
    const COMMUTATIVE: bool = Self::LEVEL <= 1;
    /// `(a*b)*c == a*(b*c)` (Level <= 2).
    const ASSOCIATIVE: bool = Self::LEVEL <= 2;
    /// `(a*a)*b == a*(a*b)` und `(b*a)*a == b*(a*a)` (Level <= 3).
    const ALTERNATIVE: bool = Self::LEVEL <= 3;
    /// `(a*b)*a == a*(b*a)` (alle Stufen).
    const FLEXIBLE: bool = true;
    /// `a^m * a^n == a^(m+n)` (alle Stufen).
    const POWER_ASSOCIATIVE: bool = true;
    /// `N(a*b) == N(a)*N(b)` (Hurwitz, Level <= 3).
    const MULTIPLICATIVE_NORM: bool = Self::LEVEL <= 3;
    /// Nichttriviale Nullteiler moeglich (Level >= 4).
    const HAS_ZERO_DIVISORS: bool = Self::LEVEL >= 4;

    /// Einselement.
    fn one() -> Self;
    /// Einbettung eines Skalars.
    fn from_scalar(s: Self::Scalar) -> Self;
    /// Konjugation.
    fn conjugate(self) -> Self;
    /// Realteil (Koeffizient 0).
    fn real(self) -> Self::Scalar;
    /// Quadrierte Norm `a * conj(a)` als Skalar (Summe der Koeffizientenquadrate).
    fn norm_sqr(self) -> Self::Scalar;
    /// Koeffizient `i < DIM` (Basis `e0 = 1, e1, ...`).
    fn coeff(&self, i: usize) -> Self::Scalar;
    /// Baut ein Element aus `f(0), f(1), .., f(DIM - 1)`.
    fn from_fn(f: impl FnMut(usize) -> Self::Scalar) -> Self;
    /// Alle Koeffizienten mit einem Skalar multiplizieren.
    fn scale(self, s: Self::Scalar) -> Self;

    /// Basiselement `e_i`.
    fn basis(i: usize) -> Self {
        Self::from_fn(|j| if i == j { Self::Scalar::ONE } else { Self::Scalar::ZERO })
    }

    /// Inverses `conj(a) / N(a)`, falls `N(a)` im Skalartyp invertierbar ist
    /// (Floats: `N(a) != 0`; Ganzzahlen: `N(a) == 1`).
    ///
    /// Es gilt `a * a.inverse() == 1`. Ab `Sedenion` ist das KEIN
    /// Divisions-Inverses: wegen der Nullteiler kann man damit nicht
    /// eindeutig dividieren.
    fn inverse(self) -> Option<Self> {
        self.norm_sqr()
            .checked_recip()
            .map(|r| self.conjugate().scale(r))
    }
}

/// Kommutativ: `a*b == b*a`.
pub trait Commutative: CayleyDickson {}
/// Assoziativ: `(a*b)*c == a*(b*c)`.
pub trait Associative: CayleyDickson {}
/// Alternativ: `(a*a)*b == a*(a*b)`, `(b*a)*a == b*(a*a)`.
pub trait Alternative: CayleyDickson {}
/// Flexibel: `(a*b)*a == a*(b*a)`.
pub trait Flexible: CayleyDickson {}
/// Potenzassoziativ: `a^m * a^n == a^(m+n)`.
pub trait PowerAssociative: CayleyDickson {}
/// Multiplikative Norm (Kompositionsalgebra): `N(a*b) == N(a)*N(b)`.
pub trait MultiplicativeNorm: CayleyDickson {}

/// Normierte Divisionsalgebra (nur Real, Complex, Quaternion, Octonion
/// ueber Float-Skalaren): keine Nullteiler, `/` ist `a * b^-1`.
pub trait DivisionAlgebra: CayleyDickson + MultiplicativeNorm + Div<Output = Self> {
    /// Kehrwert `1 / a` (`conj(a) / N(a)`; bei `a == 0` ergibt sich inf/NaN).
    fn recip(self) -> Self {
        Self::one() / self
    }
    /// Linksdivision `a^-1 * b`, die `a * x = b` loest (bei nichtkommutativen Stufen
    /// verschieden von `/`, das `x * a = b` loest).
    fn left_div(self, rhs: Self) -> Self {
        self.recip() * rhs
    }
}
