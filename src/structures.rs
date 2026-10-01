//! Strukturen: reine Konjunktionen von Gesetzen.
//!
//! Jeder Trait hier ist **leer** und gilt per Blanket-Impl automatisch für jeden
//! Typ, der seine Bestandteile erfüllt. Das erzwingt das Makro `structure!`,
//! denn einen Body oder eine manuelle Impl kann man damit gar nicht schreiben.
//! So kann oberhalb der Basis kein neues Axiom entstehen.

use crate::laws::*;
use crate::signature::{Additive, Join, LessEq, Meet, Multiplicative};

macro_rules! structure {
    ($(#[$m:meta])* $name:ident<$($p:ident),*> = $($bound:tt)+) => {
        $(#[$m])*
        pub trait $name<$($p),*>: $($bound)+ {}
        impl<T, $($p),*> $name<$($p),*> for T where T: $($bound)+ {}
    };
}

// ===========================================================================
// Beidseitige Zusammenfassungen
// ===========================================================================

structure!(
    /// `e • x = x = x • e`
    Identity<Op> = LeftIdentity<Op> + RightIdentity<Op>
);
structure!(
    /// `z • x = z = x • z`
    Absorbing<Op> = LeftAbsorbing<Op> + RightAbsorbing<Op>
);
structure!(
    /// `x⁻¹ • x = e = x • x⁻¹`
    Inverse<Op> = LeftInverse<Op> + RightInverse<Op>
);
structure!(Cancellative<Op> = LeftCancellative<Op> + RightCancellative<Op>);
structure!(Alternative<Op> = LeftAlternative<Op> + RightAlternative<Op>);
structure!(Distributive<Mul, Add> = LeftDistributive<Mul, Add> + RightDistributive<Mul, Add>);
structure!(
    /// `x ≠ 0 ⇒ x⁻¹ · x = 1 = x · x⁻¹`
    InverseExceptZero<Mul, Add> = LeftInverseExceptZero<Mul, Add> + RightInverseExceptZero<Mul, Add>
);
structure!(Monotone<Op, R> = LeftMonotone<Op, R> + RightMonotone<Op, R>);

// ===========================================================================
// Eine Operation
// ===========================================================================

structure!(Semigroup<Op> = Associative<Op>);
structure!(Monoid<Op> = Semigroup<Op> + Identity<Op>);
structure!(CommutativeMonoid<Op> = Monoid<Op> + Commutative<Op>);
structure!(Group<Op> = Monoid<Op> + Inverse<Op>);
structure!(AbelianGroup<Op> = Group<Op> + Commutative<Op>);
structure!(
    /// Halbgruppe, in der jedes Element idempotent ist.
    Band<Op> = Semigroup<Op> + Idempotent<Op>
);
structure!(Semilattice<Op> = Band<Op> + Commutative<Op>);

// ===========================================================================
// Ordnungen
// ===========================================================================

structure!(Preorder<R> = Reflexive<R> + Transitive<R>);
structure!(PartialOrder<R> = Preorder<R> + Antisymmetric<R>);
structure!(TotalOrder<R> = PartialOrder<R> + Total<R>);

// ===========================================================================
// Zwei Operationen (mit festen Markern)
// ===========================================================================

structure!(
    /// Ring mit Eins.
    Ring<> = AbelianGroup<Additive>
        + Monoid<Multiplicative>
        + Distributive<Multiplicative, Additive>
);
structure!(CommutativeRing<> = Ring + Commutative<Multiplicative>);
structure!(
    /// Ring, in dem `x·x = x` gilt. Er ist automatisch kommutativ.
    BooleanRing<> = Ring + Idempotent<Multiplicative>
);
structure!(
    Lattice<> = Semilattice<Meet>
        + Semilattice<Join>
        + Absorption<Meet, Join>
        + Absorption<Join, Meet>
);
structure!(
    /// Partielle Ordnung, mit der `+` von beiden Seiten verträglich ist.
    OrderedAdditiveMonoid<> = CommutativeMonoid<Additive>
        + PartialOrder<LessEq>
        + Monotone<Additive, LessEq>
);
structure!(
    IntegralDomain<> = CommutativeRing
        + ZeroDivisorFree<Multiplicative, Additive>
        + NonTrivial<Multiplicative, Additive>
);
structure!(
    /// Schiefkörper: Jedes Element `≠ 0` ist invertierbar.
    DivisionRing<> = Ring
        + InverseExceptZero<Multiplicative, Additive>
        + NonTrivial<Multiplicative, Additive>
);
structure!(Field<> = DivisionRing + Commutative<Multiplicative>);
