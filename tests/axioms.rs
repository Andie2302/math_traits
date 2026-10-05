use math_traits::*;

// Deterministische Pseudozufallswerte (kleine Ganzzahlen -> exakte Arithmetik).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> i64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) % 7) as i64 - 3
    }
}
fn rand<A: CayleyDickson>(r: &mut Lcg) -> A
where
    A::Scalar: From<i8>,
{
    A::from_fn(|_| A::Scalar::from(r.next() as i8))
}

/// Liefert (kommutativ, assoziativ, alternativ, flexibel, N(ab)=N(a)N(b)) als gemessene Werte.
fn measure<A: CayleyDickson + core::fmt::Debug>(seed: u64) -> [bool; 6]
where
    A::Scalar: From<i8> + core::fmt::Debug,
{
    let mut r = Lcg(seed);
    let mut res = [true; 6];
    for _ in 0..200 {
        let (a, b, c): (A, A, A) = (rand(&mut r), rand(&mut r), rand(&mut r));
        res[0] &= a * b == b * a;
        res[1] &= (a * b) * c == a * (b * c);
        res[2] &= (a * a) * b == a * (a * b) && (b * a) * a == b * (a * a);
        res[3] &= (a * b) * a == a * (b * a);
        res[4] &= (a * b).norm_sqr() == a.norm_sqr() * b.norm_sqr();
        res[5] &= (a * a) * (a * a) == a * (a * (a * a));
    }
    res
}

macro_rules! check_level {
    ($name:ident, $ty:ty, $lvl:expr, dim $dim:expr) => {
        #[test]
        fn $name() {
            type A = $ty;
            assert_eq!(A::LEVEL, $lvl);
            assert_eq!(A::DIM, $dim);
            let m = measure::<A>(42);
            // Das gemessene Verhalten darf der Deklaration nicht widersprechen
            // (Deklaration "true" => gemessen "true"); ab der ersten falschen Stufe
            // muss die Eigenschaft auch tatsaechlich fehlschlagen.
            assert_eq!(m[0], A::COMMUTATIVE, "kommutativ");
            assert_eq!(m[1], A::ASSOCIATIVE, "assoziativ");
            assert_eq!(m[2], A::ALTERNATIVE, "alternativ");
            assert_eq!(m[3], A::FLEXIBLE, "flexibel");
            assert_eq!(m[4], A::MULTIPLICATIVE_NORM, "multiplikative Norm");
            assert_eq!(m[5], A::POWER_ASSOCIATIVE, "potenzassoziativ");
        }
    };
}
check_level!(real, Real<i32>, 0, dim 1);
check_level!(complex, Complex<i32>, 1, dim 2);
check_level!(quaternion, Quaternion<i32>, 2, dim 4);
check_level!(octonion, Octonion<i32>, 3, dim 8);
check_level!(sedenion, Sedenion<i32>, 4, dim 16);
check_level!(trigintaduonion, Trigintaduonion<i32>, 5, dim 32);

#[test]
fn marker_traits_exist() {
    fn comm<A: Commutative>() {}
    fn assoc<A: Associative>() {}
    fn alt<A: Alternative>() {}
    fn flex<A: Flexible + PowerAssociative>() {}
    fn norm<A: MultiplicativeNorm>() {}
    fn div<A: DivisionAlgebra>() {}
    fn nz<A: NonTrivialZero>() {}
    comm::<Real<u8>>();
    comm::<Complex<i8>>();
    assoc::<Quaternion<i64>>();
    alt::<Octonion<f32>>();
    norm::<Octonion<i128>>();
    flex::<Trigintaduonion<f64>>();
    div::<Real<f64>>();
    div::<Complex<f32>>();
    div::<Quaternion<f64>>();
    div::<Octonion<f64>>();
    nz::<Sedenion<i32>>();
    nz::<Trigintaduonion<f64>>();
}

#[allow(clippy::eq_op)]
fn basics<A: CayleyDickson + core::ops::Neg<Output = A> + core::fmt::Debug>()
where
    A::Scalar: From<i8> + core::fmt::Debug,
{
    let mut r = Lcg(7);
    let one = A::one();
    for _ in 0..50 {
        let a: A = rand(&mut r);
        assert_eq!(a * one, a);
        assert_eq!(one * a, a);
        assert_eq!(a + A::zero(), a);
        assert_eq!(a - a, A::zero());
        assert_eq!(a + (-a), A::zero());
        assert_eq!(a.conjugate().conjugate(), a);
        // a * conj(a) = N(a) (reell)
        assert_eq!(a * a.conjugate(), A::from_scalar(a.norm_sqr()));
        assert_eq!(a.conjugate() * a, A::from_scalar(a.norm_sqr()));
        for i in 0..A::DIM {
            assert_eq!(A::from_fn(|j| a.coeff(j)).coeff(i), a.coeff(i));
        }
    }
    assert!(A::zero().is_zero());
    assert_eq!(A::basis(0), one);
}

