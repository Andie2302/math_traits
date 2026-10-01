//! Prüft die deklarierten Gesetze der Beispiel-Impls an konkreten Werten.

use math_traits::laws::*;
use math_traits::signature::*;
use math_traits::structures::*;

/// Jede Struktur gilt allein aufgrund der deklarierten Gesetze.
#[test]
fn structures_follow_from_laws() {
    fn integral_domain<T: IntegralDomain>() {}
    fn boolean_ring<T: BooleanRing>() {}
    fn commutative_ring<T: CommutativeRing>() {}
    fn lattice<T: Lattice>() {}
    fn total_order<T: TotalOrder<LessEq>>() {}

    integral_domain::<bool>();
    boolean_ring::<bool>();
    lattice::<bool>();
    total_order::<bool>();

    commutative_ring::<i32>();
    commutative_ring::<u64>();
    lattice::<i8>();
    total_order::<u128>();
}

const BOOLS: [bool; 2] = [false, true];

#[test]
fn bool_laws_exhaustive() {
    for x in BOOLS {
        assert!(<bool as LeftIdentity<Additive>>::holds(&x));
        assert!(<bool as RightIdentity<Additive>>::holds(&x));
        assert!(<bool as LeftInverse<Additive>>::holds(&x));
        assert!(<bool as RightInverse<Additive>>::holds(&x));
        assert!(<bool as LeftIdentity<Multiplicative>>::holds(&x));
        assert!(<bool as RightIdentity<Multiplicative>>::holds(&x));
        assert!(<bool as Idempotent<Multiplicative>>::holds(&x));
        assert!(<bool as Idempotent<Meet>>::holds(&x));
        assert!(<bool as Idempotent<Join>>::holds(&x));
        assert!(<bool as Reflexive<LessEq>>::holds(&x));
        for y in BOOLS {
            assert!(<bool as Commutative<Additive>>::holds(&x, &y));
            assert!(<bool as Commutative<Multiplicative>>::holds(&x, &y));
            assert!(<bool as ZeroDivisorFree<Multiplicative, Additive>>::holds(&x, &y));
            assert!(<bool as Absorption<Meet, Join>>::holds(&x, &y));
            assert!(<bool as Absorption<Join, Meet>>::holds(&x, &y));
            assert!(<bool as Antisymmetric<LessEq>>::holds(&x, &y));
            assert!(<bool as Total<LessEq>>::holds(&x, &y));
            for z in BOOLS {
                assert!(<bool as Associative<Additive>>::holds(&x, &y, &z));
                assert!(<bool as Associative<Multiplicative>>::holds(&x, &y, &z));
                assert!(<bool as LeftDistributive<Multiplicative, Additive>>::holds(&x, &y, &z));
                assert!(<bool as RightDistributive<Multiplicative, Additive>>::holds(&x, &y, &z));
                assert!(<bool as LeftCancellative<Additive>>::holds(&x, &y, &z));
                assert!(<bool as RightCancellative<Additive>>::holds(&x, &y, &z));
                assert!(<bool as Transitive<LessEq>>::holds(&x, &y, &z));
            }
        }
    }
}

/// `u8` vollständig für ein- und zweistellige Gesetze, Stichproben für dreistellige.
#[test]
fn u8_laws() {
    let sample = || (0..=255u8).step_by(15);
    for x in 0..=255u8 {
        assert!(<u8 as LeftIdentity<Additive>>::holds(&x));
        assert!(<u8 as RightInverse<Additive>>::holds(&x));
        assert!(<u8 as RightIdentity<Multiplicative>>::holds(&x));
        assert!(<u8 as Idempotent<Meet>>::holds(&x));
        for y in 0..=255u8 {
            assert!(<u8 as Commutative<Multiplicative>>::holds(&x, &y));
            assert!(<u8 as Absorption<Join, Meet>>::holds(&x, &y));
            assert!(<u8 as Total<LessEq>>::holds(&x, &y));
        }
    }
    for x in sample() {
        for y in sample() {
            for z in sample() {
                assert!(<u8 as Associative<Multiplicative>>::holds(&x, &y, &z));
                assert!(<u8 as LeftDistributive<Multiplicative, Additive>>::holds(&x, &y, &z));
                assert!(<u8 as LeftCancellative<Additive>>::holds(&x, &y, &z));
            }
        }
    }
}

/// Die Prüffunktionen finden auch Gegenbeispiele.
#[test]
fn checks_detect_violations() {
    // u8 hat Nullteiler: 16 · 16 = 256 ≡ 0
    let ok = (0..=255u8)
        .all(|x| (0..=255u8).all(|y| zero_divisor_free::<u8>(&x, &y)));
    assert!(!ok);

    // f64: (0.1 + 0.2) + 0.3 ≠ 0.1 + (0.2 + 0.3)
    assert_ne!((0.1f64 + 0.2) + 0.3, 0.1 + (0.2 + 0.3));
}

/// Prüft `ZeroDivisorFree` an Typen, die das Gesetz *nicht* deklariert haben.
fn zero_divisor_free<T>(x: &T, y: &T) -> bool
where
    T: BinaryOp<Multiplicative> + HasIdentity<Additive> + PartialEq,
{
    let zero = <T as HasIdentity<Additive>>::identity();
    op::<Multiplicative, _>(x, y) != zero || *x == zero || *y == zero
}
