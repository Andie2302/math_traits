//! Skalarprodukt, Norm, CG, RK4 und das n-Körper-Problem.

use math_traits::geometry::*;
use math_traits::impls::cayley_dickson::Complex;
use math_traits::laws::*;
use math_traits::signature::*;
use math_traits::solve::solve_linear_pivoting;
use math_traits::structures::*;

#[test]
fn structures() {
    fn euclidean<V: EuclideanSpace<S>, S: RealField>() {}
    fn hermitian<V: InnerProductSpace<S>, S>() {}
    euclidean::<[f64; 3], f64>();
    euclidean::<[f32; 2], f32>();
    hermitian::<[Complex<f64>; 4], Complex<f64>>();
    hermitian::<[i64; 3], i64>();
}

#[test]
fn hermitian_laws_exact_over_gaussian_integers() {
    type C = Complex<i64>;
    let vs: Vec<[C; 2]> = vec![
        [C::new(1, 2), C::new(-3, 0)],
        [C::new(0, -1), C::new(2, 5)],
        [C::new(4, 4), C::new(-1, 1)],
    ];
    let ss = [C::new(2, -1), C::new(0, 3), C::new(-1, 0)];
    for u in &vs {
        for v in &vs {
            assert!(<[C; 2] as InnerConjugateSymmetric<C>>::holds(u, v));
            for s in &ss {
                assert!(<[C; 2] as InnerHomogeneous<C>>::holds(s, u, v));
            }
            for w in &vs {
                assert!(<[C; 2] as InnerAdditive<C>>::holds(u, v, w));
            }
        }
    }
    // ⟨v, v⟩ ist reell, obwohl die Einträge komplex sind.
    let vv = vs[0].inner(&vs[0]);
    assert_eq!(vv, C::new(1 + 4 + 9, 0));
}

#[test]
fn norm_and_friends() {
    assert_eq!(norm(&[3.0, 4.0]), 5.0);
    assert_eq!(distance(&[1.0, 1.0, 1.0], &[1.0, 4.0, 5.0]), 5.0);
    let n = normalize(&[0.0, 2.0, 0.0]).unwrap();
    assert_eq!(n, [0.0, 1.0, 0.0]);
    assert_eq!(normalize(&[0.0, 0.0]), None);
    assert_eq!(cross(&[1, 0, 0], &[0, 1, 0]), [0, 0, 1]);
    for v in [[1.0, -2.0, 0.5], [0.0, 0.0, 0.0], [3.0, 1.0, -7.0]] {
        assert!(<[f64; 3] as InnerNonNegative<f64, LessEq>>::holds(&v));
    }
}

#[test]
fn conjugate_gradient_matches_gauss() {
    let a = [[4.0, 1.0, 0.0], [1.0, 3.0, -1.0], [0.0, -1.0, 2.0]];
    let b = [1.0, 2.0, 3.0];
    let apply = |v: &[f64; 3]| -> [f64; 3] {
        std::array::from_fn(|i| (0..3).map(|j| a[i][j] * v[j]).sum())
    };
    let x = conjugate_gradient(apply, &b, [0.0; 3], &1e-12, 50).unwrap();
    let rows = a.iter().map(|r| r.to_vec()).collect();
    let g = solve_linear_pivoting(rows, b.to_vec()).unwrap();
    for i in 0..3 {
        assert!((x[i] - g[i]).abs() < 1e-10);
    }
}

#[test]
fn rk4_exponential_and_oscillator() {
    // y' = y, y(0) = 1 ⇒ y(1) = e (hier als 1-dim. Vektor)
    let mut y = [1.0];
    let h = 0.01;
    for k in 0..100 {
        y = rk4_step(|_, y: &[f64; 1]| *y, &(k as f64 * h), &y, &h);
    }
    assert!((y[0] - std::f64::consts::E).abs() < 1e-9);

    // x'' = −x als System (x, v): nach 2π zurück am Start
    let mut s = [1.0, 0.0];
    let n = 1000;
    let h = 2.0 * std::f64::consts::PI / n as f64;
    for k in 0..n {
        s = rk4_step(|_, s: &[f64; 2]| [s[1], -s[0]], &(k as f64 * h), &s, &h);
    }
    assert!((s[0] - 1.0).abs() < 1e-9 && s[1].abs() < 1e-9);
}

fn simulate(x: &mut Vec<[f64; 2]>, v: &mut Vec<[f64; 2]>, m: &[f64], t: f64, steps: usize) {
    let h = t / steps as f64;
    for _ in 0..steps {
        let (x1, v1) = verlet_step(|p| gravity(p, m, &1.0), x, v, &h);
        *x = x1;
        *v = v1;
    }
}

#[test]
fn two_body_circular_orbit() {
    // Zwei gleiche Massen im Abstand 1 auf einer Kreisbahn
    let m = [1.0, 1.0];
    let speed = 0.5f64.sqrt();
    let mut x = vec![[-0.5, 0.0], [0.5, 0.0]];
    let mut v = vec![[0.0, -speed], [0.0, speed]];
    let e0 = energy(&x, &v, &m, &1.0);
    let period = std::f64::consts::PI / speed;
    simulate(&mut x, &mut v, &m, period, 20_000);
    assert!(distance(&x[0], &[-0.5, 0.0]) < 1e-3, "{x:?}");
    assert!((energy(&x, &v, &m, &1.0) - e0).abs() < 1e-9);
}

#[test]
fn three_body_figure_eight() {
    // Chenciner-Montgomery: drei gleiche Massen auf einer Acht.
    let m = [1.0, 1.0, 1.0];
    let p = [0.970_004_36, -0.243_087_53];
    let v3 = [-0.932_407_37, -0.864_731_46];
    let mut x = vec![p, [-p[0], -p[1]], [0.0, 0.0]];
    let mut v = vec![
        [-v3[0] / 2.0, -v3[1] / 2.0],
        [-v3[0] / 2.0, -v3[1] / 2.0],
        v3,
    ];
    let start = x.clone();
    let e0 = energy(&x, &v, &m, &1.0);
    simulate(&mut x, &mut v, &m, 6.325_913_98, 20_000);
    for (a, b) in x.iter().zip(&start) {
        assert!(distance(a, b) < 1e-3, "{x:?}");
    }
    assert!((energy(&x, &v, &m, &1.0) - e0).abs() < 1e-8);
}
