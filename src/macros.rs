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
///
///     // Generisch: Gesetze gelten unter Bedingungen an die Parameter.
///     for[T: AbelianGroup<Additive>, const N: usize] [T; N] {
///         Additive: associative, commutative, identity, inverse;
///     }
/// }
/// ```
///
/// Parameter dürfen beliebige Typen sein, z. B. `Vec3: module` mit dem
/// Skalartyp `f64`: `f64: module;`.
///
/// Zusammengesetzte Schlüsselwörter (`identity`, `distributive`,
/// `partial_order`, `module`, …) expandieren in mehrere Gesetze.
#[macro_export]
macro_rules! laws {
    () => {};
    (for [$($g:tt)*] $t:ty { $($body:tt)* } $($rest:tt)*) => {
        $crate::__laws_body!([$($g)*] $t; $($body)*);
        $crate::laws!($($rest)*);
    };
    ($t:ty { $($body:tt)* } $($rest:tt)*) => {
        $crate::__laws_body!([] $t; $($body)*);
        $crate::laws!($($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __laws_body {
    ($g:tt $t:ty;) => {};
    ($g:tt $t:ty; [$a:ty, $b:ty, $c:ty] : $($law:ident),+ ; $($rest:tt)*) => {
        $( $crate::__law!($law; $g $t; $a, $b, $c); )+
        $crate::__laws_body!($g $t; $($rest)*);
    };
    ($g:tt $t:ty; [$a:ty, $b:ty] : $($law:ident),+ ; $($rest:tt)*) => {
        $( $crate::__law!($law; $g $t; $a, $b); )+
        $crate::__laws_body!($g $t; $($rest)*);
    };
    ($g:tt $t:ty; $a:ty : $($law:ident),+ ; $($rest:tt)*) => {
        $( $crate::__law!($law; $g $t; $a); )+
        $crate::__laws_body!($g $t; $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_law {
    ($tr:ident; [$($g:tt)*] $t:ty; $($p:ty),+) => {
        impl<$($g)*> $crate::laws::$tr<$($p),+> for $t {
            fn __sealed(_: $crate::__private::Token) {}
        }
    };
}

/// Zuordnung Schlüsselwort → aktuelle Basis-Gesetze.
#[doc(hidden)]
#[macro_export]
macro_rules! __law {
    // --- Atome: eine Operation bzw. ein Parameter ------------------------------
    (left_identity; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(LeftIdentity; $g $t; $a); };
    (right_identity; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(RightIdentity; $g $t; $a); };
    (left_absorbing; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(LeftAbsorbing; $g $t; $a); };
    (right_absorbing; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(RightAbsorbing; $g $t; $a); };
    (left_inverse; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(LeftInverse; $g $t; $a); };
    (right_inverse; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(RightInverse; $g $t; $a); };
    (left_cancellative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(LeftCancellative; $g $t; $a); };
    (right_cancellative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(RightCancellative; $g $t; $a); };
    (associative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Associative; $g $t; $a); };
    (left_alternative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(LeftAlternative; $g $t; $a); };
    (right_alternative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(RightAlternative; $g $t; $a); };
    (flexible; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Flexible; $g $t; $a); };
    (medial; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Medial; $g $t; $a); };
    (commutative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Commutative; $g $t; $a); };
    (idempotent; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Idempotent; $g $t; $a); };
    (involutive; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Involutive; $g $t; $a); };
    (anti_multiplicative; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(AntiMultiplicative; $g $t; $a); };
    (reflexive; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Reflexive; $g $t; $a); };
    (antisymmetric; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Antisymmetric; $g $t; $a); };
    (transitive; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Transitive; $g $t; $a); };
    (total; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(Total; $g $t; $a); };
    (scalar_identity; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(ScalarIdentity; $g $t; $a); };
    (scalar_compatible; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(ScalarCompatible; $g $t; $a); };
    (scalar_distributes_over_vectors; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(ScalarDistributesOverVectors; $g $t; $a); };
    (scalar_distributes_over_scalars; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(ScalarDistributesOverScalars; $g $t; $a); };
    (exp_inverts_ln; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(ExpInvertsLn; $g $t; $a); };
    (sqrt_squares; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(SqrtSquares; $g $t; $a); };
    (inverse_where_defined; $g:tt $t:ty; $a:ty) => { $crate::__impl_law!(InverseWhereDefined; $g $t; $a); };
    // --- Atome: zwei Parameter -------------------------------------------------
    (left_distributive; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(LeftDistributive; $g $t; $a, $b); };
    (right_distributive; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(RightDistributive; $g $t; $a, $b); };
    (absorption; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(Absorption; $g $t; $a, $b); };
    (zero_divisor_free; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(ZeroDivisorFree; $g $t; $a, $b); };
    (anticommutative; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(Anticommutative; $g $t; $a, $b); };
    (left_inverse_except_zero; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(LeftInverseExceptZero; $g $t; $a, $b); };
    (right_inverse_except_zero; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(RightInverseExceptZero; $g $t; $a, $b); };
    (nontrivial; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(NonTrivial; $g $t; $a, $b); };
    (left_monotone; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(LeftMonotone; $g $t; $a, $b); };
    (right_monotone; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(RightMonotone; $g $t; $a, $b); };
    (division_with_remainder; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(DivisionWithRemainder; $g $t; $a, $b); };
    (remainder_decreases; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(RemainderDecreases; $g $t; $a, $b); };
    (exp_homomorphism; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(ExpHomomorphism; $g $t; $a, $b); };
    (sqrt_of_non_negative; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(SqrtOfNonNegative; $g $t; $a, $b); };
    (pythagorean; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(Pythagorean; $g $t; $a, $b); };
    (sine_addition; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(SineAddition; $g $t; $a, $b); };
    (cosine_addition; $g:tt $t:ty; $a:ty, $b:ty) => { $crate::__impl_law!(CosineAddition; $g $t; $a, $b); };
    // --- Atome: drei Parameter -------------------------------------------------
    (positive_product; $g:tt $t:ty; $a:ty, $b:ty, $c:ty) => { $crate::__impl_law!(PositiveProduct; $g $t; $a, $b, $c); };
    // --- Zusammengesetzte Schlüsselwörter --------------------------------------
    (identity; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(left_identity; $g $t; $a);
        $crate::__law!(right_identity; $g $t; $a);
    };
    (absorbing; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(left_absorbing; $g $t; $a);
        $crate::__law!(right_absorbing; $g $t; $a);
    };
    (inverse; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(left_inverse; $g $t; $a);
        $crate::__law!(right_inverse; $g $t; $a);
    };
    (cancellative; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(left_cancellative; $g $t; $a);
        $crate::__law!(right_cancellative; $g $t; $a);
    };
    (alternative; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(left_alternative; $g $t; $a);
        $crate::__law!(right_alternative; $g $t; $a);
    };
    (preorder; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(reflexive; $g $t; $a);
        $crate::__law!(transitive; $g $t; $a);
    };
    (partial_order; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(preorder; $g $t; $a);
        $crate::__law!(antisymmetric; $g $t; $a);
    };
    (total_order; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(partial_order; $g $t; $a);
        $crate::__law!(total; $g $t; $a);
    };
    (conjugation; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(involutive; $g $t; $a);
        $crate::__law!(anti_multiplicative; $g $t; $a);
    };
    (module; $g:tt $t:ty; $a:ty) => {
        $crate::__law!(scalar_identity; $g $t; $a);
        $crate::__law!(scalar_compatible; $g $t; $a);
        $crate::__law!(scalar_distributes_over_vectors; $g $t; $a);
        $crate::__law!(scalar_distributes_over_scalars; $g $t; $a);
    };
    (distributive; $g:tt $t:ty; $a:ty, $b:ty) => {
        $crate::__law!(left_distributive; $g $t; $a, $b);
        $crate::__law!(right_distributive; $g $t; $a, $b);
    };
    (monotone; $g:tt $t:ty; $a:ty, $b:ty) => {
        $crate::__law!(left_monotone; $g $t; $a, $b);
        $crate::__law!(right_monotone; $g $t; $a, $b);
    };
    (inverse_except_zero; $g:tt $t:ty; $a:ty, $b:ty) => {
        $crate::__law!(left_inverse_except_zero; $g $t; $a, $b);
        $crate::__law!(right_inverse_except_zero; $g $t; $a, $b);
    };
    (euclidean; $g:tt $t:ty; $a:ty, $b:ty) => {
        $crate::__law!(division_with_remainder; $g $t; $a, $b);
        $crate::__law!(remainder_decreases; $g $t; $a, $b);
    };
    (trigonometric; $g:tt $t:ty; $a:ty, $b:ty) => {
        $crate::__law!(pythagorean; $g $t; $a, $b);
        $crate::__law!(sine_addition; $g $t; $a, $b);
        $crate::__law!(cosine_addition; $g $t; $a, $b);
    };
    // --- Unbekannt -------------------------------------------------------------
    ($law:ident; $($rest:tt)*) => {
        compile_error!(concat!("laws!: unbekanntes Gesetz oder falsche Parameterzahl: `", stringify!($law), "`"));
    };
}
