//! Gleichungslöser über verschiedenen Körpern.

use math_traits::derived::gcd;
use math_traits::impls::cayley_dickson::Complex;
use math_traits::laws::*;
use math_traits::signature::*;
use math_traits::solve::*;

#[test]
fn bisection_sqrt2() {
    let r = bisect(|x: &f64| x * x - 2.0, 0.0, 2.0, &1e-12, 100).unwrap();
    assert!((r - 2f64.sqrt()).abs() < 1e-11);
    // Kein Vorzeichenwechsel
    assert_eq!(bisect(|x: &f64| x * x + 1.0, -1.0, 1.0, &1e-12, 100), None);
}

#[test]
fn gauss_f64_with_pivoting() {
    // Pivot nötig: a₀₀ = 0
    let a = vec![
        vec![0.0, 2.0, 1.0],
        vec![1.0, -2.0, -3.0],
        vec![-1.0, 1.0, 2.0],
    ];
    let b = vec![-8.0, 0.0, 3.0];
    let x = solve_linear_pivoting(a, b).unwrap();
    for (xi, ei) in x.iter().zip([-4.0f64, -5.0, 2.0]) {
        assert!((xi - ei).abs() < 1e-12, "{x:?}");
    }
    // Singulär
    assert_eq!(
        solve_linear_pivoting(vec![vec![1.0, 2.0], vec![2.0, 4.0]], vec![1.0, 2.0]),
        None
    );
}

#[test]
fn gauss_exact_over_gf2() {
    // x ⊕ y = 1, y = 1  ⇒  x = 0
    let a = vec![vec![true, true], vec![false, true]];
    assert_eq!(solve_linear(a, vec![true, true]), Some(vec![false, true]));
}

#[test]
fn gauss_over_complex_numbers() {
    // i·z = 1  ⇒  z = -i
    type C = Complex<f64>;
    let i = C::new(0.0, 1.0);
    let x = solve_linear(vec![vec![i]], vec![C::new(1.0, 0.0)]).unwrap();
    assert_eq!(x, vec![C::new(0.0, -1.0)]);
}

#[test]
fn newton_2d() {
    // x² + y² = 4, x = y  ⇒  x = y = √2
    let f = |v: &[f64]| vec![v[0] * v[0] + v[1] * v[1] - 4.0, v[0] - v[1]];
    let j = |v: &[f64]| vec![vec![2.0 * v[0], 2.0 * v[1]], vec![1.0, -1.0]];
    let x = newton(f, j, vec![1.0, 0.5], &1e-12, 50).unwrap();
    assert!((x[0] - 2f64.sqrt()).abs() < 1e-10 && (x[1] - 2f64.sqrt()).abs() < 1e-10);
}

#[test]
fn euclid() {
    assert_eq!(gcd(&48i64, &18), 6);
    assert_eq!(gcd(&17u32, &5), 1);
    assert_eq!(gcd(&0i32, &7), 7);
    for a in -20i32..=20 {
        for b in -20i32..=20 {
            assert!(<i32 as DivisionWithRemainder<Multiplicative, Additive>>::holds(&a, &b));
            assert!(<i32 as RemainderDecreases<Multiplicative, Additive>>::holds(&a, &b));
        }
    }
    assert!(<i32 as DivisionWithRemainder<Multiplicative, Additive>>::holds(&i32::MIN, &-1));
}

#[test]
fn vectors_form_a_module() {
    let vs: Vec<[i64; 3]> = vec![[1, 2, 3], [-4, 0, 5], [7, -1, 0]];
    for s in -3i64..=3 {
        for t in -3i64..=3 {
            for v in &vs {
                assert!(<[i64; 3] as ScalarCompatible<i64>>::holds(&s, &t, v));
                assert!(<[i64; 3] as ScalarDistributesOverScalars<i64>>::holds(
                    &s, &t, v
                ));
                for w in &vs {
                    assert!(<[i64; 3] as ScalarDistributesOverVectors<i64>>::holds(
                        &s, v, w
                    ));
                }
            }
        }
    }
    assert!(<[i64; 3] as ScalarIdentity<i64>>::holds(&vs[0]));
}
