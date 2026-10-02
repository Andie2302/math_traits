//! Gesetze: die aktuelle Basis.
//!
//! Jedes Gesetz ist **genau eine Aussage** und verweist nur auf die
//! [Signatur](crate::signature), nie auf andere Gesetze. Damit sind die
//! Gesetze untereinander unabhängig.
//!
//! Jedes Gesetz bringt eine Prüffunktion `holds` mit, mit der man das
//! Versprechen an konkreten Werten testen kann.
//!
//! Gesetze werden **nicht** von Hand implementiert, sondern über
//! [`laws!`](crate::laws!). Wird ein Gesetz später feiner zerlegt, wandert es
//! nach [`crate::structures`] (als Konjunktion der neuen, feineren Gesetze).
//! Das Schlüsselwort im Makro bleibt gleich.

use crate::__private::Token;
use crate::signature::{
    Additive, BinaryOp, BinaryRelation, HasAbsorbing, HasConjugate, HasDivRem, HasEuclideanSize,
    HasExp, HasIdentity, HasInverse, HasLn, HasPartialInverse, HasSinCos, HasSqrt, InnerProduct,
    Multiplicative, ScalarMul, op,
};
use crate::signature::{Contract, ExpMap, HasAtan2, HasRootsOfUnity, Outer, TryBinaryOp};

// ===========================================================================
// 1. Ausgezeichnete Elemente
// ===========================================================================

/// `e • x = x`
pub trait LeftIdentity<Op>: HasIdentity<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&Self::identity(), x) == *x
    }
}

/// `x • e = x`
pub trait RightIdentity<Op>: HasIdentity<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, &Self::identity()) == *x
    }
}

/// `z • x = z`
pub trait LeftAbsorbing<Op>: HasAbsorbing<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&Self::absorbing(), x) == Self::absorbing()
    }
}

/// `x • z = z`
pub trait RightAbsorbing<Op>: HasAbsorbing<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, &Self::absorbing()) == Self::absorbing()
    }
}

// ===========================================================================
// 2. Inverse und Kürzbarkeit
// ===========================================================================

/// `x⁻¹ • x = e`
pub trait LeftInverse<Op>: HasInverse<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&x.inverse(), x) == Self::identity()
    }
}

/// `x • x⁻¹ = e`
pub trait RightInverse<Op>: HasInverse<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, &x.inverse()) == Self::identity()
    }
}

/// `c • x = c • y  ⇒  x = y`
pub trait LeftCancellative<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(c: &Self, x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(c, x) != op::<Op, _>(c, y) || x == y
    }
}

/// `x • c = y • c  ⇒  x = y`
pub trait RightCancellative<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(c: &Self, x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, c) != op::<Op, _>(y, c) || x == y
    }
}

// ===========================================================================
// 3. Klammerung
// ===========================================================================

/// `(x • y) • z = x • (y • z)`
pub trait Associative<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&op::<Op, _>(x, y), z) == op::<Op, _>(x, &op::<Op, _>(y, z))
    }
}

/// `(x • x) • y = x • (x • y)`
pub trait LeftAlternative<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&op::<Op, _>(x, x), y) == op::<Op, _>(x, &op::<Op, _>(x, y))
    }
}

/// `(y • x) • x = y • (x • x)`
pub trait RightAlternative<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&op::<Op, _>(y, x), x) == op::<Op, _>(y, &op::<Op, _>(x, x))
    }
}

/// `(x • y) • x = x • (y • x)`
pub trait Flexible<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&op::<Op, _>(x, y), x) == op::<Op, _>(x, &op::<Op, _>(y, x))
    }
}

/// `(x • y) • (u • v) = (x • u) • (y • v)`
pub trait Medial<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, u: &Self, v: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(&op::<Op, _>(x, y), &op::<Op, _>(u, v))
            == op::<Op, _>(&op::<Op, _>(x, u), &op::<Op, _>(y, v))
    }
}

// ===========================================================================
// 4. Vertauschen und Selbstanwendung
// ===========================================================================

/// `x • y = y • x`
pub trait Commutative<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, y) == op::<Op, _>(y, x)
    }
}

/// Jedes Element ist idempotent: `x • x = x`
pub trait Idempotent<Op>: BinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, x) == *x
    }
}

