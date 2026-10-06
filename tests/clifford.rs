use math_traits::*;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> i32 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 33) % 7) as i32 - 3
    }
}
fn rand<A: Algebra<Scalar = i32>>(r: &mut Lcg) -> A {
    A::from_fn(|_| r.next())
}

type Cl300<T> = Clifford<T, 3, 0, 0, 8>;

#[test]
fn basis_products_cl3() {
    type A = Cl3<i32>;
    let (e1, e2, e3) = (A::generator(0), A::generator(1), A::generator(2));
    assert_eq!(e1 * e1, A::one());
    assert_eq!(e1 * e2, -(e2 * e1));
    let e12 = e1 * e2;
    assert_eq!(e12, A::blade(0b011, 1));
    assert_eq!(e12 * e12, -A::one());
    assert_eq!(e1 * e2 * e3, A::pseudoscalar());
    assert_eq!(A::pseudoscalar() * A::pseudoscalar(), -A::one());
    assert_eq!(A::GENERATORS, 3);
}

#[test]
fn metric_signs_per_signature() {
    // Cl(1,3): e0^2 = +1, e1..e3^2 = -1
    type A = Sta<i32>;
    assert_eq!(A::generator(0) * A::generator(0), A::one());
    for i in 1..4 {
        assert_eq!(A::generator(i) * A::generator(i), -A::one());
    }
    // Cl(2,0,1): e2^2 = 0
    type P = Pga2<i32>;
    assert_eq!(P::generator(0) * P::generator(0), P::one());
    assert!((P::generator(2) * P::generator(2)).is_zero());
    // Cl(4,1): e4^2 = -1
    type C = Cga3<i32>;
    assert_eq!(C::generator(3) * C::generator(3), C::one());
    assert_eq!(C::generator(4) * C::generator(4), -C::one());
}

/// Misst, ob (kommutativ, assoziativ, alternativ, flexibel, potenzassoziativ) gilt.
fn measure<A: Algebra<Scalar = i32> + core::fmt::Debug>(seed: u64) -> [bool; 5] {
    let mut r = Lcg(seed);
    let mut res = [true; 5];
    for _ in 0..60 {
        let (a, b, c): (A, A, A) = (rand(&mut r), rand(&mut r), rand(&mut r));
        res[0] &= a * b == b * a;
        res[1] &= (a * b) * c == a * (b * c);
        res[2] &= (a * a) * b == a * (a * b) && (b * a) * a == b * (a * a);
        res[3] &= (a * b) * a == a * (b * a);
        res[4] &= (a * a) * (a * a) == a * (a * (a * a));
    }
    res
}

macro_rules! axioms {
    ($($name:ident: $p:literal, $q:literal, $r:literal, $n:literal;)*) => {$(
        #[test]
        fn $name() {
            type A = Clifford<i32, $p, $q, $r, $n>;
            assert_eq!(A::DIM, $n);
            let m = measure::<A>(11);
            assert_eq!(m[0], A::COMMUTATIVE, "kommutativ");
            assert!(A::ASSOCIATIVE && m[1], "assoziativ");
            assert!(A::ALTERNATIVE && m[2], "alternativ");
            assert!(A::FLEXIBLE && m[3], "flexibel");
            assert!(A::POWER_ASSOCIATIVE && m[4], "potenzassoziativ");
            // Einselement und Distributivitaet
            let mut r = Lcg(5);
            for _ in 0..20 {
                let (a, b, c): (A, A, A) = (rand(&mut r), rand(&mut r), rand(&mut r));
                assert_eq!(a * A::one(), a);
                assert_eq!(A::one() * a, a);
                assert_eq!(a * (b + c), a * b + a * c);
                assert_eq!((a + b) * c, a * c + b * c);
                // Involutionen: Reverse und Konjugation drehen Produkte um, Gradinvolution nicht
                assert_eq!((a * b).reverse(), b.reverse() * a.reverse());
                assert_eq!((a * b).clifford_conjugate(), b.clifford_conjugate() * a.clifford_conjugate());
                assert_eq!((a * b).grade_involution(), a.grade_involution() * b.grade_involution());
                assert_eq!(a.reverse().reverse(), a);
                assert_eq!(a.grade_involution().grade_involution(), a);
                assert_eq!(a.clifford_conjugate(), a.reverse().grade_involution());
                // Keilprodukt: assoziativ und antisymmetrisch auf Vektoren
                assert_eq!(a.wedge(b).wedge(c), a.wedge(b.wedge(c)));
            }
        }
    )*};
}
axioms! {
    axioms_cl000: 0, 0, 0, 1;
    axioms_cl100: 1, 0, 0, 2;
    axioms_cl010: 0, 1, 0, 2;
    axioms_cl001: 0, 0, 1, 2;
    axioms_cl200: 2, 0, 0, 4;
    axioms_cl020: 0, 2, 0, 4;
    axioms_cl110: 1, 1, 0, 4;
    axioms_cl030: 0, 3, 0, 8;
    axioms_cl111: 1, 1, 1, 8;
    axioms_cl201: 2, 0, 1, 8;
    axioms_cl004: 0, 0, 4, 16;
    axioms_cl211: 2, 1, 1, 16;
    axioms_cl410: 4, 1, 0, 32;
}

