//! Gleichungslöser: Bisektion, Gauß-Elimination, Newton in `n` Dimensionen.
//!
//! Alle Löser sind generisch und verlangen nur Strukturen aus
//! [`crate::structures`], plus `Clone` und `PartialEq`. Das sind
//! Rust-Bedürfnisse, keine mathematischen Annahmen.
//!
//! * [`solve_linear`] braucht nur einen [`Field`]. Es läuft exakt, z. B. über
//!   `GF(p)` oder rationalen Zahlen.
//! * [`solve_linear_pivoting`], [`bisect`] und [`newton`] brauchen ein
//!   [`OrderedField`], weil sie Beträge vergleichen.

use crate::derived::{abs, sub, try_div};
use crate::signature::{Additive, HasIdentity, Multiplicative, op};
use crate::structures::{Field, OrderedField};

fn zero<T: HasIdentity<Additive>>() -> T {
    T::identity()
}

fn one<T: HasIdentity<Multiplicative>>() -> T {
    T::identity()
}

/// `x ≤ 0 ≤ y` oder `y ≤ 0 ≤ x`: Vorzeichenwechsel zwischen `x` und `y`.
fn sign_change<T: OrderedField>(x: &T, y: &T) -> bool {
    let z = zero::<T>();
    (x.relates(&z) && z.relates(y)) || (y.relates(&z) && z.relates(x))
}

/// Nullstelle von `f` in `[a, b]` per Bisektion.
///
/// Voraussetzung: `f(a)` und `f(b)` haben verschiedene Vorzeichen.
/// `None`, wenn das nicht gilt oder `max_iter` nicht reicht.
///
/// Das Halbieren braucht `2 ≠ 0`. In geordneten Körpern ist das immer so,
/// denn sie haben Charakteristik 0.
pub fn bisect<T>(f: impl Fn(&T) -> T, a: T, b: T, tol: &T, max_iter: usize) -> Option<T>
where
    T: OrderedField + PartialEq + Clone,
{
    let two = op::<Additive, _>(&one::<T>(), &one());
    let (mut a, mut b) = (a, b);
    let mut fa = f(&a);
    if !sign_change(&fa, &f(&b)) {
        return None;
    }
    for _ in 0..max_iter {
        let m = try_div(&op::<Additive, _>(&a, &b), &two)?;
        if abs(&sub(&b, &a)).relates(tol) {
            return Some(m);
        }
        let fm = f(&m);
        if fm == zero() {
            return Some(m);
        }
        if sign_change(&fa, &fm) {
            b = m;
        } else {
            a = m;
            fa = fm;
        }
    }
    None
}

/// Löst `A · x = b` per Gauß-Jordan-Elimination.
/// Pivot ist das erste Element `≠ 0` in der Spalte, also exakt für jeden Körper.
pub fn solve_linear<T>(a: Vec<Vec<T>>, b: Vec<T>) -> Option<Vec<T>>
where
    T: Field + PartialEq + Clone,
{
    gauss_jordan(a, b, |m, col| (col..m.len()).find(|&r| m[r][col] != zero()))
}

/// Wie [`solve_linear`], aber mit Spaltenpivotsuche: Pivot ist das betragsgrößte
/// Element. Numerisch stabil für Gleitkommazahlen.
pub fn solve_linear_pivoting<T>(a: Vec<Vec<T>>, b: Vec<T>) -> Option<Vec<T>>
where
    T: OrderedField + PartialEq + Clone,
{
    gauss_jordan(a, b, |m, col| {
        (col..m.len())
            .filter(|&r| m[r][col] != zero())
            .reduce(|best, r| {
                if abs(&m[best][col]).relates(&abs(&m[r][col])) {
                    r
                } else {
                    best
                }
            })
    })
}

fn gauss_jordan<T>(
    mut a: Vec<Vec<T>>,
    mut b: Vec<T>,
    choose_pivot: impl Fn(&[Vec<T>], usize) -> Option<usize>,
) -> Option<Vec<T>>
where
    T: Field + PartialEq + Clone,
{
    let n = b.len();
    if a.len() != n || a.iter().any(|row| row.len() != n) {
        return None;
    }
    for col in 0..n {
        let p = choose_pivot(&a, col)?;
        a.swap(col, p);
        b.swap(col, p);

        let inv = try_div(&one::<T>(), &a[col][col])?;
        for x in a[col].iter_mut() {
            *x = op::<Multiplicative, _>(&inv, x);
        }
        b[col] = op::<Multiplicative, _>(&inv, &b[col]);

        for r in 0..n {
            if r == col || a[r][col] == zero() {
                continue;
            }
            let factor = a[r][col].clone();
            let pivot_row = a[col].clone();
            for (x, p) in a[r].iter_mut().zip(&pivot_row) {
                *x = sub(x, &op::<Multiplicative, _>(&factor, p));
            }
            let t = op::<Multiplicative, _>(&factor, &b[col]);
            b[r] = sub(&b[r], &t);
        }
    }
    Some(b)
}

/// Newton-Verfahren für `F(x) = 0` mit `F: Tⁿ → Tⁿ`.
///
/// In jedem Schritt wird `J(x) · Δ = F(x)` gelöst und `x ← x − Δ` gesetzt.
/// Abbruch, sobald `max |Δᵢ| ≤ tol`. `None` bei singulärer Jacobi-Matrix
/// oder wenn `max_iter` nicht reicht.
pub fn newton<T>(
    f: impl Fn(&[T]) -> Vec<T>,
    jacobian: impl Fn(&[T]) -> Vec<Vec<T>>,
    x0: Vec<T>,
    tol: &T,
    max_iter: usize,
) -> Option<Vec<T>>
where
    T: OrderedField + PartialEq + Clone,
{
    let mut x = x0;
    for _ in 0..max_iter {
        let delta = solve_linear_pivoting(jacobian(&x), f(&x))?;
        for (xi, di) in x.iter_mut().zip(&delta) {
            *xi = sub(xi, di);
        }
        if delta.iter().all(|d| abs(d).relates(tol)) {
            return Some(x);
        }
    }
    None
}
