//! Zielkatalog: Wann ist die Basis abgeschlossen?
//!
//! Jede Zeile hier ist ein Ziel aus `BASIS.md`. Ein Ziel gilt als erreicht,
//! wenn es *nur* mit Bounds aus `structures` kompiliert. Offene Ziele stehen
//! als `// OFFEN:` dabei. Sind keine mehr offen, ist die Basis für diese
//! Version abgeschlossen.

#![allow(dead_code)]

use math_traits::autodiff::*;
use math_traits::derived::*;
use math_traits::geometry::*;
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
fn elementary_ring<T: ElementaryRing>() {}
fn real_field<T: RealField>() {}
fn inner_product_space<V: InnerProductSpace<S>, S>() {}
fn euclidean_space<V: EuclideanSpace<S>, S: RealField>() {}
// OFFEN: Einheitswurzeln – Signatur für FFT (Schritt 3)

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

#[test]
fn inner_products() {
    euclidean_space::<[f64; 3], f64>();
    inner_product_space::<[Complex<f64>; 2], Complex<f64>>();
}

#[test]
fn elementary_and_autodiff() {
    real_field::<f64>();
    elementary_ring::<Complex<f64>>();
    elementary_ring::<Dual<f64>>();
    elementary_ring::<Dual<Dual<f64>>>();
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
fn first_derivative<T: CommutativeRing>(f: impl Fn(&Dual<T>) -> Dual<T>, x: T) -> (T, T) {
    derivative(f, x)
}
fn newton_auto<T: OrderedField + PartialEq + Clone>(
    f: impl Fn(&[Dual<T>]) -> Vec<Dual<T>>,
    x0: Vec<T>,
    tol: &T,
) -> Option<Vec<T>> {
    newton_autodiff(f, x0, tol, 100)
}
fn length<V: EuclideanSpace<S>, S: RealField>(v: &V) -> S {
    norm(v)
}
fn cg<V: EuclideanSpace<S>, S: RealField + Clone>(
    a: impl Fn(&V) -> V,
    b: &V,
    x0: V,
    tol: &S,
) -> Option<V> {
    conjugate_gradient(a, b, x0, tol, 1000)
}
fn ode<V: Module<S>, S: OrderedField>(f: impl Fn(&S, &V) -> V, t: &S, y: &V, h: &S) -> V {
    rk4_step(f, t, y, h)
}
fn n_body<V: EuclideanSpace<S>, S: RealField>(
    x: &[V],
    v: &[V],
    m: &[S],
    g: &S,
    h: &S,
) -> (Vec<V>, Vec<V>) {
    verlet_step(|p| gravity(p, m, g), x, v, h)
}
// OFFEN: Polynomauswertung (Horner) – geht schon mit `Ring`, fehlt nur als Funktion

// --- Projekte ---------------------------------------------------------------
//
// OFFEN: IAPWS-95/06/10, trockene Luft – Basis vollständig (RealField + AutoDiff),
//        fehlt nur Implementierung
// OFFEN: Neuronale Netze (Layer, Backprop, Adam, Dropout, Faltung)
//        – Faltung über FFT (Schritt 3), Rückwärts-AutoDiff (Implementierung);
//          Layer verschiedener Größe: siehe BASIS.md, „Grenzen der Signatur“
// ERREICHT: 2-/3-Körper-Problem – `n_body` oben, Achter-Bahn in tests/geometry.rs
// OFFEN: Navier-Stokes – Basis bereit bis auf evtl. FFT (Schritt 3);
//        Gitter und Operatoren sind Implementierung
// OFFEN: FFT 1D/2D/3D – braucht Einheitswurzeln (Schritt 3)
// OFFEN: Droste-Effekt („Logarithmus eines Bildes“) – Basis vollständig
//        (komplexes exp/ln), fehlt nur Implementierung

#[test]
fn catalog_compiles() {}
