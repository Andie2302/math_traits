//! Elementarfunktionen und AutoDiff.

use math_traits::autodiff::*;
use math_traits::derived::{powf, sigmoid, sub, tanh};
use math_traits::impls::cayley_dickson::Complex;
use math_traits::laws::InverseWhereDefined;
use math_traits::signature::*;
use math_traits::structures::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9 * (1.0 + b.abs())
}

fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}

/// `f(x) = sin(x) · eˣ`, einmal generisch geschrieben.
fn f<T: ElementaryRing>(x: &T) -> T {
    mul(&x.sin(), &x.exp())
}

#[test]
fn structures() {
    fn real<T: RealField>() {}
    fn elementary<T: ElementaryRing>() {}
    real::<f64>();
    real::<f32>();
    elementary::<Complex<f64>>();
    elementary::<Dual<f64>>();
    elementary::<Dual<Dual<f64>>>();
    elementary::<Dual<Complex<f64>>>();
}

#[test]
fn first_and_second_derivative() {
    for x in [-1.3, 0.0, 0.7, 2.5] {
        let (v, d) = derivative(f, x);
        assert!(close(v, x.sin() * x.exp()));
        assert!(close(d, (x.cos() + x.sin()) * x.exp()));

        let (v2, d2, dd2) = second_derivative(f, x);
        assert!(close(v2, v) && close(d2, d));
        assert!(close(dd2, 2.0 * x.cos() * x.exp()));
    }
}

#[test]
fn derivatives_of_partial_functions() {
    // d/dx ln(x) = 1/x, d/dx sqrt(x) = 1/(2 sqrt x), d/dx x^2.5 = 2.5 x^1.5
    let x = 1.7;
    let ln = Dual::variable(x).ln().unwrap();
    assert!(close(ln.eps, 1.0 / x));
    let sq = Dual::variable(x).sqrt().unwrap();
    assert!(close(sq.eps, 0.5 / x.sqrt()));
    let p = powf(&Dual::variable(x), &Dual::constant(2.5)).unwrap();
    assert!(close(p.eps, 2.5 * x.powf(1.5)));
    assert_eq!(Dual::variable(-1.0).ln(), None);
}

#[test]
fn activations() {
    let x = 0.3;
    let t = tanh(&Dual::variable(x)).unwrap();
    assert!(close(t.re, x.tanh()) && close(t.eps, 1.0 - x.tanh().powi(2)));
    let s = sigmoid(&Dual::variable(x)).unwrap();
    let sx = 1.0 / (1.0 + (-x).exp());
    assert!(close(s.re, sx) && close(s.eps, sx * (1.0 - sx)));
}

#[test]
fn jacobian_and_gradient() {
    // F(x, y) = (x²·y, sin(x) + y)
    let fv = |v: &[Dual<f64>]| {
        vec![
            mul(&mul(&v[0], &v[0]), &v[1]),
            op::<Additive, _>(&v[0].sin(), &v[1]),
        ]
    };
    let (vals, j) = jacobian(fv, &[2.0, 3.0]);
    assert_eq!(vals[0], 12.0);
    assert_eq!(j[0], vec![12.0, 4.0]);
    assert!(close(j[1][0], 2f64.cos()) && j[1][1] == 1.0);

    // f(x, y) = x·y + exp(x) ⇒ ∇f = (y + eˣ, x)
    let (v, g) = gradient(
        |v| op::<Additive, _>(&mul(&v[0], &v[1]), &v[0].exp()),
        &[0.5, 2.0],
    );
    assert!(close(v, 1.0 + 0.5f64.exp()));
    assert!(close(g[0], 2.0 + 0.5f64.exp()) && g[1] == 0.5);
}

