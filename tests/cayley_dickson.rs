//! Cayley-Dickson über `i64`: Alle Gesetze werden **exakt** geprüft.
//! Für die verlorenen Gesetze wird jeweils ein Gegenbeispiel gesucht.

use math_traits::impls::cayley_dickson::*;
use math_traits::laws::*;
use math_traits::signature::*;
use math_traits::structures::*;

/// Baut ein Element aus seinen `DIM` Koeffizienten.
trait Coeffs: Sized {
    const DIM: usize;
    fn from_coeffs(c: &[i64]) -> Self;
}
impl Coeffs for i64 {
    const DIM: usize = 1;
    fn from_coeffs(c: &[i64]) -> Self {
        c[0]
    }
}
impl<T: Coeffs> Coeffs for CayleyDickson<T> {
    const DIM: usize = 2 * T::DIM;
    fn from_coeffs(c: &[i64]) -> Self {
        CayleyDickson::new(T::from_coeffs(&c[..T::DIM]), T::from_coeffs(&c[T::DIM..]))
    }
}

/// Deterministische Stichprobe mit Koeffizienten in `-3..=3`.
fn samples<T: Coeffs>(n: usize) -> Vec<T> {
    let mut state = 0x2545_F491_4F6C_DD1Du64;
    (0..n)
        .map(|_| {
            let c: Vec<i64> = (0..T::DIM)
                .map(|_| {
                    state = state
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    ((state >> 33) % 7) as i64 - 3
                })
                .collect();
            T::from_coeffs(&c)
        })
        .collect()
}

/// Basiselement `eᵢ`.
fn unit<T: Coeffs>(i: usize) -> T {
    let mut c = vec![0; T::DIM];
    c[i] = 1;
    T::from_coeffs(&c)
}

fn mul<T: BinaryOp<Multiplicative>>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}

fn associative<T: BinaryOp<Multiplicative> + PartialEq>(x: &T, y: &T, z: &T) -> bool {
    mul(&mul(x, y), z) == mul(x, &mul(y, z))
}

fn left_alternative<T: BinaryOp<Multiplicative> + PartialEq>(x: &T, y: &T) -> bool {
    mul(&mul(x, x), y) == mul(x, &mul(x, y))
}

/// Gesetze, die auf jeder Stufe gelten.
fn common_laws<T>(xs: &[T])
where
    T: Flexible<Multiplicative>
        + Identity<Multiplicative>
        + Distributive<Multiplicative, Additive>
        + Involution<Multiplicative>
        + AbelianGroup<Additive>
        + PartialEq,
{
    for x in xs {
        assert!(<T as LeftIdentity<Multiplicative>>::holds(x));
        assert!(<T as Involutive<Multiplicative>>::holds(x));
        for y in xs {
            assert!(<T as Flexible<Multiplicative>>::holds(x, y));
            assert!(<T as AntiMultiplicative<Multiplicative>>::holds(x, y));
            assert!(<T as Commutative<Additive>>::holds(x, y));
            for z in xs.iter().take(10) {
                assert!(<T as LeftDistributive<Multiplicative, Additive>>::holds(
                    x, y, z
                ));
                assert!(<T as RightDistributive<Multiplicative, Additive>>::holds(
                    x, y, z
                ));
            }
        }
    }
}

#[test]
fn structures_per_level() {
    fn field<T: Field>() {}
    fn division_ring<T: DivisionRing>() {}
    fn alternative<T: AlternativeRing + InverseExceptZero<Multiplicative, Additive>>() {}
    fn non_associative<T: NonAssociativeRing + Flexible<Multiplicative>>() {}

    field::<Complex<f64>>();
    division_ring::<Quaternion<f64>>();
    alternative::<Octonion<f64>>();
    non_associative::<Sedenion<f64>>();

    fn commutative_ring<T: CommutativeRing>() {}
    fn ring<T: Ring>() {}
    commutative_ring::<Complex<i64>>();
    ring::<Quaternion<i64>>();
}

#[test]
fn complex() {
    let xs = samples::<Complex<i64>>(30);
    common_laws(&xs);
    for x in &xs {
        for y in &xs {
            assert!(<Complex<i64> as Commutative<Multiplicative>>::holds(x, y));
            for z in &xs {
                assert!(<Complex<i64> as Associative<Multiplicative>>::holds(
                    x, y, z
                ));
            }
        }
    }
    // i² = -1
    let i = unit::<Complex<i64>>(1);
    assert_eq!(mul(&i, &i), Complex::new(-1, 0));
}

#[test]
fn quaternions_lose_commutativity() {
    let xs = samples::<Quaternion<i64>>(30);
    common_laws(&xs);
    for x in &xs {
        for y in &xs {
            for z in &xs {
                assert!(<Quaternion<i64> as Associative<Multiplicative>>::holds(
                    x, y, z
                ));
            }
        }
    }
    let (i, j) = (unit::<Quaternion<i64>>(1), unit::<Quaternion<i64>>(2));
    assert_ne!(mul(&i, &j), mul(&j, &i));
}

#[test]
fn octonions_lose_associativity() {
    let xs = samples::<Octonion<i64>>(30);
    common_laws(&xs);
    for x in &xs {
        for y in &xs {
            assert!(<Octonion<i64> as LeftAlternative<Multiplicative>>::holds(
                x, y
            ));
            assert!(<Octonion<i64> as RightAlternative<Multiplicative>>::holds(
                x, y
            ));
        }
    }
    let e = |i| unit::<Octonion<i64>>(i);
    let broken = (1..8).any(|a| (1..8).any(|b| (1..8).any(|c| !associative(&e(a), &e(b), &e(c)))));
    assert!(broken, "Oktonionen müssen nicht-assoziativ sein");
}

#[test]
fn sedenions_lose_alternativity_and_get_zero_divisors() {
    let xs = samples::<Sedenion<i64>>(20);
    common_laws(&xs);

    let broken = xs
        .iter()
        .any(|x| xs.iter().any(|y| !left_alternative(x, y)));
    assert!(broken, "Sedenionen müssen nicht-alternativ sein");

    // Suche Nullteiler der Form (eᵢ ± eⱼ)(eₖ ± eₗ) = 0.
    type S = Sedenion<i64>;
    let zero = <S as HasIdentity<Additive>>::identity();
    let pairs: Vec<S> = (1..16)
        .flat_map(|i| (i + 1..16).flat_map(move |j| [(i, j, 1), (i, j, -1)]))
        .map(|(i, j, s)| {
            let mut c = [0i64; 16];
            c[i] = 1;
            c[j] = s;
            S::from_coeffs(&c)
        })
        .collect();
    let found = pairs
        .iter()
        .any(|a| pairs.iter().any(|b| mul(a, b) == zero));
    assert!(found, "Sedenionen müssen Nullteiler haben");
}

#[test]
fn inverses_over_f64() {
    let q: Quaternion<f64> = Quaternion::new(Complex::new(1.0, 2.0), Complex::new(-0.5, 3.0));
    let inv = q.try_inverse().unwrap();
    let one = mul(&q, &inv);
    assert!((one.re.re - 1.0).abs() < 1e-12);
    assert!(one.re.im.abs() < 1e-12 && one.im.re.abs() < 1e-12 && one.im.im.abs() < 1e-12);

    let zero = <Octonion<f64> as HasIdentity<Additive>>::identity();
    assert_eq!(zero.try_inverse(), None);
}
