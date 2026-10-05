#![cfg(any(feature = "f16", feature = "f128"))]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]
use math_traits::*;

#[cfg(feature = "f16")]
#[test]
fn f16_octonion() {
    let o = Octonion::<f16>::basis(5);
    assert_eq!(o * o, -Octonion::<f16>::one());
    assert_eq!(Octonion::<f16>::one() / Octonion::<f16>::one(), Octonion::<f16>::one());
}

#[cfg(feature = "f128")]
#[test]
fn f128_sedenion() {
    let (a, b) = Sedenion::<f128>::zero_divisor_pair();
    assert!(Sedenion::<f128>::is_nontrivial_zero_product(&a, &b));
}