#[test]
fn basics_all_levels() {
    basics::<Real<i32>>();
    basics::<Complex<i32>>();
    basics::<Quaternion<i64>>();
    basics::<Octonion<i16>>();
    basics::<Sedenion<i32>>();
    basics::<Trigintaduonion<i32>>();
    basics::<Complex<f64>>();
    basics::<Sedenion<f32>>();
}

#[test]
fn imaginary_units_square_to_minus_one() {
    fn check<A: CayleyDickson + core::ops::Neg<Output = A> + core::fmt::Debug>() {
        for i in 1..A::DIM {
            assert_eq!(A::basis(i) * A::basis(i), -A::one(), "e{i}^2");
        }
    }
    check::<Complex<i32>>();
    check::<Quaternion<i32>>();
    check::<Octonion<i32>>();
    check::<Sedenion<i32>>();
    check::<Trigintaduonion<i32>>();
}

#[test]
fn quaternion_ijk() {
    type Q = Quaternion<i32>;
    let (i, j, k) = (Q::basis(1), Q::basis(2), Q::basis(3));
    assert_eq!(i * j, k);
    assert_eq!(j * i, -k);
    assert_eq!(i * j * k, -Q::one());
}

#[test]
fn zero_divisors() {
    fn check<A: NonTrivialZero + core::fmt::Debug>() {
        let (a, b) = A::zero_divisor_pair();
        assert!(!a.is_zero() && !b.is_zero());
        assert!((a * b).is_zero());
        assert!(A::is_nontrivial_zero_product(&a, &b));
    }
    check::<Sedenion<i32>>();
    check::<Sedenion<f64>>();
    check::<Trigintaduonion<i64>>();
    check::<Trigintaduonion<f32>>();

    // Unterhalb von Sedenion: kein Nullteiler unter Basis-Summen/-Differenzen.
    type O = Octonion<i32>;
    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                for l in 0..8 {
                    for s in [1, -1] {
                        let a = O::basis(i) + O::basis(j);
                        let b = O::basis(k) + O::basis(l).scale(s);
                        if !a.is_zero() && !b.is_zero() {
                            assert!(!(a * b).is_zero());
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn division_floats() {
    let a = Complex::<f64>::from_re_im(3.0, 4.0);
    let b = Complex::<f64>::from_re_im(1.0, -2.0);
    let q = a / b;
    assert_eq!(q * b, a);
    assert_eq!(a.inverse().unwrap() * a, Complex::one());

    let a = Quaternion::<f64>::from_wxyz(1.0, 2.0, -1.0, 0.5);
    let b = Quaternion::<f64>::from_wxyz(0.5, -1.0, 2.0, 1.0);
    let close = |x: Quaternion<f64>, y: Quaternion<f64>| (x - y).norm_sqr() < 1e-24;
    assert!(close((a / b) * b, a));
    assert!(close(a * a.left_div(b), b));

    let o = Octonion::<f64>::from([1.0, 2.0, 0.0, -1.0, 0.5, 0.0, 3.0, -2.0]);
    let p = Octonion::<f64>::from([2.0, -1.0, 1.0, 0.0, 0.0, 1.5, -1.0, 1.0]);
    assert!(((o / p) * p - o).norm_sqr() < 1e-20);

    assert_eq!(Real(7) / Real(2), Real(3));
    assert_eq!(Real(7.0) / Real(2.0), Real(3.5));
}

#[test]
fn inverse_semantics() {
    assert!(Complex::<f64>::zero().inverse().is_none());
    // Ganzzahlen: nur Einheiten (N == 1) sind invertierbar.
    assert!(Quaternion::<i32>::basis(2).inverse().is_some());
    assert!(Quaternion::<i32>::from_wxyz(1, 1, 0, 0).inverse().is_none());
    // Sedenion-Nullteiler: Inverses im Sinne a*a^-1 = 1 existiert (N > 0), ist aber kein Divisionsinverses.
    let (a, _) = Sedenion::<f64>::zero_divisor_pair();
    assert!(a.inverse().is_some());
}

#[test]
fn unsigned_real_and_all_scalar_types() {
    fn real<T: Scalar>() {
        let one = Real::<T>::one();
        assert_eq!(one * one, one);
        assert!((Real::<T>::zero()).is_zero());
        assert_eq!(Real::<T>::LEVEL, 0);
    }
    real::<u8>(); real::<u16>(); real::<u32>(); real::<u64>(); real::<u128>(); real::<usize>();
    real::<i8>(); real::<i16>(); real::<i32>(); real::<i64>(); real::<i128>(); real::<isize>();
    real::<f32>(); real::<f64>();
    assert_eq!(Real(7u8) / Real(2u8), Real(3u8));

    fn signed<T: Signed>() {
        let q = Sedenion::<T>::one();
        assert_eq!(q * q, q);
        let z = Trigintaduonion::<T>::basis(31);
        assert_eq!(z * z, -Trigintaduonion::<T>::one());
    }
    signed::<i8>(); signed::<i16>(); signed::<i32>(); signed::<i64>(); signed::<i128>(); signed::<isize>();
    signed::<f32>(); signed::<f64>();
}