// ===========================================================================
// 5. Zwei Operationen
// ===========================================================================

/// `x · (y + z) = (x · y) + (x · z)`
pub trait LeftDistributive<Mul, Add>: BinaryOp<Mul> + BinaryOp<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Mul, _>(x, &op::<Add, _>(y, z))
            == op::<Add, _>(&op::<Mul, _>(x, y), &op::<Mul, _>(x, z))
    }
}

/// `(y + z) · x = (y · x) + (z · x)`
pub trait RightDistributive<Mul, Add>: BinaryOp<Mul> + BinaryOp<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Mul, _>(&op::<Add, _>(y, z), x)
            == op::<Add, _>(&op::<Mul, _>(y, x), &op::<Mul, _>(z, x))
    }
}

/// `x • (x ∘ y) = x` (Absorptionsgesetz für Verbände)
pub trait Absorption<Outer, Inner>: BinaryOp<Outer> + BinaryOp<Inner> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Outer, _>(x, &op::<Inner, _>(x, y)) == *x
    }
}

/// `x · y = 0  ⇒  x = 0 ∨ y = 0`, wobei `0` das Element von `Add` ist.
pub trait ZeroDivisorFree<Mul, Add>: BinaryOp<Mul> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        let zero = <Self as HasIdentity<Add>>::identity();
        op::<Mul, _>(x, y) != zero || *x == zero || *y == zero
    }
}

/// `x · y = -(y · x)`, wobei `-` das Inverse von `Add` ist.
pub trait Anticommutative<Mul, Add>: BinaryOp<Mul> + HasInverse<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Mul, _>(x, y) == <Self as HasInverse<Add>>::inverse(&op::<Mul, _>(y, x))
    }
}

/// `try_inverse(x) = Some(y)  ⇒  y · x = 1 = x · y`: Wo ein Inverses
/// geliefert wird, ist es korrekt. Sagt nichts darüber, *wo* es existiert.
pub trait InverseWhereDefined<Mul>: HasPartialInverse<Mul> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        let one = <Self as HasIdentity<Mul>>::identity();
        x.try_inverse()
            .is_none_or(|y| op::<Mul, _>(&y, x) == one && op::<Mul, _>(x, &y) == one)
    }
}

/// `x ≠ 0  ⇒  x⁻¹ existiert und x⁻¹ · x = 1`, wobei `0` das Element von `Add` ist.
pub trait LeftInverseExceptZero<Mul, Add>: HasPartialInverse<Mul> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        *x == <Self as HasIdentity<Add>>::identity()
            || x.try_inverse()
                .is_some_and(|i| op::<Mul, _>(&i, x) == <Self as HasIdentity<Mul>>::identity())
    }
}

/// `x ≠ 0  ⇒  x⁻¹ existiert und x · x⁻¹ = 1`, wobei `0` das Element von `Add` ist.
pub trait RightInverseExceptZero<Mul, Add>: HasPartialInverse<Mul> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        *x == <Self as HasIdentity<Add>>::identity()
            || x.try_inverse()
                .is_some_and(|i| op::<Mul, _>(x, &i) == <Self as HasIdentity<Mul>>::identity())
    }
}

/// `1 ≠ 0`: Die Elemente von `Mul` und `Add` sind verschieden.
pub trait NonTrivial<Mul, Add>: HasIdentity<Mul> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds() -> bool
    where
        Self: PartialEq,
    {
        <Self as HasIdentity<Mul>>::identity() != <Self as HasIdentity<Add>>::identity()
    }
}

// ===========================================================================
// 6. Relationen
// ===========================================================================

/// `x R x`
pub trait Reflexive<R>: BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool {
        x.relates(x)
    }
}

/// `x R y ∧ y R x  ⇒  x = y`
pub trait Antisymmetric<R>: BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        !(x.relates(y) && y.relates(x)) || x == y
    }
}

/// `x R y ∧ y R z  ⇒  x R z`
pub trait Transitive<R>: BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool {
        !(x.relates(y) && y.relates(z)) || x.relates(z)
    }
}

/// `x R y ∨ y R x`
pub trait Total<R>: BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool {
        x.relates(y) || y.relates(x)
    }
}