#[test]
fn marker_traits_and_flags() {
    fn assoc<A: Associative + Alternative + Flexible + PowerAssociative>() {}
    fn comm<A: Commutative>() {}
    fn norm<A: MultiplicativeNorm>() {}
    assoc::<Cl3<f64>>();
    assoc::<Cga3<i64>>();
    assoc::<Clifford<i8, 5, 0, 2, 128>>();
    comm::<ClComplex<f32>>();
    comm::<SplitComplex<i32>>();
    comm::<DualNumber<i32>>();
    comm::<Clifford<i32, 0, 0, 0, 1>>();
    norm::<ClQuaternion<f64>>();
    const { assert!(!Cl3::<f64>::COMMUTATIVE) };
    const { assert!(Cl3::<f64>::HAS_ZERO_DIVISORS) };
    const { assert!(!ClComplex::<f64>::HAS_ZERO_DIVISORS) };
    const { assert!(!ClQuaternion::<f64>::HAS_ZERO_DIVISORS) };
    const { assert!(ClQuaternion::<f64>::MULTIPLICATIVE_NORM) };
    const { assert!(!Pga3::<f64>::MULTIPLICATIVE_NORM) };
}

#[test]
fn same_algebra_as_complex_and_quaternion() {
    let mut r = Lcg(3);
    for _ in 0..30 {
        let (a, b): (Complex<i32>, Complex<i32>) = (rand(&mut r), rand(&mut r));
        assert_eq!(
            ClComplex::from(a * b),
            ClComplex::from(a) * ClComplex::from(b)
        );
        assert_eq!(Complex::from(ClComplex::from(a)), a);
        let (a, b): (Quaternion<i32>, Quaternion<i32>) = (rand(&mut r), rand(&mut r));
        assert_eq!(
            ClQuaternion::from(a * b),
            ClQuaternion::from(a) * ClQuaternion::from(b)
        );
        assert_eq!(Quaternion::from(ClQuaternion::from(a)), a);
        // Clifford-Konjugation entspricht der Quaternionen-Konjugation
        assert_eq!(
            ClQuaternion::from(a.conjugate()),
            ClQuaternion::from(a).clifford_conjugate()
        );
    }
}

#[test]
fn zero_divisors() {
    fn check<const P: usize, const Q: usize, const R: usize, const N: usize>() -> bool {
        match Clifford::<i32, P, Q, R, N>::try_zero_divisor_pair() {
            Some((a, b)) => {
                assert!(!a.is_zero() && !b.is_zero(), "({P},{Q},{R})");
                assert!((a * b).is_zero(), "({P},{Q},{R}) a*b != 0");
                true
            }
            None => false,
        }
    }
    // Divisionsalgebren: keine Nullteiler
    assert!(!check::<0, 0, 0, 1>());
    assert!(!check::<0, 1, 0, 2>());
    assert!(!check::<0, 2, 0, 4>());
    // Alle anderen Signaturen bis n = 4 haben welche
    assert!(check::<1, 0, 0, 2>());
    assert!(check::<0, 0, 1, 2>());
    assert!(check::<0, 3, 0, 8>());
    assert!(check::<0, 4, 0, 16>());
    assert!(check::<1, 1, 0, 4>());
    assert!(check::<2, 0, 0, 4>());
    assert!(check::<0, 1, 1, 4>());
    assert!(check::<0, 2, 1, 8>());
    assert!(check::<3, 0, 1, 16>());
    assert!(check::<4, 1, 0, 32>());
    // Trait fuer benannte Algebren mit Nullteilern
    fn nz<A: NonTrivialZero + core::fmt::Debug>() {
        let (a, b) = A::zero_divisor_pair();
        assert!(A::is_nontrivial_zero_product(&a, &b));
    }
    nz::<Cl3<i32>>();
    nz::<Pga3<f64>>();
    nz::<Cga3<i64>>();
    nz::<Sta<f32>>();
    nz::<DualNumber<i32>>();
    nz::<SplitComplex<f64>>();
    nz::<Grassmann3<i32>>();
}

#[test]
fn no_zero_divisors_in_division_algebras_by_search() {
    // Brute-Force ueber kleine Koeffizienten: in Cl(0,2) (Quaternionen) ist a*b = 0 nur fuer a = 0 oder b = 0.
    type A = ClQuaternion<i32>;
    let vals = [-1, 0, 1];
    let all: Vec<A> = (0..81)
        .map(|n| A::from_fn(|i| vals[(n / 3usize.pow(i as u32)) % 3]))
        .collect();
    for &a in &all {
        for &b in &all {
            if !a.is_zero() && !b.is_zero() {
                assert!(!(a * b).is_zero());
            }
        }
    }
}

