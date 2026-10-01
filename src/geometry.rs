//! Geometrie und Dynamik auf Räumen mit Skalarprodukt.
//!
//! * Norm, Abstand, Normieren: [`EuclideanSpace`] über einem [`RealField`].
//!   Die Norm ist eine **Folgerung**: `‖v‖ = sqrt(⟨v, v⟩)`. Die Wurzel
//!   existiert, weil `⟨v, v⟩ ≥ 0` (Gesetz) und nichtnegative Zahlen eine
//!   Wurzel haben (Gesetz).
//! * Konjugierte Gradienten: lösen `A·x = b` für symmetrisch positiv
//!   definites `A`, nur über das Skalarprodukt. Die Matrix wird nie gebraucht,
//!   nur ihre Anwendung. Das passt später auf Gitter (Navier-Stokes).
//! * RK4 und Velocity-Verlet: gewöhnliche Differentialgleichungen auf
//!   beliebigen Modulen, z. B. das n-Körper-Problem.

use crate::derived::{sub, try_div};
use crate::signature::{Additive, BinaryOp, HasIdentity, InnerProduct, Multiplicative, op};
use crate::structures::{CommutativeRing, EuclideanSpace, Module, OrderedField, RealField};

fn add<T: BinaryOp<Additive>>(a: &T, b: &T) -> T {
    op::<Additive, _>(a, b)
}

fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}

fn from_u8<S: OrderedField>(n: u8) -> S {
    let one = <S as HasIdentity<Multiplicative>>::identity();
    (0..n).fold(<S as HasIdentity<Additive>>::identity(), |acc, _| {
        add(&acc, &one)
    })
}

// --- Norm ---------------------------------------------------------------------

/// `‖v‖² = ⟨v, v⟩`
pub fn norm_sq<V: InnerProduct<S>, S>(v: &V) -> S {
    v.inner(v)
}

/// `‖v‖ = sqrt(⟨v, v⟩)`
pub fn norm<V: EuclideanSpace<S>, S: RealField>(v: &V) -> S {
    norm_sq(v)
        .sqrt()
        .expect("⟨v, v⟩ ≥ 0 hat nach den Gesetzen eine Wurzel")
}

/// `‖v − w‖`
pub fn distance<V: EuclideanSpace<S>, S: RealField>(v: &V, w: &V) -> S {
    norm(&sub(v, w))
}

/// `v / ‖v‖`, `None` für `v = 0`.
pub fn normalize<V: EuclideanSpace<S>, S: RealField>(v: &V) -> Option<V> {
    let one = <S as HasIdentity<Multiplicative>>::identity();
    try_div(&one, &norm(v)).map(|inv| v.scale(&inv))
}

/// Kreuzprodukt in drei Dimensionen.
pub fn cross<T: CommutativeRing>(a: &[T; 3], b: &[T; 3]) -> [T; 3] {
    let c = |i: usize, j: usize| sub(&mul(&a[i], &b[j]), &mul(&a[j], &b[i]));
    [c(1, 2), c(2, 0), c(0, 1)]
}

// --- Konjugierte Gradienten ----------------------------------------------------

/// Löst `A·x = b` für symmetrisch positiv definites `A` (gegeben als
/// Anwendung `apply(v) = A·v`). Abbruch, sobald `‖r‖ ≤ tol`.
pub fn conjugate_gradient<V, S>(
    apply: impl Fn(&V) -> V,
    b: &V,
    x0: V,
    tol: &S,
    max_iter: usize,
) -> Option<V>
where
    V: EuclideanSpace<S>,
    S: RealField + Clone,
{
    let tol_sq = mul(tol, tol);
    let mut x = x0;
    let mut r = sub(b, &apply(&x));
    let mut p = add(&r, &V::identity());
    let mut rs = norm_sq(&r);
    for _ in 0..=max_iter {
        if rs.relates(&tol_sq) {
            return Some(x);
        }
        let ap = apply(&p);
        let alpha = try_div(&rs, &p.inner(&ap))?;
        x = add(&x, &p.scale(&alpha));
        r = sub(&r, &ap.scale(&alpha));
        let rs_new = norm_sq(&r);
        let beta = try_div(&rs_new, &rs)?;
        p = add(&r, &p.scale(&beta));
        rs = rs_new;
    }
    None
}

