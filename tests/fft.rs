//! FFT: exakt über GF(17) und Gaußschen Zahlen, angenähert über ℂ.

use math_traits::fft::*;
use math_traits::impls::cayley_dickson::Complex;
use math_traits::laws;
use math_traits::laws::PrimitiveRootOfUnity;
use math_traits::signature::*;

const P: u32 = 17;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Gf17(u32);

impl BinaryOp<Additive> for Gf17 {
    fn op(&self, r: &Self) -> Self {
        Gf17((self.0 + r.0) % P)
    }
}
impl HasIdentity<Additive> for Gf17 {
    fn identity() -> Self {
        Gf17(0)
    }
}
impl HasInverse<Additive> for Gf17 {
    fn inverse(&self) -> Self {
        Gf17((P - self.0) % P)
    }
}
impl BinaryOp<Multiplicative> for Gf17 {
    fn op(&self, r: &Self) -> Self {
        Gf17((self.0 * r.0) % P)
    }
}
impl HasIdentity<Multiplicative> for Gf17 {
    fn identity() -> Self {
        Gf17(1)
    }
}
impl HasPartialInverse<Multiplicative> for Gf17 {
    fn try_inverse(&self) -> Option<Self> {
        (1..P).map(Gf17).find(|y| self.0 * y.0 % P == 1)
    }
}
impl HasRootsOfUnity for Gf17 {
    /// 3 erzeugt die Einheitengruppe (Ordnung 16), also ist `3^(16/n)` eine
    /// primitive n-te Einheitswurzel für jeden Teiler n von 16.
    fn primitive_root_of_unity(n: usize) -> Option<Self> {
        (n > 0 && 16 % n == 0).then(|| Gf17((0..16 / n).fold(1, |a, _| a * 3 % P)))
    }
}

laws! {
    Gf17 {
        Additive: associative, commutative, identity, inverse;
        Multiplicative: associative, commutative, identity, primitive_root_of_unity;
        [Multiplicative, Additive]: distributive, inverse_except_zero, nontrivial;
    }
}

/// Naive DFT als Referenz: `X_k = Σ x_j ω^(−jk)`.
fn dft(x: &[Gf17]) -> Vec<Gf17> {
    let n = x.len();
    let w = Gf17::primitive_root_of_unity(n).unwrap();
    let pw = |e: usize| (0..(n - e % n) % n).fold(Gf17(1), |a, _| op::<Multiplicative, _>(&a, &w));
    (0..n)
        .map(|k| {
            x.iter().enumerate().fold(Gf17(0), |acc, (j, xj)| {
                op::<Additive, _>(&acc, &op::<Multiplicative, _>(xj, &pw(j * k)))
            })
        })
        .collect()
}

#[test]
fn roots_of_unity_laws() {
    for n in 0..=20 {
        assert!(
            <Gf17 as PrimitiveRootOfUnity<Multiplicative>>::holds(n),
            "n = {n}"
        );
        assert!(<Complex<i64> as PrimitiveRootOfUnity<Multiplicative>>::holds(n));
        assert!(<i32 as PrimitiveRootOfUnity<Multiplicative>>::holds(n));
    }
}

#[test]
fn ntt_matches_dft_and_inverts() {
    for n in [1, 2, 4, 8, 16] {
        let x: Vec<Gf17> = (0..n as u32).map(|i| Gf17((i * 7 + 3) % P)).collect();
        let mut y = x.clone();
        fft(&mut y).unwrap();
        assert_eq!(y, dft(&x));
        ifft(&mut y).unwrap();
        assert_eq!(y, x);
    }
    let mut bad = vec![Gf17(1); 3];
    assert_eq!(fft(&mut bad), None);
}

#[test]
fn cyclic_convolution_exact() {
    let a: Vec<Gf17> = [1, 2, 3, 0, 0, 0, 0, 0].map(Gf17).to_vec();
    let b: Vec<Gf17> = [4, 5, 0, 0, 0, 0, 0, 0].map(Gf17).to_vec();
    // (1 + 2x + 3x²)(4 + 5x) = 4 + 13x + 22x² + 15x³, 22 ≡ 5 (mod 17)
    let (mut ca, mut cb) = (a.clone(), b.clone());
    convolve_cyclic(&mut ca, &mut cb).unwrap();
    assert_eq!(ca, [4, 13, 5, 15, 0, 0, 0, 0].map(Gf17).to_vec());
}

#[test]
fn fft_2d_and_3d_exact() {
    // 2D: Zeilen, dann Spalten = separable DFT
    let (r, c) = (4, 8);
    let x: Vec<Gf17> = (0..(r * c) as u32).map(|i| Gf17(i * i % P)).collect();
    let mut y = x.clone();
    fft_nd(&mut y, &[r, c]).unwrap();
    let mut expected = x.clone();
    for row in expected.chunks_mut(c) {
        row.copy_from_slice(&dft(row));
    }
    for col in 0..c {
        let column: Vec<Gf17> = (0..r).map(|i| expected[i * c + col]).collect();
        for (i, v) in dft(&column).into_iter().enumerate() {
            expected[i * c + col] = v;
        }
    }
    assert_eq!(y, expected);

    // 3D: hin und zurück
    let shape = [2, 4, 2];
    let z: Vec<Gf17> = (0..16u32).map(|i| Gf17((5 * i + 1) % P)).collect();
    let mut w = z.clone();
    fft_nd(&mut w, &shape).unwrap();
    ifft_nd(&mut w, &shape).unwrap();
    assert_eq!(w, z);
}

#[test]
fn fft_over_gaussian_integers_exact() {
    type G = Complex<i64>;
    let x = [G::new(1, 0), G::new(2, -1), G::new(0, 3), G::new(-1, 1)];
    let mut y = x;
    fft(&mut y).unwrap();
    // X₀ = Σ xⱼ
    assert_eq!(y[0], G::new(2, 3));
}

#[test]
fn fft_over_complex_f64() {
    type C = Complex<f64>;
    let n = 64;
    let x: Vec<C> = (0..n)
        .map(|i| C::new((i as f64 * 0.3).sin(), (i as f64).cos()))
        .collect();
    let mut y = x.clone();
    fft(&mut y).unwrap();
    ifft(&mut y).unwrap();
    for (a, b) in x.iter().zip(&y) {
        assert!((a.re - b.re).abs() < 1e-12 && (a.im - b.im).abs() < 1e-12);
    }
    // Ein reiner Ton landet in genau einem Frequenz-Bin.
    let tone: Vec<C> = (0..n)
        .map(|j| {
            let phi = 2.0 * std::f64::consts::PI * 5.0 * j as f64 / n as f64;
            C::new(phi.cos(), phi.sin())
        })
        .collect();
    let mut t = tone;
    fft(&mut t).unwrap();
    for (k, v) in t.iter().enumerate() {
        let mag = (v.re * v.re + v.im * v.im).sqrt();
        if k == 5 {
            assert!((mag - n as f64).abs() < 1e-9);
        } else {
            assert!(mag < 1e-9, "k = {k}: {mag}");
        }
    }
}
