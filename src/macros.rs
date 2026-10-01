//! Das `laws!`-Makro: die einzige Stelle, an der Schlüsselwörter auf die
//! aktuelle Basis abgebildet werden.
//!
//! **Ein Gesetz feiner zerlegen** (z. B. `associative` → `a1` + `a2`):
//! 1. In `laws.rs` die neuen Gesetze `A1`, `A2` anlegen.
//! 2. `Associative` aus `laws.rs` entfernen und in `structures.rs` als
//!    `Associative<Op> = A1<Op> + A2<Op>` neu anlegen.
//! 3. Unten die Zeile `(associative; …)` auf die beiden neuen Schlüsselwörter
//!    umleiten.
//!
//! Nutzercode, der `associative` schreibt oder `T: Associative<Op>` verlangt,
//! kompiliert danach unverändert.

/// Deklariert Gesetze für einen Typ.
///
/// ```ignore
/// laws! {
///     MyType {
///         Additive: associative, commutative, identity, inverse;
///         Multiplicative: associative, identity;
///         [Multiplicative, Additive]: distributive;
///         LessEq: total_order;
///         [Additive, LessEq]: monotone;
///     }
/// }
/// ```
///
/// Zusammengesetzte Schlüsselwörter (`identity`, `distributive`,
/// `partial_order`, …) expandieren in mehrere Gesetze.
#[macro_export]
macro_rules! laws {
    ($($t:ty { $($body:tt)* })*) => {
        $( $crate::__laws_body!($t; $($body)*); )*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __laws_body {
    ($t:ty;) => {};
    ($t:ty; [$a:ident, $b:ident] : $($law:ident),+ ; $($rest:tt)*) => {
        $( $crate::__law!($law; $t; $a, $b); )+
        $crate::__laws_body!($t; $($rest)*);
    };
    ($t:ty; $op:ident : $($law:ident),+ ; $($rest:tt)*) => {
        $( $crate::__law!($law; $t; $op); )+
        $crate::__laws_body!($t; $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_law {
    ($tr:ident; $t:ty; $($p:ident),+) => {
        impl $crate::laws::$tr<$($p),+> for $t {
            fn __sealed(_: $crate::__private::Token) {}
        }
    };
}

/// Zuordnung Schlüsselwort → aktuelle Basis-Gesetze.
#[doc(hidden)]
#[macro_export]
macro_rules! __law {
    // --- Atome: eine Operation -------------------------------------------
    (left_identity; $t:ty; $o:ident) => { $crate::__impl_law!(LeftIdentity; $t; $o); };
    (right_identity; $t:ty; $o:ident) => { $crate::__impl_law!(RightIdentity; $t; $o); };
    (left_absorbing; $t:ty; $o:ident) => { $crate::__impl_law!(LeftAbsorbing; $t; $o); };
    (right_absorbing; $t:ty; $o:ident) => { $crate::__impl_law!(RightAbsorbing; $t; $o); };
    (left_inverse; $t:ty; $o:ident) => { $crate::__impl_law!(LeftInverse; $t; $o); };
    (right_inverse; $t:ty; $o:ident) => { $crate::__impl_law!(RightInverse; $t; $o); };
    (left_cancellative; $t:ty; $o:ident) => { $crate::__impl_law!(LeftCancellative; $t; $o); };
    (right_cancellative; $t:ty; $o:ident) => { $crate::__impl_law!(RightCancellative; $t; $o); };
    (associative; $t:ty; $o:ident) => { $crate::__impl_law!(Associative; $t; $o); };
    (left_alternative; $t:ty; $o:ident) => { $crate::__impl_law!(LeftAlternative; $t; $o); };
    (right_alternative; $t:ty; $o:ident) => { $crate::__impl_law!(RightAlternative; $t; $o); };
    (flexible; $t:ty; $o:ident) => { $crate::__impl_law!(Flexible; $t; $o); };
    (medial; $t:ty; $o:ident) => { $crate::__impl_law!(Medial; $t; $o); };
    (commutative; $t:ty; $o:ident) => { $crate::__impl_law!(Commutative; $t; $o); };
    (idempotent; $t:ty; $o:ident) => { $crate::__impl_law!(Idempotent; $t; $o); };

    // --- Atome: Relationen ------------------------------------------------
    (reflexive; $t:ty; $r:ident) => { $crate::__impl_law!(Reflexive; $t; $r); };
    (antisymmetric; $t:ty; $r:ident) => { $crate::__impl_law!(Antisymmetric; $t; $r); };
    (transitive; $t:ty; $r:ident) => { $crate::__impl_law!(Transitive; $t; $r); };
    (total; $t:ty; $r:ident) => { $crate::__impl_law!(Total; $t; $r); };

    // --- Atome: zwei Parameter --------------------------------------------
    (left_distributive; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(LeftDistributive; $t; $m, $a); };
    (right_distributive; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(RightDistributive; $t; $m, $a); };
    (absorption; $t:ty; $o:ident, $i:ident) => { $crate::__impl_law!(Absorption; $t; $o, $i); };
    (zero_divisor_free; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(ZeroDivisorFree; $t; $m, $a); };
    (anticommutative; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(Anticommutative; $t; $m, $a); };
    (left_inverse_except_zero; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(LeftInverseExceptZero; $t; $m, $a); };
    (right_inverse_except_zero; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(RightInverseExceptZero; $t; $m, $a); };
    (nontrivial; $t:ty; $m:ident, $a:ident) => { $crate::__impl_law!(NonTrivial; $t; $m, $a); };
    (left_monotone; $t:ty; $o:ident, $r:ident) => { $crate::__impl_law!(LeftMonotone; $t; $o, $r); };
    (right_monotone; $t:ty; $o:ident, $r:ident) => { $crate::__impl_law!(RightMonotone; $t; $o, $r); };

    // --- Zusammengesetzte Schlüsselwörter ---------------------------------
    (identity; $t:ty; $o:ident) => {
        $crate::__law!(left_identity; $t; $o);
        $crate::__law!(right_identity; $t; $o);
    };
    (absorbing; $t:ty; $o:ident) => {
        $crate::__law!(left_absorbing; $t; $o);
        $crate::__law!(right_absorbing; $t; $o);
    };
    (inverse; $t:ty; $o:ident) => {
        $crate::__law!(left_inverse; $t; $o);
        $crate::__law!(right_inverse; $t; $o);
    };
    (cancellative; $t:ty; $o:ident) => {
        $crate::__law!(left_cancellative; $t; $o);
        $crate::__law!(right_cancellative; $t; $o);
    };
    (alternative; $t:ty; $o:ident) => {
        $crate::__law!(left_alternative; $t; $o);
        $crate::__law!(right_alternative; $t; $o);
    };
    (preorder; $t:ty; $r:ident) => {
        $crate::__law!(reflexive; $t; $r);
        $crate::__law!(transitive; $t; $r);
    };
    (partial_order; $t:ty; $r:ident) => {
        $crate::__law!(preorder; $t; $r);
        $crate::__law!(antisymmetric; $t; $r);
    };
    (total_order; $t:ty; $r:ident) => {
        $crate::__law!(partial_order; $t; $r);
        $crate::__law!(total; $t; $r);
    };
    (distributive; $t:ty; $m:ident, $a:ident) => {
        $crate::__law!(left_distributive; $t; $m, $a);
        $crate::__law!(right_distributive; $t; $m, $a);
    };
    (inverse_except_zero; $t:ty; $m:ident, $a:ident) => {
        $crate::__law!(left_inverse_except_zero; $t; $m, $a);
        $crate::__law!(right_inverse_except_zero; $t; $m, $a);
    };
    (monotone; $t:ty; $o:ident, $r:ident) => {
        $crate::__law!(left_monotone; $t; $o, $r);
        $crate::__law!(right_monotone; $t; $o, $r);
    };

    // --- Unbekannt ---------------------------------------------------------
    ($law:ident; $($rest:tt)*) => {
        compile_error!(concat!("laws!: unbekanntes Gesetz oder falsche Parameterzahl: `", stringify!($law), "`"));
    };
}