#[test]
fn newton_without_handwritten_jacobian() {
    // x² + y² = 4, x = y
    let four = Dual::constant(4.0);
    let f = |v: &[Dual<f64>]| {
        vec![
            sub(
                &op::<Additive, _>(&mul(&v[0], &v[0]), &mul(&v[1], &v[1])),
                &four,
            ),
            sub(&v[0], &v[1]),
        ]
    };
    let x = newton_autodiff(f, vec![1.0, 0.5], &1e-12, 50).unwrap();
    assert!(close(x[0], 2f64.sqrt()) && close(x[1], 2f64.sqrt()));
}

/// Ein Helmholtz-Ansatz im Stil von IAPWS: `φ(τ) = n₀ + n₁·τ + n₂·ln τ + n₃·e^(−τ)`.
/// Die Wärmekapazität hängt an `τ²·φ''(τ)`. Die zweite Ableitung kommt per AutoDiff.
#[test]
fn iapws_style_second_derivative() {
    fn phi<T: ElementaryRing + InverseWhereDefined<Multiplicative> + Clone>(
        tau: &T,
        n: &[T; 4],
    ) -> T {
        let ln = tau.ln().expect("τ > 0");
        let e = tau.inverse().exp();
        [
            n[0].clone(),
            mul(&n[1], tau),
            mul(&n[2], &ln),
            mul(&n[3], &e),
        ]
        .into_iter()
        .fold(<T as HasIdentity<Additive>>::identity(), |a, b| {
            op::<Additive, _>(&a, &b)
        })
    }
    let n = [1.5, -0.3, 2.0, 0.8];
    let tau = 1.25;
    let nd = n.map(|c| Dual::constant(Dual::constant(c)));
    let (_, d1, d2) = second_derivative(|t| phi(t, &nd), tau);
    assert!(close(d1, n[1] + n[2] / tau - n[3] * (-tau).exp()));
    assert!(close(d2, -n[2] / (tau * tau) + n[3] * (-tau).exp()));
}

#[test]
fn complex_elementary() {
    type C = Complex<f64>;
    let ipi = C::new(0.0, std::f64::consts::PI);
    let e = ipi.exp(); // Euler: e^{iπ} = −1
    assert!(close(e.re, -1.0) && e.im.abs() < 1e-15);

    let minus_one = C::new(-1.0, 0.0);
    assert_eq!(minus_one.sqrt(), Some(C::new(0.0, 1.0)));

    let z = C::new(0.3, -2.0);
    let back = z.ln().unwrap().exp();
    assert!(close(back.re, z.re) && close(back.im, z.im));
    assert_eq!(C::new(0.0, 0.0).ln(), None);

    // sin² + cos² = 1 auch komplex
    let s = z.sin();
    let c = z.cos();
    let one = op::<Additive, _>(&mul(&s, &s), &mul(&c, &c));
    assert!(close(one.re, 1.0) && one.im.abs() < 1e-12);
}

#[test]
fn atan2_and_derivatives_through_complex_functions() {
    use math_traits::laws::Atan2InvertsSinCos;
    for t in [-3.0, -1.0, 0.0, 0.5, 3.1] {
        assert!(<f64 as Atan2InvertsSinCos<Multiplicative>>::holds(&t));
    }
    // d/dx atan2(1, x) = −1 / (1 + x²)
    let x = 0.7;
    let a = Dual::constant(1.0).atan2(&Dual::variable(x));
    assert!(close(a.eps, -1.0 / (1.0 + x * x)));

    // f(x) = ln(x + i):  d/dx Re = x/(x²+1),  d/dx Im = −1/(x²+1)
    type CD = Complex<Dual<f64>>;
    let z = CD::new(Dual::variable(x), Dual::constant(1.0));
    let l = z.ln().unwrap();
    assert!(close(l.re.eps, x / (x * x + 1.0)));
    assert!(close(l.im.eps, -1.0 / (x * x + 1.0)));
    // exp ∘ ln = id, auch mit Ableitung
    let back = l.exp();
    assert!(close(back.re.re, x) && close(back.re.eps, 1.0) && close(back.im.re, 1.0));
}
