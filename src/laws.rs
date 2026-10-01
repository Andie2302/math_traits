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
    BinaryOp, BinaryRelation, HasAbsorbing, HasIdentity, HasInverse, HasPartialInverse, op,
};

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
