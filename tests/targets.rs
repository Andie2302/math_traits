//! Zielkatalog: Wann ist die Basis abgeschlossen?
//!
//! Jede Zeile hier ist ein Ziel aus `BASIS.md`. Ein Ziel gilt als erreicht,
//! wenn es *nur* mit Bounds aus `structures` kompiliert. Offene Ziele stehen
//! als `// OFFEN:` dabei. Sind keine mehr offen, ist die Basis für diese
//! Version abgeschlossen.

#![allow(dead_code)]

use math_traits::derived::*;
use math_traits::impls::cayley_dickson::*;
use math_traits::signature::*;
use math_traits::solve::*;
use math_traits::structures::*;

// --- Strukturen -----------------------------------------------------------

fn monoid<T: Monoid<Additive>>() {}
fn group<T: Group<Additive>>() {}
fn abelian_group<T: AbelianGroup<Additive>>() {}
fn ring<T: Ring>() {}
fn commutative_ring<T: CommutativeRing>() {}
fn integral_domain<T: IntegralDomain>() {}
fn euclidean_ring<T: EuclideanRing>() {}
fn division_ring<T: DivisionRing>() {}
fn field<T: Field>() {}
fn ordered_field<T: OrderedField>() {}
fn alternative_ring<T: AlternativeRing>() {}
fn non_associative_ring<T: NonAssociativeRing>() {}
fn module<V: Module<S>, S: Ring>() {}
fn vector_space<V: Module<S>, S: Field>() {}
fn lattice<T: Lattice>() {}
fn total_order<T: TotalOrder<LessEq>>() {}
// OFFEN: NormedSpace – braucht Signatur `‖·‖: V → S` (siehe BASIS.md)

// --- Cayley-Dickson bis zu den Sedenionen --------------------------------

#[test]
fn cayley_dickson_tower() {
    ordered_field::<f64>();
    field::<Complex<f64>>();
    division_ring::<Quaternion<f64>>();
    alternative_ring::<Octonion<f64>>();
    non_associative_ring::<Sedenion<f64>>();
    euclidean_ring::<i64>();
    vector_space::<[f64; 3], f64>();
    module::<[i64; 4], i64>();
}

// --- Algorithmen ----------------------------------------------------------

fn power<T: Monoid<Multiplicative>>(x: &T) -> T {
    pow::<Multiplicative, _>(x, 10)
}
fn inverse_power<T: Group<Additive>>(x: &T) -> T {
    pow_signed::<Additive, _>(x, -3)
}
fn total<T: CommutativeMonoid<Additive>>(v: Vec<T>) -> T {
    sum(v)
}
fn quotient<T: Field>(a: &T, b: &T) -> Option<T> {
    try_div(a, b)
}
fn greatest_common_divisor<T: EuclideanRing + PartialEq>(a: &T, b: &T) -> T {
    gcd(a, b)
}
fn gauss_exact<T: Field + PartialEq + Clone>(a: Vec<Vec<T>>, b: Vec<T>) -> Option<Vec<T>> {
    solve_linear(a, b)
}
fn gauss_pivot<T: OrderedField + PartialEq + Clone>(a: Vec<Vec<T>>, b: Vec<T>) -> Option<Vec<T>> {
    solve_linear_pivoting(a, b)
}
fn bisection<T: OrderedField + PartialEq + Clone>(
    f: impl Fn(&T) -> T,
    a: T,
    b: T,
    tol: &T,
) -> Option<T> {
    bisect(f, a, b, tol, 200)
}
fn newton_nd<T: OrderedField + PartialEq + Clone>(
    f: impl Fn(&[T]) -> Vec<T>,
    j: impl Fn(&[T]) -> Vec<Vec<T>>,
    x0: Vec<T>,
    tol: &T,
) -> Option<Vec<T>> {
    newton(f, j, x0, tol, 100)
}
// OFFEN: Polynomauswertung (Horner) – geht schon mit `Ring`, fehlt nur als Funktion
// OFFEN: Konvergenz in einer Norm statt komponentenweise – braucht NormedSpace

#[test]
fn catalog_compiles() {}