/// `x R y  ⇒  (z • x) R (z • y)`
pub trait LeftMonotone<Op, R>: BinaryOp<Op> + BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool {
        !x.relates(y) || op::<Op, _>(z, x).relates(&op::<Op, _>(z, y))
    }
}

/// `x R y  ⇒  (x • z) R (y • z)`
pub trait RightMonotone<Op, R>: BinaryOp<Op> + BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool {
        !x.relates(y) || op::<Op, _>(x, z).relates(&op::<Op, _>(y, z))
    }
}

/// `0 R x ∧ 0 R y  ⇒  0 R (x · y)` (Produkt nichtnegativer Elemente ist nichtnegativ)
pub trait PositiveProduct<Mul, Add, R>:
    BinaryOp<Mul> + HasIdentity<Add> + BinaryRelation<R>
{
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool {
        let zero = <Self as HasIdentity<Add>>::identity();
        !(zero.relates(x) && zero.relates(y)) || zero.relates(&op::<Mul, _>(x, y))
    }
}

// ===========================================================================
// 7. Konjugation
// ===========================================================================

/// `(x*)* = x`
pub trait Involutive<Mul>: HasConjugate {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        x.conj().conj() == *x
    }
}

/// `(x · y)* = y* · x*`
pub trait AntiMultiplicative<Mul>: BinaryOp<Mul> + HasConjugate {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Mul, _>(x, y).conj() == op::<Mul, _>(&y.conj(), &x.conj())
    }
}

// ===========================================================================
// 8. Division mit Rest
// ===========================================================================

/// `b ≠ 0  ⇒  a = q · b + r` mit `(q, r) = a.div_rem(b)`
pub trait DivisionWithRemainder<Mul, Add>: HasDivRem + BinaryOp<Mul> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, b: &Self) -> bool
    where
        Self: PartialEq,
    {
        *b == <Self as HasIdentity<Add>>::identity()
            || a.div_rem(b)
                .is_some_and(|(q, r)| op::<Add, _>(&op::<Mul, _>(&q, b), &r) == *a)
    }
}

/// `b ≠ 0  ⇒  r = 0 ∨ |r| < |b|`: Der Rest schrumpft, also endet der
/// euklidische Algorithmus.
pub trait RemainderDecreases<Mul, Add>: HasDivRem + HasEuclideanSize + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, b: &Self) -> bool
    where
        Self: PartialEq,
    {
        let zero = <Self as HasIdentity<Add>>::identity();
        *b == zero
            || a.div_rem(b)
                .is_some_and(|(_, r)| r == zero || r.euclidean_size() < b.euclidean_size())
    }
}

// ===========================================================================
// 9. Externe Operation (Module, Vektorräume)
// ===========================================================================

/// `1 · v = v`
pub trait ScalarIdentity<S>: ScalarMul<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(v: &Self) -> bool
    where
        Self: PartialEq,
        S: HasIdentity<Multiplicative>,
    {
        v.scale(&S::identity()) == *v
    }
}

/// `(s · t) · v = s · (t · v)`
pub trait ScalarCompatible<S>: ScalarMul<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(s: &S, t: &S, v: &Self) -> bool
    where
        Self: PartialEq,
        S: BinaryOp<Multiplicative>,
    {
        v.scale(&op::<Multiplicative, _>(s, t)) == v.scale(t).scale(s)
    }
}

/// `s · (v + w) = s · v + s · w`
pub trait ScalarDistributesOverVectors<S>: ScalarMul<S> + BinaryOp<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(s: &S, v: &Self, w: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Additive, _>(v, w).scale(s) == op::<Additive, _>(&v.scale(s), &w.scale(s))
    }
}

/// `(s + t) · v = s · v + t · v`
pub trait ScalarDistributesOverScalars<S>: ScalarMul<S> + BinaryOp<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(s: &S, t: &S, v: &Self) -> bool
    where
        Self: PartialEq,
        S: BinaryOp<Additive>,
    {
        v.scale(&op::<Additive, _>(s, t)) == op::<Additive, _>(&v.scale(s), &v.scale(t))
    }
}

// ===========================================================================
// 10. Elementarfunktionen
// ===========================================================================

