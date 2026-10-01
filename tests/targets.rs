//! Zielkatalog: Wann ist die Basis abgeschlossen?
//!
//! Jede Zeile hier ist ein Ziel aus `BASIS.md`. Ein Ziel gilt als erreicht,
//! wenn es *nur* mit Bounds aus `structures` kompiliert. Offene Ziele stehen
//! als `// OFFEN:` dabei. Sind keine mehr offen, ist die Basis für diese
//! Version abgeschlossen.

#![allow(dead_code)]

use math_traits::derived::*;
use math_traits::signature::*;
use math_traits::structures::*;

// --- Strukturen -----------------------------------------------------------

fn monoid<T: Monoid<Additive>>() {}
fn group<T: Group<Additive>>() {}
fn abelian_group<T: AbelianGroup<Additive>>() {}
fn ring<T: Ring>() {}
fn commutative_ring<T: CommutativeRing>() {}
fn integral_domain<T: IntegralDomain>() {}
fn division_ring<T: DivisionRing>() {}
fn field<T: Field>() {}
fn lattice<T: Lattice>() {}
fn total_order<T: TotalOrder<LessEq>>() {}
// OFFEN: OrderedField    – braucht ein Gesetz `0 ≤ x ∧ 0 ≤ y ⇒ 0 ≤ x·y`
// OFFEN: EuclideanDomain – braucht Division mit Rest in der Signatur
// OFFEN: Module / VectorSpace – braucht externe Operation `S × V → V`
// OFFEN: NormedSpace     – baut auf VectorSpace + OrderedField auf

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
// OFFEN: gcd               – braucht EuclideanDomain
// OFFEN: Polynomauswertung – geht schon (Ring), fehlt nur als Funktion
// OFFEN: lineare Gleichungssysteme (Gauß) – braucht Field + Vektoren

#[test]
fn catalog_compiles() {}
