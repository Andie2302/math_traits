//! Schnelle Fourier-Transformation (radix 2) in beliebig vielen Dimensionen.
//!
//! Generisch über jeden kommutativen Ring mit Einheitswurzeln:
//! * über ℂ (`Complex<f64>`) die klassische FFT,
//! * über endlichen Körpern `GF(p)` die zahlentheoretische Transformation (NTT),
//!   und die ist **exakt**.
//!
//! Alles läuft in-place auf Slices, auch mehrdimensional über Schrittweiten.
//! Es wird kein Heap gebraucht, deshalb läuft es auch in `no_std`.
//!
//! Die Hin-Transformation braucht nur einen [`CommutativeRing`]. Die
//! Rücktransformation teilt durch `n` und braucht deshalb einen [`Field`].

use crate::derived::{pow, try_div};
use crate::signature::{Additive, HasIdentity, HasRootsOfUnity, Multiplicative, op};
use crate::structures::{CommutativeRing, Field};

fn mul<T: CommutativeRing>(a: &T, b: &T) -> T {
    op::<Multiplicative, _>(a, b)
}

/// `n` als Element des Rings (`1 + 1 + … + 1`, per Verdoppeln).
fn from_usize<T: CommutativeRing>(mut n: usize) -> T {
    let mut acc = <T as HasIdentity<Additive>>::identity();
    let mut base = <T as HasIdentity<Multiplicative>>::identity();
    while n > 0 {
        if n & 1 == 1 {
            acc = op::<Additive, _>(&acc, &base);
        }
        base = op::<Additive, _>(&base, &base);
        n >>= 1;
    }
    acc
}

/// Radix-2-FFT auf den Elementen `data[offset + k·stride]` für `k < n`.
/// `w` ist eine primitive `n`-te Einheitswurzel.
fn fft_strided<T>(data: &mut [T], offset: usize, stride: usize, n: usize, w: &T)
where
    T: CommutativeRing + Clone,
{
    let at = |k: usize| offset + k * stride;

    // Bit-Umkehr-Permutation
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            data.swap(at(i), at(j));
        }
    }

    // Schmetterlinge
    let mut len = 2;
    while len <= n {
        let w_len = pow::<Multiplicative, _>(w, (n / len) as u64);
        for start in (0..n).step_by(len) {
            let mut twiddle = <T as HasIdentity<Multiplicative>>::identity();
            for k in 0..len / 2 {
                let (a, b) = (at(start + k), at(start + k + len / 2));
                let u = data[a].clone();
                let v = mul(&data[b], &twiddle);
                data[a] = op::<Additive, _>(&u, &v);
                data[b] = crate::derived::sub(&u, &v);
                twiddle = mul(&twiddle, &w_len);
            }
        }
        len <<= 1;
    }
}

/// Transformiert entlang jeder Achse eines zeilenweise gespeicherten
/// Arrays der Form `shape`. Vorwärts: `X_k = Σ x_j·ω^(−jk)`, rückwärts mit
/// `ω^(+jk)` (ohne Skalierung). Über ℂ ist das die übliche Konvention.
fn transform<T>(data: &mut [T], shape: &[usize], inverse: bool) -> Option<()>
where
    T: CommutativeRing + HasRootsOfUnity + Clone,
{
    if shape.iter().product::<usize>() != data.len() {
        return None;
    }
    for (axis, &n) in shape.iter().enumerate() {
        if !n.is_power_of_two() {
            return None;
        }
        if n == 1 {
            continue;
        }
        let w = T::primitive_root_of_unity(n)?;
        // Übliche Konvention: vorwärts mit ω⁻¹ = ωⁿ⁻¹, rückwärts mit ω.
        let w = if inverse {
            w
        } else {
            pow::<Multiplicative, _>(&w, (n - 1) as u64)
        };
        let stride: usize = shape[axis + 1..].iter().product();
        let outer: usize = shape[..axis].iter().product();
        for o in 0..outer {
            for i in 0..stride {
                fft_strided(data, o * n * stride + i, stride, n, &w);
            }
        }
    }
    Some(())
}

/// FFT in-place. `None`, wenn die Länge keine Zweierpotenz ist oder es keine
/// passende Einheitswurzel gibt.
pub fn fft<T: CommutativeRing + HasRootsOfUnity + Clone>(data: &mut [T]) -> Option<()> {
    transform(data, &[data.len()], false)
}

/// Inverse FFT in-place, inklusive Division durch `n`.
pub fn ifft<T: Field + HasRootsOfUnity + Clone>(data: &mut [T]) -> Option<()> {
    ifft_nd(data, &[data.len()])
}

/// FFT über alle Achsen eines zeilenweise gespeicherten Arrays der Form
/// `shape`, z. B. `&[rows, cols]` oder `&[nx, ny, nz]`.
pub fn fft_nd<T: CommutativeRing + HasRootsOfUnity + Clone>(
    data: &mut [T],
    shape: &[usize],
) -> Option<()> {
    transform(data, shape, false)
}

/// Inverse von [`fft_nd`].
pub fn ifft_nd<T: Field + HasRootsOfUnity + Clone>(data: &mut [T], shape: &[usize]) -> Option<()> {
    transform(data, shape, true)?;
    let inv = try_div(
        &<T as HasIdentity<Multiplicative>>::identity(),
        &from_usize::<T>(data.len()),
    )?;
    for x in data.iter_mut() {
        *x = mul(x, &inv);
    }
    Some(())
}

/// Zyklische Faltung `a ⊛ b` über die FFT, Ergebnis in `a`. `b` wird dabei
/// überschrieben (mit seiner Transformierten). Grundlage für Filter.
pub fn convolve_cyclic<T: Field + HasRootsOfUnity + Clone>(a: &mut [T], b: &mut [T]) -> Option<()> {
    if a.len() != b.len() {
        return None;
    }
    fft(a)?;
    fft(b)?;
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x = mul(x, y);
    }
    ifft(a)
}