/// `exp(x + y) = exp(x) · exp(y)`
pub trait ExpHomomorphism<Add, Mul>: HasExp + BinaryOp<Add> + BinaryOp<Mul> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Add, _>(x, y).exp() == op::<Mul, _>(&x.exp(), &y.exp())
    }
}

/// `ln(x) = y  ⇒  exp(y) = x`
///
/// Der Parameter `Mul` dient nur der Einordnung (Schlüsselwort unter
/// `Multiplicative:`).
pub trait ExpInvertsLn<Mul>: HasExp + HasLn {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        x.ln().is_none_or(|y| y.exp() == *x)
    }
}

/// `sqrt(x) = y  ⇒  y · y = x`
pub trait SqrtSquares<Mul>: HasSqrt + BinaryOp<Mul> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        x.sqrt().is_none_or(|y| op::<Mul, _>(&y, &y) == *x)
    }
}

/// `0 R x  ⇒  sqrt(x)` existiert und `0 R sqrt(x)`
pub trait SqrtOfNonNegative<Add, R>: HasSqrt + HasIdentity<Add> + BinaryRelation<R> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool {
        let zero = <Self as HasIdentity<Add>>::identity();
        !zero.relates(x) || x.sqrt().is_some_and(|y| zero.relates(&y))
    }
}

/// `sin²(x) + cos²(x) = 1`
pub trait Pythagorean<Mul, Add>: HasSinCos + HasIdentity<Mul> + BinaryOp<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        let (s, c) = (x.sin(), x.cos());
        op::<Add, _>(&op::<Mul, _>(&s, &s), &op::<Mul, _>(&c, &c))
            == <Self as HasIdentity<Mul>>::identity()
    }
}

/// `sin(x + y) = sin(x)·cos(y) + cos(x)·sin(y)`
pub trait SineAddition<Mul, Add>: HasSinCos + BinaryOp<Mul> + BinaryOp<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Add, _>(x, y).sin()
            == op::<Add, _>(
                &op::<Mul, _>(&x.sin(), &y.cos()),
                &op::<Mul, _>(&x.cos(), &y.sin()),
            )
    }
}

/// `cos(x + y) = cos(x)·cos(y) − sin(x)·sin(y)`
pub trait CosineAddition<Mul, Add>: HasSinCos + BinaryOp<Mul> + HasInverse<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Add, _>(x, y).cos()
            == op::<Add, _>(
                &op::<Mul, _>(&x.cos(), &y.cos()),
                &op::<Mul, _>(&x.sin(), &y.sin()).inverse(),
            )
    }
}

// ===========================================================================
// 11. Konjugation und Addition, reelle Elemente
// ===========================================================================

/// `(x + y)* = x* + y*`
pub trait ConjugateAdditive<Add>: HasConjugate + BinaryOp<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Add, _>(x, y).conj() == op::<Add, _>(&x.conj(), &y.conj())
    }
}

/// `x* = x`: Jedes Element ist reell (Konjugation ist die Identität).
pub trait SelfConjugate<Mul>: HasConjugate {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        x.conj() == *x
    }
}

// ===========================================================================
// 12. Skalarprodukt
// ===========================================================================

/// `⟨u, v + w⟩ = ⟨u, v⟩ + ⟨u, w⟩`
pub trait InnerAdditive<S>: InnerProduct<S> + BinaryOp<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(u: &Self, v: &Self, w: &Self) -> bool
    where
        S: BinaryOp<Additive> + PartialEq,
    {
        u.inner(&op::<Additive, _>(v, w)) == op::<Additive, _>(&u.inner(v), &u.inner(w))
    }
}

/// `⟨u, s · v⟩ = s · ⟨u, v⟩`
pub trait InnerHomogeneous<S>: InnerProduct<S> + ScalarMul<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(s: &S, u: &Self, v: &Self) -> bool
    where
        S: BinaryOp<Multiplicative> + PartialEq,
    {
        u.inner(&v.scale(s)) == op::<Multiplicative, _>(s, &u.inner(v))
    }
}

/// `⟨v, u⟩ = ⟨u, v⟩*` (für reelle Skalare: Symmetrie)
pub trait InnerConjugateSymmetric<S>: InnerProduct<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(u: &Self, v: &Self) -> bool
    where
        S: HasConjugate + PartialEq,
    {
        v.inner(u) == u.inner(v).conj()
    }
}

