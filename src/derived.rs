//! Folgerungen: generische Operationen, die *nur* Struktur-Bounds verwenden.
//!
//! Hier entsteht der Nutzen der Hierarchie. Diese Schicht führt keine neuen
//! Annahmen ein: Jede Funktion ist korrekt allein aufgrund der Gesetze, die
//! ihr Bound verlangt.
//!
//! Faustregel: Braucht eine Funktion hier etwas, das sich nicht als Bound aus
//! [`crate::structures`] ausdrücken lässt, fehlt ein Gesetz oder eine Signatur
//! in der Basis.

use crate::laws::InverseWhereDefined;
use crate::signature::{Additive, HasIdentity, HasInverse, HasPartialInverse, Multiplicative, op};
use crate::structures::{
    CommutativeMonoid, DivisionRing, ElementaryRing, EuclideanRing, Group, Monoid, OrderedField,
};

/// `xⁿ` per Square-and-Multiply in `O(log n)` Schritten.
///
/// Braucht [`Monoid`]: `x⁰ = e` verlangt das Einselement, und die
/// Umklammerung beim Quadrieren ist nur mit Assoziativität korrekt.
pub fn pow<Op, T: Monoid<Op>>(x: &T, mut n: u64) -> T {
    let mut acc = <T as HasIdentity<Op>>::identity();
    // `x • e` statt `clone()`: so braucht `T` kein `Clone`.
    let mut base = op::<Op, _>(x, &<T as HasIdentity<Op>>::identity());
    while n > 0 {
        if n & 1 == 1 {
            acc = op::<Op, _>(&acc, &base);
        }
        n >>= 1;
        if n > 0 {
            base = op::<Op, _>(&base, &base);
        }
    }
    acc
}

/// `xⁿ` für ganzzahlige `n`. Mit `x⁻ⁿ = (x⁻¹)ⁿ` ist das nur in einer [`Group`] sinnvoll.
pub fn pow_signed<Op, T: Group<Op>>(x: &T, n: i64) -> T {
    if n >= 0 {
        pow::<Op, _>(x, n.unsigned_abs())
    } else {
        pow::<Op, _>(&<T as HasInverse<Op>>::inverse(x), n.unsigned_abs())
    }
}

/// `x₁ • x₂ • … • xₙ`, leeres Produkt = `e`. Die Reihenfolge bleibt erhalten.
pub fn fold<Op, T: Monoid<Op>>(items: impl IntoIterator<Item = T>) -> T {
    items
        .into_iter()
        .fold(<T as HasIdentity<Op>>::identity(), |acc, x| {
            op::<Op, _>(&acc, &x)
        })
}

/// Summe. Braucht Kommutativität nicht für die Korrektheit, aber für die
/// Erwartung, dass die Reihenfolge egal ist.
pub fn sum<T: CommutativeMonoid<Additive>>(items: impl IntoIterator<Item = T>) -> T {
    fold::<Additive, _>(items)
}

/// Produkt in der gegebenen Reihenfolge.
pub fn product<T: Monoid<Multiplicative>>(items: impl IntoIterator<Item = T>) -> T {
    fold::<Multiplicative, _>(items)
}

/// `a - b = a + (-b)`
pub fn sub<T: Group<Additive>>(a: &T, b: &T) -> T {
    op::<Additive, _>(a, &<T as HasInverse<Additive>>::inverse(b))
}

/// `a / b = a · b⁻¹`, `None` für `b = 0`.
pub fn try_div<T: DivisionRing>(a: &T, b: &T) -> Option<T> {
    <T as HasPartialInverse<Multiplicative>>::try_inverse(b).map(|i| op::<Multiplicative, _>(a, &i))
}

/// Größter gemeinsamer Teiler per euklidischem Algorithmus. Er endet, weil
/// der Rest nach [`RemainderDecreases`](crate::laws::RemainderDecreases)
/// schrumpft.
pub fn gcd<T: EuclideanRing + PartialEq>(a: &T, b: &T) -> T {
    let zero = <T as HasIdentity<Additive>>::identity();
    let mut a = op::<Additive, _>(a, &zero);
    let mut b = op::<Additive, _>(b, &zero);
    while b != zero {
        let (_, r) = a.div_rem(&b).expect("b ≠ 0");
        a = b;
        b = r;
    }
    a
}

/// `|x|` in einem geordneten Körper.
pub fn abs<T: OrderedField>(x: &T) -> T {
    let zero = <T as HasIdentity<Additive>>::identity();
    if zero.relates(x) {
        op::<Additive, _>(x, &zero)
    } else {
        <T as HasInverse<Additive>>::inverse(x)
    }
}

/// `xʸ = exp(y · ln x)`, `None` wo `ln x` nicht definiert ist.
pub fn powf<T: ElementaryRing>(x: &T, y: &T) -> Option<T> {
    x.ln().map(|l| op::<Multiplicative, _>(y, &l).exp())
}

/// `tanh(x) = (e²ˣ − 1) / (e²ˣ + 1)`
pub fn tanh<T: ElementaryRing + InverseWhereDefined<Multiplicative>>(x: &T) -> Option<T> {
    let one = <T as HasIdentity<Multiplicative>>::identity();
    let e2 = op::<Additive, _>(x, x).exp();
    let num = sub(&e2, &one);
    let den = op::<Additive, _>(&e2, &one);
    den.try_inverse().map(|d| op::<Multiplicative, _>(&num, &d))
}

/// Logistische Funktion `σ(x) = 1 / (1 + e⁻ˣ)`
pub fn sigmoid<T: ElementaryRing + InverseWhereDefined<Multiplicative>>(x: &T) -> Option<T> {
    let one = <T as HasIdentity<Multiplicative>>::identity();
    op::<Additive, _>(&one, &x.inverse().exp()).try_inverse()
}
