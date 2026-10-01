//! Signatur: *was es gibt*, nicht *was gilt*.
//!
//! Hier stehen nur Operationen, ausgezeichnete Elemente und Relationen. Diese
//! Traits tragen echte Daten bzw. Funktionen und behaupten nichts. Dass etwa
//! [`HasIdentity::identity`] wirklich neutral ist, sagt erst ein Gesetz aus
//! [`crate::laws`].
//!
//! Operationen werden über Marker-Typen unterschieden, damit ein Typ mehrere
//! Operationen derselben Form tragen kann (z. B. `+` und `·`, oder `∧` und `∨`).

// ===========================================================================
// Marker für Operationen und Relationen
// ===========================================================================

/// Marker: additive Operation `+`.
pub enum Additive {}

/// Marker: multiplikative Operation `·`.
pub enum Multiplicative {}

/// Marker: Infimum `∧` (Verbände).
pub enum Meet {}

/// Marker: Supremum `∨` (Verbände).
pub enum Join {}

/// Marker: Ordnungsrelation `≤`.
pub enum LessEq {}

// ===========================================================================
// Operationen und ausgezeichnete Elemente
// ===========================================================================

/// Eine totale binäre Operation `•` auf `Self`.
pub trait BinaryOp<Op>: Sized {
    fn op(&self, rhs: &Self) -> Self;
}

/// Ein ausgezeichnetes Element `e` für die Operation `Op`.
pub trait HasIdentity<Op>: BinaryOp<Op> {
    fn identity() -> Self;
}

/// Eine einstellige Operation `x ↦ x⁻¹` für die Operation `Op`.
pub trait HasInverse<Op>: HasIdentity<Op> {
    fn inverse(&self) -> Self;
}

/// Eine *partielle* einstellige Operation `x ↦ x⁻¹`: `None`, wo es kein
/// Inverses gibt (z. B. für `0` bei der Multiplikation eines Körpers).
pub trait HasPartialInverse<Op>: HasIdentity<Op> {
    fn try_inverse(&self) -> Option<Self>;
}

/// Ein ausgezeichnetes Element `z` für die Operation `Op`.
pub trait HasAbsorbing<Op>: BinaryOp<Op> {
    fn absorbing() -> Self;
}

// ===========================================================================
// Relationen
// ===========================================================================

/// Eine binäre Relation `R` auf `Self`.
pub trait BinaryRelation<R> {
    fn relates(&self, other: &Self) -> bool;
}

/// Wendet `Op` an, ohne Mehrdeutigkeit, wenn ein Typ mehrere Operationen hat.
#[inline]
pub fn op<Op, T: BinaryOp<Op>>(a: &T, b: &T) -> T {
    <T as BinaryOp<Op>>::op(a, b)
}

// ===========================================================================
// Weitere Operationen
// ===========================================================================

/// Konjugation `x ↦ x*` (z. B. komplexe Konjugation; für reelle Zahlen die
/// Identität). Grundlage der Cayley-Dickson-Konstruktion.
pub trait HasConjugate: Sized {
    fn conj(&self) -> Self;
}

/// Division mit Rest: `a.div_rem(b) = Some((q, r))`, `None` für `b = 0`.
pub trait HasDivRem: Sized {
    fn div_rem(&self, divisor: &Self) -> Option<(Self, Self)>;
}

/// Eine Größe in `ℕ`, die bei Division mit Rest für den Rest schrumpft.
pub trait HasEuclideanSize {
    fn euclidean_size(&self) -> u128;
}

/// Externe Operation `S × V → V`: Skalar mal Vektor, `s · v`.
pub trait ScalarMul<S>: Sized {
    fn scale(&self, s: &S) -> Self;
}