/// `0 R ⟨v, v⟩`
pub trait InnerNonNegative<S, R>: InnerProduct<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(v: &Self) -> bool
    where
        S: HasIdentity<Additive> + BinaryRelation<R>,
    {
        S::identity().relates(&v.inner(v))
    }
}

/// `⟨v, v⟩ = 0  ⇒  v = 0`
pub trait InnerDefinite<S>: InnerProduct<S> + HasIdentity<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(v: &Self) -> bool
    where
        Self: PartialEq,
        S: HasIdentity<Additive> + PartialEq,
    {
        v.inner(v) != S::identity() || *v == Self::identity()
    }
}

// ===========================================================================
// 13. Einheitswurzeln
// ===========================================================================

/// `primitive_root_of_unity(n) = Some(ω)  ⇒  ωⁿ = 1 ∧ ωᵏ ≠ 1 für 0 < k < n`
pub trait PrimitiveRootOfUnity<Mul>: HasRootsOfUnity + HasIdentity<Mul> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(n: usize) -> bool
    where
        Self: PartialEq,
    {
        let Some(w) = Self::primitive_root_of_unity(n) else {
            return true;
        };
        let one = <Self as HasIdentity<Mul>>::identity();
        let mut p = op::<Mul, _>(&w, &one);
        for _ in 1..n {
            if p == one {
                return false;
            }
            p = op::<Mul, _>(&p, &w);
        }
        n > 0 && p == one
    }
}

// ===========================================================================
// 14. Kontraktion und Tensorprodukt (Typen A × B → C)
// ===========================================================================

/// `(a · b) · c = a · (b · c)` über drei Typen.
pub trait ContractAssociative<B, C>: Contract<B> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds<Out>(a: &Self, b: &B, c: &C) -> bool
    where
        <Self as Contract<B>>::Output: Contract<C, Output = Out>,
        B: Contract<C>,
        Self: Contract<<B as Contract<C>>::Output, Output = Out>,
        Out: PartialEq,
    {
        a.contract(b).contract(c) == a.contract(&b.contract(c))
    }
}

/// `a · (b + b') = a · b + a · b'`
pub trait ContractLeftDistributive<B>: Contract<B> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, b: &B, b2: &B) -> bool
    where
        B: BinaryOp<Additive>,
        Self::Output: BinaryOp<Additive> + PartialEq,
    {
        a.contract(&op::<Additive, _>(b, b2)) == op::<Additive, _>(&a.contract(b), &a.contract(b2))
    }
}

/// `(a + a') · b = a · b + a' · b`
pub trait ContractRightDistributive<B>: Contract<B> + BinaryOp<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, a2: &Self, b: &B) -> bool
    where
        Self::Output: BinaryOp<Additive> + PartialEq,
    {
        op::<Additive, _>(a, a2).contract(b) == op::<Additive, _>(&a.contract(b), &a2.contract(b))
    }
}

/// `(s·a) · b = s·(a · b) = a · (s·b)`
pub trait ContractHomogeneous<B, S>: Contract<B> + ScalarMul<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(s: &S, a: &Self, b: &B) -> bool
    where
        B: ScalarMul<S>,
        Self::Output: ScalarMul<S> + PartialEq,
    {
        let ab = a.contract(b).scale(s);
        a.scale(s).contract(b) == ab && a.contract(&b.scale(s)) == ab
    }
}

/// `(a ⊗ b) ⊗ c = a ⊗ (b ⊗ c)` über drei Typen.
pub trait OuterAssociative<B, C>: Outer<B> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds<Out>(a: &Self, b: &B, c: &C) -> bool
    where
        <Self as Outer<B>>::Output: Outer<C, Output = Out>,
        B: Outer<C>,
        Self: Outer<<B as Outer<C>>::Output, Output = Out>,
        Out: PartialEq,
    {
        a.outer(b).outer(c) == a.outer(&b.outer(c))
    }
}

/// `a ⊗ (b + b') = a ⊗ b + a ⊗ b'`
pub trait OuterLeftDistributive<B>: Outer<B> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, b: &B, b2: &B) -> bool
    where
        B: BinaryOp<Additive>,
        Self::Output: BinaryOp<Additive> + PartialEq,
    {
        a.outer(&op::<Additive, _>(b, b2)) == op::<Additive, _>(&a.outer(b), &a.outer(b2))
    }
}