// --- Gewöhnliche Differentialgleichungen --------------------------------------------

/// Ein Schritt des klassischen Runge-Kutta-Verfahrens 4. Ordnung für
/// `y' = f(t, y)`.
pub fn rk4_step<V, S>(f: impl Fn(&S, &V) -> V, t: &S, y: &V, h: &S) -> V
where
    V: Module<S>,
    S: OrderedField,
{
    let two: S = from_u8(2);
    let half_h = try_div(h, &two).expect("2 ≠ 0");
    let sixth_h = try_div(h, &from_u8(6)).expect("6 ≠ 0");
    let t_half = add(t, &half_h);

    let k1 = f(t, y);
    let k2 = f(&t_half, &add(y, &k1.scale(&half_h)));
    let k3 = f(&t_half, &add(y, &k2.scale(&half_h)));
    let k4 = f(&add(t, h), &add(y, &k3.scale(h)));

    let sum = add(&add(&k1, &k2.scale(&two)), &add(&k3.scale(&two), &k4));
    add(y, &sum.scale(&sixth_h))
}

/// Ein Schritt Velocity-Verlet für `x'' = a(x)` mit vielen Körpern.
/// Symplektisch: Die Energie driftet auch über lange Zeiten kaum.
pub fn verlet_step<V, S>(
    accel: impl Fn(&[V]) -> Vec<V>,
    positions: &[V],
    velocities: &[V],
    h: &S,
) -> (Vec<V>, Vec<V>)
where
    V: Module<S>,
    S: OrderedField,
{
    let half_h = try_div(h, &from_u8(2)).expect("2 ≠ 0");
    let a0 = accel(positions);
    let v_half: Vec<V> = velocities
        .iter()
        .zip(&a0)
        .map(|(v, a)| add(v, &a.scale(&half_h)))
        .collect();
    let x1: Vec<V> = positions
        .iter()
        .zip(&v_half)
        .map(|(x, v)| add(x, &v.scale(h)))
        .collect();
    let a1 = accel(&x1);
    let v1 = v_half
        .iter()
        .zip(&a1)
        .map(|(v, a)| add(v, &a.scale(&half_h)))
        .collect();
    (x1, v1)
}

/// Gravitationsbeschleunigung `aᵢ = Σⱼ G·mⱼ·(xⱼ − xᵢ) / ‖xⱼ − xᵢ‖³`.
pub fn gravity<V, S>(positions: &[V], masses: &[S], g: &S) -> Vec<V>
where
    V: EuclideanSpace<S>,
    S: RealField,
{
    positions
        .iter()
        .enumerate()
        .map(|(i, xi)| {
            positions
                .iter()
                .zip(masses)
                .enumerate()
                .filter(|&(j, _)| j != i)
                .fold(V::identity(), |acc, (_, (xj, mj))| {
                    let d = sub(xj, xi);
                    let r = norm(&d);
                    let r3 = mul(&mul(&r, &r), &r);
                    let k = try_div(&mul(g, mj), &r3).expect("Körper dürfen nicht zusammenfallen");
                    add(&acc, &d.scale(&k))
                })
        })
        .collect()
}

/// Gesamtenergie `Σ ½·mᵢ·‖vᵢ‖² − Σᵢ<ⱼ G·mᵢ·mⱼ / ‖xᵢ − xⱼ‖` (zum Prüfen der
/// Energieerhaltung).
pub fn energy<V, S>(positions: &[V], velocities: &[V], masses: &[S], g: &S) -> S
where
    V: EuclideanSpace<S>,
    S: RealField,
{
    let half = try_div(&<S as HasIdentity<Multiplicative>>::identity(), &from_u8(2)).unwrap();
    let mut e = <S as HasIdentity<Additive>>::identity();
    for (v, m) in velocities.iter().zip(masses) {
        e = add(&e, &mul(&mul(&half, m), &norm_sq(v)));
    }
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let pot = try_div(
                &mul(&mul(g, &masses[i]), &masses[j]),
                &distance(&positions[i], &positions[j]),
            )
            .expect("Körper dürfen nicht zusammenfallen");
            e = add(&e, &pot.inverse());
        }
    }
    e
}
