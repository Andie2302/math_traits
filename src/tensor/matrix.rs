//! Lineare Algebra auf Matrizen fester Größe.
//!
//! Jede Funktion verlangt nur die Struktur, die sie mathematisch braucht:
//!
//! | Funktion | braucht |
//! |---|---|
//! | [`transpose`], [`diagonal`], Dreiecks-/Diagonalprüfung | nichts (`Clone`, `PartialEq`) |
//! | [`adjoint`] (konjugiert transponiert) | Konjugation |
//! | [`trace`] | additives Monoid |
//! | [`det`], [`lu`], [`inverse`] | [`Field`] (exakt, z. B. über `GF(p)` oder ℚ) |
//! | [`lu_pivoting`] | [`OrderedField`] (Pivot = betragsgrößtes Element) |
//! | [`qr`] | [`RealField`] (Wurzel für die Spaltennormen) |
//! | [`eigen_symmetric`] (Jacobi-Verfahren) | [`RealField`] |
//!
//! Die Schur-Zerlegung allgemeiner Matrizen braucht komplexe Eigenwerte, also
//! `Complex<T>` mit `T: RealField`. Das ist ein [`Field`] mit Wurzeln und
//! Skalarprodukt, und damit ist alles Nötige in der Basis vorhanden. Für
//! symmetrische Matrizen ist die Schur-Zerlegung genau [`eigen_symmetric`].

use super::{SquareMatrix, Tensor1, Tensor2};
use crate::derived::{abs, sub, try_div};
use crate::signature::{Additive, BinaryOp, HasConjugate, HasIdentity, Multiplicative, op};
use crate::structures::{Field, Monoid, OrderedField, RealField};

fn add<T: BinaryOp<Additive>>(a: &T, b: &T) -> T {
    op::<Additive, _>(a, b)
}
fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}
fn zero<T: HasIdentity<Additive>>() -> T {
    T::identity()
}
fn one<T: HasIdentity<Multiplicative>>() -> T {
    T::identity()
}

// --- Struktur ohne Arithmetik -------------------------------------------------------

/// `Aᵀ`
pub fn transpose<T: Clone, const M: usize, const N: usize>(
    a: &Tensor2<T, M, N>,
) -> Tensor2<T, N, M> {
    Tensor2(core::array::from_fn(|i| {
        core::array::from_fn(|j| a.0[j][i].clone())
    }))
}

/// `A* = conj(Aᵀ)`
pub fn adjoint<T: HasConjugate, const M: usize, const N: usize>(
    a: &Tensor2<T, M, N>,
) -> Tensor2<T, N, M> {
    Tensor2(core::array::from_fn(|i| {
        core::array::from_fn(|j| a.0[j][i].conj())
    }))
}

/// Hauptdiagonale.
pub fn diagonal<T: Clone, const N: usize>(a: &SquareMatrix<T, N>) -> Tensor1<T, N> {
    Tensor1(core::array::from_fn(|i| a.0[i][i].clone()))
}

/// Diagonalmatrix aus einem Vektor.
pub fn from_diagonal<T: Clone + HasIdentity<Additive>, const N: usize>(
    d: &Tensor1<T, N>,
) -> SquareMatrix<T, N> {
    Tensor2(core::array::from_fn(|i| {
        core::array::from_fn(|j| if i == j { d.0[i].clone() } else { zero() })
    }))
}

/// Spur `Σ aᵢᵢ`.
pub fn trace<T: Monoid<Additive>, const N: usize>(a: &SquareMatrix<T, N>) -> T {
    (0..N).fold(zero(), |acc, i| add(&acc, &a.0[i][i]))
}

/// Alle Einträge unterhalb der Diagonale sind `0`.
pub fn is_upper_triangular<T, const M: usize, const N: usize>(a: &Tensor2<T, M, N>) -> bool
where
    T: HasIdentity<Additive> + PartialEq,
{
    (0..M).all(|i| (0..N.min(i)).all(|j| a.0[i][j] == zero()))
}

/// Alle Einträge oberhalb der Diagonale sind `0`.
pub fn is_lower_triangular<T, const M: usize, const N: usize>(a: &Tensor2<T, M, N>) -> bool
where
    T: HasIdentity<Additive> + PartialEq,
{
    (0..M).all(|i| (i + 1..N).all(|j| a.0[i][j] == zero()))
}

/// Nur die Diagonale ist besetzt.
pub fn is_diagonal<T, const M: usize, const N: usize>(a: &Tensor2<T, M, N>) -> bool
where
    T: HasIdentity<Additive> + PartialEq,
{
    is_upper_triangular(a) && is_lower_triangular(a)
}

// --- LU-Zerlegung ------------------------------------------------------------------

