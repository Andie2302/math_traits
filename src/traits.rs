use crate::scalar::Scalar;
use core::ops::{Add, Div, Mul, Sub};

/// Das triviale Nullelement existiert auf jeder Cayley-Dickson-Stufe.
pub trait TrivialZero: Sized {
    /// Das Nullelement (alle Koeffizienten null).
    fn zero() -> Self;
    /// `true`, wenn alle Koeffizienten null sind.
    fn is_zero(&self) -> bool;
}

/// Algebren mit nichttrivialen Nullteilern (z.B. Cayley-Dickson ab `Sedenion`):
/// es gibt `a != 0`, `b != 0` mit `a * b == 0`.
pub trait NonTrivialZero: TrivialZero + Algebra {
    /// Ein konkretes Paar `(a, b)` mit `a != 0`, `b != 0` und `a * b == 0`.
    fn zero_divisor_pair() -> (Self, Self);

    /// `true`, wenn `a` und `b` beide ungleich null sind, aber `a * b == 0`.
    fn is_nontrivial_zero_product(a: &Self, b: &Self) -> bool {
        !a.is_zero() && !b.is_zero() && (*a * *b).is_zero()
    }
}

/// Gemeinsame Basis aller Algebren dieser Bibliothek (Cayley-Dickson- und Clifford-Algebren).
///
/// Eine Algebra ueber dem Skalar [`Algebra::Scalar`] mit Dimension [`Algebra::DIM`],
/// Eins, `+`, `-` und `*`. Negation (`Neg`) und Division (`Div`) haengen vom Skalartyp
/// bzw. der Algebra ab und sind deshalb separate Bounds.
pub trait Algebra:
    TrivialZero + Copy + PartialEq + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>
{
    /// Koeffiziententyp.
    type Scalar: Scalar;

    /// Dimension ueber dem Skalar.
    const DIM: usize;

    /// `a*b == b*a` fuer alle a, b.
    const COMMUTATIVE: bool;
    /// `(a*b)*c == a*(b*c)`.
    const ASSOCIATIVE: bool;
    /// `(a*a)*b == a*(a*b)` und `(b*a)*a == b*(a*a)`.
    const ALTERNATIVE: bool;
    /// `(a*b)*a == a*(b*a)`.
    const FLEXIBLE: bool;
    /// `a^m * a^n == a^(m+n)`.
    const POWER_ASSOCIATIVE: bool;
    /// `N(a*b) == N(a)*N(b)` (Kompositionsalgebra).
    const MULTIPLICATIVE_NORM: bool;
    /// Es gibt nichttriviale Nullteiler.
    const HAS_ZERO_DIVISORS: bool;

    /// Einselement.
    fn one() -> Self;
    /// Einbettung eines Skalars.
    fn from_scalar(s: Self::Scalar) -> Self;
    /// Koeffizient `i < DIM` bezueglich der Standardbasis.
    fn coeff(&self, i: usize) -> Self::Scalar;
    /// Baut ein Element aus `f(0), f(1), .., f(DIM - 1)`.
    fn from_fn(f: impl FnMut(usize) -> Self::Scalar) -> Self;
    /// Alle Koeffizienten mit einem Skalar multiplizieren.
    fn scale(self, s: Self::Scalar) -> Self;

    /// Basiselement `e_i`.
    fn basis(i: usize) -> Self {
        Self::from_fn(|j| {
            if i == j {
                Self::Scalar::ONE
            } else {
                Self::Scalar::ZERO
            }
        })
    }

    /// Zweiseitiges Inverses, falls es sich im Skalartyp bilden laesst.
    ///
    /// Cayley-Dickson: `conj(a) / N(a)`, falls `N(a)` invertierbar ist. Ab `Sedenion`
    /// ist das KEIN Divisions-Inverses (Nullteiler). Clifford: `reverse(a) / (a * reverse(a))`,
    /// falls dieses Produkt ein invertierbarer Skalar ist.
    fn inverse(self) -> Option<Self>;
}

/// Gemeinsames Trait aller Stufen `Real<T>` bis `Trigintaduonion<T>`.
pub trait CayleyDickson: Algebra {
    /// Anzahl der Verdopplungen (Real = 0, Complex = 1, ..., Trigintaduonion = 5);
    /// `DIM == 2^LEVEL`.
    const LEVEL: usize;

    /// Konjugation.
    fn conjugate(self) -> Self;
    /// Realteil (Koeffizient 0).
    fn real(self) -> Self::Scalar;
    /// Quadrierte Norm `a * conj(a)` als Skalar (Summe der Koeffizientenquadrate).
    fn norm_sqr(self) -> Self::Scalar;
}

/// Kommutativ: `a*b == b*a`.
pub trait Commutative: Algebra {}
/// Assoziativ: `(a*b)*c == a*(b*c)`.
pub trait Associative: Algebra {}
/// Alternativ: `(a*a)*b == a*(a*b)`, `(b*a)*a == b*(a*a)`.
pub trait Alternative: Algebra {}
/// Flexibel: `(a*b)*a == a*(b*a)`.
pub trait Flexible: Algebra {}
/// Potenzassoziativ: `a^m * a^n == a^(m+n)`.
pub trait PowerAssociative: Algebra {}
/// Multiplikative Norm (Kompositionsalgebra): `N(a*b) == N(a)*N(b)`.
pub trait MultiplicativeNorm: Algebra {}

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
