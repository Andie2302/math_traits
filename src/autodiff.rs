//! Automatisches Differenzieren (Vorwärtsmodus) über [`Dual`].
//!
//! Funktionen schreibt man **generisch** über den Zahltyp, z. B.
//!
//! ```
//! use math_traits::structures::ElementaryRing;
//! use math_traits::signature::{HasExp, HasSinCos, Multiplicative, op};
//!
//! fn f<T: ElementaryRing>(x: &T) -> T {
//!     op::<Multiplicative, _>(&x.sin(), &x.exp()) // sin(x) · eˣ
//! }
//! ```
//!
//! Mit `T = f64` bekommt man den Wert, mit `T = Dual<f64>` zusätzlich die
//! erste Ableitung, mit `T = Dual<Dual<f64>>` die zweite.

pub use crate::impls::dual::Dual;
use crate::signature::{HasIdentity, Multiplicative};
#[cfg(feature = "alloc")]
use crate::solve::newton;
use crate::structures::CommutativeRing;
#[cfg(feature = "alloc")]
use crate::structures::OrderedField;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

/// `(f(x), f'(x))`
pub fn derivative<T: CommutativeRing>(f: impl Fn(&Dual<T>) -> Dual<T>, x: T) -> (T, T) {
    let y = f(&Dual::variable(x));
    (y.re, y.eps)
}

/// `(f(x), f'(x), f''(x))` über verschachtelte duale Zahlen.
pub fn second_derivative<T: CommutativeRing>(
    f: impl Fn(&Dual<Dual<T>>) -> Dual<Dual<T>>,
    x: T,
) -> (T, T, T) {
    let x = Dual::new(
        Dual::variable(x),
        Dual::constant(<T as HasIdentity<Multiplicative>>::identity()),
    );
    let y = f(&x);
    (y.re.re, y.re.eps, y.eps.eps)
}

#[cfg(feature = "alloc")]
/// `(F(x), J(x))` für `F: Tⁿ → Tᵐ`, mit `n` Auswertungen von `F`.
pub fn jacobian<T: CommutativeRing + Clone>(
    f: impl Fn(&[Dual<T>]) -> Vec<Dual<T>>,
    x: &[T],
) -> (Vec<T>, Vec<Vec<T>>) {
    let n = x.len();
    let mut values = Vec::new();
    let mut columns = Vec::with_capacity(n);
    for j in 0..n {
        let input: Vec<Dual<T>> = x
            .iter()
            .enumerate()
            .map(|(k, xk)| {
                if k == j {
                    Dual::variable(xk.clone())
                } else {
                    Dual::constant(xk.clone())
                }
            })
            .collect();
        let out = f(&input);
        values = out.iter().map(|y| y.re.clone()).collect();
        columns.push(out.into_iter().map(|y| y.eps).collect::<Vec<T>>());
    }
    if n == 0 {
        let out = f(&[]);
        values = out.into_iter().map(|y| y.re).collect();
    }
    let m = values.len();
    let rows = (0..m)
        .map(|i| columns.iter().map(|col| col[i].clone()).collect())
        .collect();
    (values, rows)
}

#[cfg(feature = "alloc")]
/// `(f(x), ∇f(x))` für `f: Tⁿ → T`.
pub fn gradient<T: CommutativeRing + Clone>(
    f: impl Fn(&[Dual<T>]) -> Dual<T>,
    x: &[T],
) -> (T, Vec<T>) {
    let (mut v, j) = jacobian(|x| vec![f(x)], x);
    let value = v
        .pop()
        .unwrap_or_else(<T as HasIdentity<crate::signature::Additive>>::identity);
    let grad = j.into_iter().next().unwrap_or_default();
    (value, grad)
}

#[cfg(feature = "alloc")]
/// Newton-Verfahren, bei dem die Jacobi-Matrix per AutoDiff entsteht.
pub fn newton_autodiff<T>(
    f: impl Fn(&[Dual<T>]) -> Vec<Dual<T>>,
    x0: Vec<T>,
    tol: &T,
    max_iter: usize,
) -> Option<Vec<T>>
where
    T: OrderedField + PartialEq + Clone,
{
    newton(
        |x| jacobian(&f, x).0,
        |x| jacobian(&f, x).1,
        x0,
        tol,
        max_iter,
    )
}

/// `(F(x), J(x))` für `F: Tᴺ → Tᴹ` mit festen Größen, ohne Heap.
pub fn jacobian_fixed<T, const M: usize, const N: usize>(
    f: impl Fn(&crate::tensor::Tensor1<Dual<T>, N>) -> crate::tensor::Tensor1<Dual<T>, M>,
    x: &crate::tensor::Tensor1<T, N>,
) -> (
    crate::tensor::Tensor1<T, M>,
    crate::tensor::Tensor2<T, M, N>,
)
where
    T: CommutativeRing + Clone,
{
    use crate::tensor::{Tensor1, Tensor2};
    let columns: [Tensor1<Dual<T>, M>; N] = core::array::from_fn(|j| {
        f(&Tensor1(core::array::from_fn(|k| {
            if k == j {
                Dual::variable(x.0[k].clone())
            } else {
                Dual::constant(x.0[k].clone())
            }
        })))
    });
    let values = Tensor1(core::array::from_fn(|i| {
        columns.first().map_or_else(
            || {
                f(&Tensor1(core::array::from_fn(|k| {
                    Dual::constant(x.0[k].clone())
                })))
                .0[i]
                    .re
                    .clone()
            },
            |c| c.0[i].re.clone(),
        )
    }));
    let jac = Tensor2(core::array::from_fn(|i| {
        core::array::from_fn(|j| columns[j].0[i].eps.clone())
    }));
    (values, jac)
}