/// `P·A = L·U`, kompakt gespeichert: `L` (Diagonale 1) unter, `U` auf und
/// über der Diagonale. `perm[i]` ist die Zeile von `A`, die in Zeile `i` steht.
#[derive(Clone, Debug, PartialEq)]
pub struct Lu<T, const N: usize> {
    pub packed: SquareMatrix<T, N>,
    pub perm: [usize; N],
    /// Ungerade Anzahl Zeilenvertauschungen (Vorzeichen der Determinante).
    pub odd: bool,
}

fn lu_with<T, const N: usize>(
    a: &SquareMatrix<T, N>,
    choose: impl Fn(&SquareMatrix<T, N>, usize) -> Option<usize>,
) -> Option<Lu<T, N>>
where
    T: Field + PartialEq + Clone,
{
    let mut m = a.clone();
    let mut perm: [usize; N] = core::array::from_fn(|i| i);
    let mut odd = false;
    for k in 0..N {
        let p = choose(&m, k)?;
        if p != k {
            m.0.swap(p, k);
            perm.swap(p, k);
            odd = !odd;
        }
        for i in k + 1..N {
            let f = try_div(&m.0[i][k], &m.0[k][k])?;
            for j in k + 1..N {
                let t = mul(&f, &m.0[k][j]);
                m.0[i][j] = sub(&m.0[i][j], &t);
            }
            m.0[i][k] = f;
        }
    }
    Some(Lu {
        packed: m,
        perm,
        odd,
    })
}

/// LU-Zerlegung über jedem Körper (Pivot: erstes Element `≠ 0`), exakt.
/// `None` für singuläre Matrizen.
pub fn lu<T: Field + PartialEq + Clone, const N: usize>(
    a: &SquareMatrix<T, N>,
) -> Option<Lu<T, N>> {
    lu_with(a, |m, k| (k..N).find(|&i| m.0[i][k] != zero()))
}

/// LU-Zerlegung mit Spaltenpivotsuche, numerisch stabil für Gleitkomma.
pub fn lu_pivoting<T: OrderedField + PartialEq + Clone, const N: usize>(
    a: &SquareMatrix<T, N>,
) -> Option<Lu<T, N>> {
    lu_with(a, |m, k| {
        (k..N).filter(|&i| m.0[i][k] != zero()).reduce(|b, i| {
            if abs(&m.0[b][k]).relates(&abs(&m.0[i][k])) {
                i
            } else {
                b
            }
        })
    })
}

impl<T: Field + PartialEq + Clone, const N: usize> Lu<T, N> {
    /// `L` mit Einsen auf der Diagonale.
    pub fn l(&self) -> SquareMatrix<T, N> {
        Tensor2(core::array::from_fn(|i| {
            core::array::from_fn(|j| match j.cmp(&i) {
                core::cmp::Ordering::Less => self.packed.0[i][j].clone(),
                core::cmp::Ordering::Equal => one(),
                core::cmp::Ordering::Greater => zero(),
            })
        }))
    }

    /// `U`
    pub fn u(&self) -> SquareMatrix<T, N> {
        Tensor2(core::array::from_fn(|i| {
            core::array::from_fn(|j| {
                if j >= i {
                    self.packed.0[i][j].clone()
                } else {
                    zero()
                }
            })
        }))
    }

    /// `det A = ± Π uᵢᵢ`
    pub fn det(&self) -> T {
        let d = (0..N).fold(one(), |acc: T, i| mul(&acc, &self.packed.0[i][i]));
        if self.odd { d.inverse() } else { d }
    }

    /// Löst `A·x = b` per Vorwärts- und Rückwärtseinsetzen.
    pub fn solve(&self, b: &Tensor1<T, N>) -> Option<Tensor1<T, N>> {
        let m = &self.packed.0;
        let mut y: [T; N] = core::array::from_fn(|i| b.0[self.perm[i]].clone());
        for i in 0..N {
            for j in 0..i {
                let t = mul(&m[i][j], &y[j]);
                y[i] = sub(&y[i], &t);
            }
        }
        for i in (0..N).rev() {
            for j in i + 1..N {
                let t = mul(&m[i][j], &y[j]);
                y[i] = sub(&y[i], &t);
            }
            y[i] = try_div(&y[i], &m[i][i])?;
        }
        Some(Tensor1(y))
    }
}

/// Determinante über jedem Körper, exakt. `0` für singuläre Matrizen.
pub fn det<T: Field + PartialEq + Clone, const N: usize>(a: &SquareMatrix<T, N>) -> T {
    lu(a).map_or_else(zero, |d| d.det())
}