#[test]
fn wedge_and_contractions() {
    type A = Cl3<i32>;
    let (e1, e2, e3) = (A::generator(0), A::generator(1), A::generator(2));
    assert_eq!(e1.wedge(e2), e1 * e2);
    assert_eq!(e1.wedge(e1), A::zero());
    assert_eq!(e1.wedge(e2), -e2.wedge(e1));
    assert_eq!(e1.wedge(e2).wedge(e3), A::pseudoscalar());
    // e1 _| e12 = e2
    assert_eq!(e1.left_contract(e1 * e2), e2);
    assert_eq!((e1 * e2).right_contract(e2), e1);
    // a*b = a _| b + a ^ b fuer Vektoren
    let mut r = Lcg(9);
    for _ in 0..20 {
        let a = A::from_vector([r.next(), r.next(), r.next()]);
        let b = A::from_vector([r.next(), r.next(), r.next()]);
        assert_eq!(a * b, A::from_scalar(a.scalar_product(b)) + a.wedge(b));
        assert_eq!(a.left_contract(b), A::from_scalar(a.scalar_product(b)));
        assert_eq!(a.grade_part(1), a);
        assert_eq!(a.vector_part::<3>(), [a.coeff(1), a.coeff(2), a.coeff(4)]);
    }
    // Grad-Projektion zerlegt
    let x = A::from_coeffs([1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(
        x.grade_part(0) + x.grade_part(1) + x.grade_part(2) + x.grade_part(3),
        x
    );
    assert_eq!(x.scalar_part(), 1);
    assert_eq!(x.grade_part(3), Cl3::pseudoscalar().scale(8));
}

#[test]
fn inverse_dual_sandwich() {
    type A = Cl300<f64>;
    // Vektor: v^-1 = v / v^2
    let v = A::from_vector([1.0, 2.0, 2.0]);
    let vi = v.inverse().unwrap();
    assert_eq!(v * vi, A::one());
    assert_eq!(vi, v.scale(1.0 / 9.0));
    assert!(A::zero().inverse().is_none());
    // Ganzzahlen: nur Einheiten
    assert!(Cl3::<i32>::generator(1).inverse().is_some());
    assert!(Cl3::<i32>::from_vector([1, 1, 0]).inverse().is_none());
    // Nullvektor in PGA hat kein Inverses
    assert!(Pga3::<f64>::generator(3).inverse().is_none());

    // Dualitaet in Cl(3,0): I^2 = -1, dual(e1) = e1 * (-I) = -e23
    let e1 = A::generator(0);
    let e23 = A::generator(1) * A::generator(2);
    assert_eq!(e1.dual().unwrap(), -e23);
    assert_eq!(e1.dual().unwrap().dual().unwrap(), -e1);
    assert!(Pga3::<f64>::generator(0).dual().is_none());

    // Rotor: 90 Grad in der e1-e2-Ebene, e1 -> -e2? (Konvention: R = cos(t/2) - sin(t/2) e12)
    let h = 0.5f64.sqrt();
    let e12 = A::generator(0) * A::generator(1);
    let rotor = A::from_scalar(h) - e12.scale(h);
    let rotated = rotor.sandwich(A::generator(0));
    let want = A::generator(1);
    assert!((rotated - want).norm_sqr() < 1e-24, "{rotated:?}");
    // Rotor erhaelt die Laenge: R R~ = 1
    assert!((rotor * rotor.reverse() - A::one()).norm_sqr() < 1e-24);
    // e3 bleibt fest
    let r3 = rotor.sandwich(A::generator(2));
    assert!((r3 - A::generator(2)).norm_sqr() < 1e-24);
}

#[test]
fn conformal_and_spacetime_smoke() {
    // CGA: Produkt in Dimension 32 laeuft und ist assoziativ
    type C = Cga3<i32>;
    let mut r = Lcg(21);
    let (a, b, c): (C, C, C) = (rand(&mut r), rand(&mut r), rand(&mut r));
    assert_eq!((a * b) * c, a * (b * c));
    // STA: Pseudoskalar quadriert zu -1 in Cl(1,3)
    type S = Sta<i32>;
    assert_eq!(S::pseudoscalar() * S::pseudoscalar(), -S::one());
    // Skalar-Norm ist indefinit: e0 hat +1, e1 hat -1
    assert_eq!(S::generator(0).norm_sqr(), 1);
    assert_eq!(S::generator(1).norm_sqr(), -1);
    // Grassmann-Algebra: alle Generatoren quadrieren zu 0, Keilprodukt = Produkt
    type G = Grassmann3<i32>;
    let (x, y) = (G::generator(0), G::generator(1));
    assert!((x * x).is_zero());
    assert_eq!(x * y, x.wedge(y));
}

#[test]
fn floats_and_all_signed_scalars() {
    fn run<T: Signed>() {
        let e1 = Cl3::<T>::generator(0);
        assert_eq!(e1 * e1, Cl3::<T>::one());
        let g = Sta::<T>::generator(1);
        assert_eq!(g * g, -Sta::<T>::one());
    }
    run::<i8>();
    run::<i16>();
    run::<i32>();
    run::<i64>();
    run::<i128>();
    run::<isize>();
    run::<f32>();
    run::<f64>();
}
