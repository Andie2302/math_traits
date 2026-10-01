//! Ein selbst definierter Körper `GF(7)` und die abgeleiteten Operationen.

use math_traits::derived::*;
use math_traits::laws;
use math_traits::laws::*;
use math_traits::signature::*;
use math_traits::structures::*;

const P: u8 = 7;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Gf7(u8);

impl BinaryOp<Additive> for Gf7 {
    fn op(&self, rhs: &Self) -> Self {
        Gf7((self.0 + rhs.0) % P)
    }
}
impl HasIdentity<Additive> for Gf7 {
    fn identity() -> Self {
        Gf7(0)
    }
}
impl HasInverse<Additive> for Gf7 {
    fn inverse(&self) -> Self {
        Gf7((P - self.0) % P)
    }
}
impl BinaryOp<Multiplicative> for Gf7 {
    fn op(&self, rhs: &Self) -> Self {
        Gf7((self.0 * rhs.0) % P)
    }
}
impl HasIdentity<Multiplicative> for Gf7 {
    fn identity() -> Self {
        Gf7(1)
    }
}
impl HasPartialInverse<Multiplicative> for Gf7 {
    /// Nach Fermat ist `x⁻¹ = x⁵`. Die Signatur darf hier aber noch keine
    /// Gesetze voraussetzen, also bewusst naiv per Suche.
    fn try_inverse(&self) -> Option<Self> {
        (1..P).map(Gf7).find(|y| (self.0 * y.0) % P == 1)
    }
}

laws! {
    Gf7 {
        Additive: associative, commutative, identity, inverse;
        Multiplicative: associative, commutative, identity;
        [Multiplicative, Additive]: distributive, inverse_except_zero, nontrivial;
    }
}

fn all() -> impl Iterator<Item = Gf7> + Clone {
    (0..P).map(Gf7)
}

#[test]
fn is_field() {
    fn field<T: Field>() {}
    field::<Gf7>();
    field::<bool>();
}

#[test]
fn field_laws_exhaustive() {
    assert!(<Gf7 as NonTrivial<Multiplicative, Additive>>::holds());
    for x in all() {
        assert!(<Gf7 as LeftInverseExceptZero<Multiplicative, Additive>>::holds(&x));
        assert!(<Gf7 as RightInverseExceptZero<Multiplicative, Additive>>::holds(&x));
        assert!(<Gf7 as RightInverse<Additive>>::holds(&x));
        for y in all() {
            assert!(<Gf7 as Commutative<Multiplicative>>::holds(&x, &y));
            for z in all() {
                assert!(<Gf7 as Associative<Multiplicative>>::holds(&x, &y, &z));
                assert!(<Gf7 as LeftDistributive<Multiplicative, Additive>>::holds(
                    &x, &y, &z
                ));
            }
        }
    }
    for x in BOOLS {
        assert!(<bool as LeftInverseExceptZero<Multiplicative, Additive>>::holds(&x));
    }
    assert!(<bool as NonTrivial<Multiplicative, Additive>>::holds());
}

const BOOLS: [bool; 2] = [false, true];

#[test]
fn pow_matches_fermat() {
    // Kleiner Satz von Fermat: x⁶ = 1 für x ≠ 0.
    for x in all().skip(1) {
        assert_eq!(pow::<Multiplicative, _>(&x, 6), Gf7(1));
        assert_eq!(pow::<Multiplicative, _>(&x, 0), Gf7(1));
    }
    assert_eq!(pow::<Multiplicative, _>(&Gf7(3), 2), Gf7(2));
    assert_eq!(pow::<Additive, _>(&Gf7(3), 5), Gf7(1)); // 3·5 = 15 ≡ 1
    assert_eq!(pow::<Multiplicative, _>(&3u32, 13), 1_594_323);
}

#[test]
fn pow_signed_uses_inverse() {
    assert_eq!(pow_signed::<Additive, _>(&Gf7(3), -1), Gf7(4));
    assert_eq!(pow_signed::<Additive, _>(&5i32, -3), -15);
}

#[test]
fn fold_sum_product() {
    assert_eq!(sum(all()), Gf7(0)); // 0+1+…+6 = 21 ≡ 0
    assert_eq!(product(all().skip(1)), Gf7(6)); // Satz von Wilson: (p-1)! ≡ -1
    assert_eq!(sum(Vec::<i64>::new()), 0);
    assert_eq!(product([2u8, 3, 4]), 24);
}

#[test]
fn division() {
    assert_eq!(try_div(&Gf7(3), &Gf7(0)), None);
    for a in all() {
        for b in all().skip(1) {
            let q = try_div(&a, &b).unwrap();
            assert_eq!(op::<Multiplicative, _>(&q, &b), a);
        }
    }
    assert_eq!(sub(&Gf7(2), &Gf7(5)), Gf7(4));
}