/// Inverse über jedem Körper, `None` für singuläre Matrizen.
pub fn inverse<T: Field + PartialEq + Clone, const N: usize>(
    a: &SquareMatrix<T, N>,
) -> Option<SquareMatrix<T, N>> {
    let d = lu(a)?;
    let mut cols: [Tensor1<T, N>; N] =
        core::array::from_fn(|_| Tensor1(core::array::from_fn(|_| zero())));
    for (j, col) in cols.iter_mut().enumerate() {
        let e = Tensor1(core::array::from_fn(
            |i| if i == j { one() } else { zero() },
        ));
        *col = d.solve(&e)?;
    }
    Some(Tensor2(core::array::from_fn(|i| {
        core::array::from_fn(|j| cols[j].0[i].clone())
    })))
}

// --- QR-Zerlegung ------------------------------------------------------------------

/// Dünne QR-Zerlegung `A = Q·R` per modifiziertem Gram-Schmidt:
/// `Q` hat orthonormale Spalten, `R` ist obere Dreiecksmatrix.
/// `None`, wenn die Spalten linear abhängig sind.
pub fn qr<T: RealField + PartialEq + Clone, const M: usize, const N: usize>(
    a: &Tensor2<T, M, N>,
) -> Option<(Tensor2<T, M, N>, SquareMatrix<T, N>)> {
    let mut q = a.clone();
    let mut r: SquareMatrix<T, N> =
        Tensor2(core::array::from_fn(|_| core::array::from_fn(|_| zero())));
    for j in 0..N {
        for k in 0..j {
            let dot = (0..M).fold(zero(), |acc: T, i| add(&acc, &mul(&q.0[i][k], &q.0[i][j])));
            for i in 0..M {
                let t = mul(&dot, &q.0[i][k]);
                q.0[i][j] = sub(&q.0[i][j], &t);
            }
            r.0[k][j] = dot;
        }
        let norm_sq = (0..M).fold(zero(), |acc: T, i| add(&acc, &mul(&q.0[i][j], &q.0[i][j])));
        let norm = norm_sq.sqrt()?;
        if norm == zero() {
            return None;
        }
        for i in 0..M {
            q.0[i][j] = try_div(&q.0[i][j], &norm)?;
        }
        r.0[j][j] = norm;
    }
    Some((q, r))
}

// --- Eigenwerte symmetrischer Matrizen (Jacobi-Verfahren) --------------------------

/// Eigenwerte und Eigenvektoren (Spalten von `V`) einer **symmetrischen**
/// Matrix per zyklischem Jacobi-Verfahren: `A = V·diag(λ)·Vᵀ`.
/// Abbruch, sobald alle Nebendiagonal-Elemente betragsmäßig `≤ tol` sind.
pub fn eigen_symmetric<T: RealField + PartialEq + Clone, const N: usize>(
    a: &SquareMatrix<T, N>,
    tol: &T,
    max_sweeps: usize,
) -> Option<(Tensor1<T, N>, SquareMatrix<T, N>)> {
    let mut a = a.clone();
    let mut v: SquareMatrix<T, N> = <SquareMatrix<T, N> as HasIdentity<Multiplicative>>::identity();
    let two = add(&one::<T>(), &one());
    for _ in 0..=max_sweeps {
        let off_ok = (0..N).all(|p| (0..N).all(|q| p == q || abs(&a.0[p][q]).relates(tol)));
        if off_ok {
            return Some((diagonal(&a), v));
        }
        for p in 0..N {
            for q in p + 1..N {
                if a.0[p][q] == zero() {
                    continue;
                }
                // θ = (a_qq − a_pp) / (2·a_pq), t = sign(θ) / (|θ| + sqrt(θ² + 1))
                let theta = try_div(&sub(&a.0[q][q], &a.0[p][p]), &mul(&two, &a.0[p][q]))?;
                let root = add(&mul(&theta, &theta), &one()).sqrt()?;
                let t = try_div(&one(), &add(&abs(&theta), &root))?;
                let t = if zero::<T>().relates(&theta) {
                    t
                } else {
                    t.inverse()
                };
                let c = try_div(&one(), &add(&mul(&t, &t), &one()).sqrt()?)?;
                let s = mul(&t, &c);
                // J = Drehung in der (p, q)-Ebene; A ← Jᵀ·A·J, V ← V·J
                let mut j: SquareMatrix<T, N> =
                    <SquareMatrix<T, N> as HasIdentity<Multiplicative>>::identity();
                j.0[p][p] = c.clone();
                j.0[q][q] = c.clone();
                j.0[p][q] = s.clone();
                j.0[q][p] = s.inverse();
                let jt = transpose(&j);
                a = mul(&mul(&jt, &a), &j);
                v = mul(&v, &j);
            }
        }
    }
    None
}
