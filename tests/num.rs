//! `Num<T>`: Operatoren, Konstanten und generische Formeln.

use math_traits::autodiff::{Dual, partials_2};
use math_traits::impls::cayley_dickson::Complex;
use math_traits::num::{Num, Real};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-12 * (1.0 + b.abs())
}

/// Eine Formel, einmal geschrieben.
fn f<R: Real>(x: R, y: R) -> R {
    x * x * y + (x * y).sin() - y.powi(-2) * 2.5 + x.powf(1.5) / 3.0
}

#[test]
fn operators_and_constants() {
    let x = Num(3.0);
    assert_eq!((x + 1.0).0, 4.0);
    assert_eq!((2.0 - x).0, -1.0);
    assert_eq!((x * 2.0 / 4.0).0, 1.5);
    assert_eq!((1.0 / x).0, 1.0 / 3.0);
    assert_eq!((-x).0, -3.0);
    assert_eq!(x.powi(-2).0, 1.0 / 9.0);
    assert!(Num(1.0) < Num(2.0));
    assert_eq!(
        [Num(1.0), Num(2.0), Num(3.0)]
            .into_iter()
            .sum::<Num<f64>>()
            .0,
        6.0
    );
    assert_eq!(Num(0.0).try_ln(), None);
}

#[test]
#[should_panic(expected = "Division")]
fn division_by_zero_panics() {
    let _ = Num(1.0) / Num(0.0);
}

#[test]
fn same_formula_for_values_and_derivatives() {
    let (x, y) = (1.3, 0.7);
    let v = f(Num(x), Num(y)).0;
    let p = partials_2(|a, b| f(Num(a), Num(b)).0, x, y);
    assert!(close(p.f, v));
    // analytisch
    let fx = 2.0 * x * y + y * (x * y).cos() + 0.5 * x.sqrt();
    let fy = x * x + x * (x * y).cos() + 5.0 * y.powi(-3);
    let fxx = 2.0 * y - y * y * (x * y).sin() + 0.25 / x.sqrt();
    let fxy = 2.0 * x + (x * y).cos() - x * y * (x * y).sin();
    let fyy = -x * x * (x * y).sin() - 15.0 * y.powi(-4);
    assert!(close(p.fx, fx) && close(p.fy, fy));
    assert!(close(p.fxx, fxx) && close(p.fxy, fxy) && close(p.fyy, fyy));
}

#[test]
fn also_complex() {
    type C = Complex<f64>;
    let z = f(Num(C::new(1.0, 0.0)), Num(C::new(2.0, 0.0))).0;
    let r = f(Num(1.0), Num(2.0)).0;
    assert!(close(z.re, r) && z.im.abs() < 1e-14);
    let _: Num<Dual<C>> = f(
        Num(Dual::variable(C::new(1.0, 1.0))),
        Num(Dual::constant(C::new(2.0, 0.0))),
    );
}