/// `(a + a') ⊗ b = a ⊗ b + a' ⊗ b`
pub trait OuterRightDistributive<B>: Outer<B> + BinaryOp<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, a2: &Self, b: &B) -> bool
    where
        Self::Output: BinaryOp<Additive> + PartialEq,
    {
        op::<Additive, _>(a, a2).outer(b) == op::<Additive, _>(&a.outer(b), &a2.outer(b))
    }
}

// ===========================================================================
// 15. Algebren und Lie-Algebren
// ===========================================================================

/// `(s·x) • y = s·(x • y) = x • (s·y)`: Skalare vertauschen mit dem Produkt.
pub trait OpHomogeneous<Op, S>: BinaryOp<Op> + ScalarMul<S> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(s: &S, x: &Self, y: &Self) -> bool
    where
        Self: PartialEq,
    {
        let xy = op::<Op, _>(x, y).scale(s);
        op::<Op, _>(&x.scale(s), y) == xy && op::<Op, _>(x, &y.scale(s)) == xy
    }
}

/// `x • x = 0` (stärker als Antikommutativität, wenn `2` nicht invertierbar ist)
pub trait Alternating<Op, Add>: BinaryOp<Op> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        Self: PartialEq,
    {
        op::<Op, _>(x, x) == <Self as HasIdentity<Add>>::identity()
    }
}

/// Jacobi-Identität `x•(y•z) + y•(z•x) + z•(x•y) = 0`
pub trait Jacobi<Op, Add>: BinaryOp<Op> + HasIdentity<Add> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self, y: &Self, z: &Self) -> bool
    where
        Self: PartialEq,
    {
        let a = op::<Op, _>(x, &op::<Op, _>(y, z));
        let b = op::<Op, _>(y, &op::<Op, _>(z, x));
        let c = op::<Op, _>(z, &op::<Op, _>(x, y));
        op::<Add, _>(&op::<Add, _>(&a, &b), &c) == <Self as HasIdentity<Add>>::identity()
    }
}

/// `exp(0) = 1`: Die Exponentialabbildung trifft das Einselement der Gruppe.
pub trait ExpMapZero<G>: ExpMap<G> + HasIdentity<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds() -> bool
    where
        G: HasIdentity<Multiplicative> + PartialEq,
    {
        Self::identity().exp_map() == G::identity()
    }
}

/// `exp(−x) · exp(x) = 1`
pub trait ExpMapNegation<G>: ExpMap<G> + HasInverse<Additive> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(x: &Self) -> bool
    where
        G: BinaryOp<Multiplicative> + HasIdentity<Multiplicative> + PartialEq,
    {
        op::<Multiplicative, _>(&x.inverse().exp_map(), &x.exp_map()) == G::identity()
    }
}

// ===========================================================================
// 16. Partielle Operationen
// ===========================================================================

/// Wenn beide Seiten definiert sind: `(a•b)•c = a•(b•c)`
pub trait PartialAssociative<Op>: TryBinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, b: &Self, c: &Self) -> bool
    where
        Self: PartialEq,
    {
        let l = a.try_op(b).and_then(|ab| ab.try_op(c));
        let r = b.try_op(c).and_then(|bc| a.try_op(&bc));
        l.is_none() || r.is_none() || l == r
    }
}

/// `a•b` ist genau dann definiert, wenn `b•a` es ist, und dann gleich.
pub trait PartialCommutative<Op>: TryBinaryOp<Op> {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(a: &Self, b: &Self) -> bool
    where
        Self: PartialEq,
    {
        a.try_op(b) == b.try_op(a)
    }
}

/// `atan2(r·sin t, r·cos t) = t` für `r > 0` und `t ∈ (−π, π]`: `atan2` kehrt
/// `sin`/`cos` um. Geprüft wird mit `r = 1`.
pub trait Atan2InvertsSinCos<Mul>: HasAtan2 + HasSinCos {
    #[doc(hidden)]
    fn __sealed(_: Token);
    fn holds(t: &Self) -> bool
    where
        Self: PartialEq,
    {
        t.sin().atan2(&t.cos()) == *t
    }
}
