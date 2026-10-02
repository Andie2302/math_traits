//! 2D inkompressible Navier-Stokes-Gleichungen, pseudospektral.
//!
//! Formulierung über die Wirbelstärke `ω = ∂v/∂x − ∂u/∂y` auf dem
//! periodischen Gebiet `[0, 2π)²`:
//!
//! ```text
//! ∂ω/∂t + u·∂ω/∂x + v·∂ω/∂y = ν ∇²ω,     ∇²ψ = −ω,     u = ∂ψ/∂y,  v = −∂ψ/∂x
//! ```
//!
//! Die Strömung `(u, v)` kommt aus der Stromfunktion `ψ` und ist damit
//! **exakt divergenzfrei**, die Inkompressibilität ist eingebaut.
//!
//! * Ableitungen im Fourier-Raum: `∂/∂x ↦ i·kₓ`, `∇² ↦ −k²`. Die
//!   Poisson-Gleichung wird zur Division durch `k²`. Dafür wird
//!   [`fft_nd`](math_traits::fft::fft_nd) aus `math_traits` verwendet.
//! * Der nichtlineare Term `u·∇ω` wird im Ortsraum gebildet, mit der
//!   2/3-Regel gegen Aliasing.
//! * Zeitintegration: [`rk4_step`](math_traits::geometry::rk4_step). Der
//!   Zustand ist ein [`Tensor2`], also ein Modul über `f64`, und passt ohne
//!   Anpassung.
//!
//! Gitter: `ω[i][j]` liegt bei `x = 2π·j/N`, `y = 2π·i/N`.

use core::f64::consts::{PI, TAU};

use math_traits::fft::{fft_nd, ifft_nd};
use math_traits::geometry::rk4_step;
use math_traits::impls::cayley_dickson::Complex;
use math_traits::tensor::{Tensor2, TensorShape};

type C = Complex<f64>;

/// Wirbelstärke auf einem `N × N`-Gitter.
pub type Vorticity<const N: usize> = Tensor2<f64, N, N>;

/// Wellenzahl zum Index `i` (`0, 1, …, N/2−1, −N/2, …, −1`).
fn wavenumber(i: usize, n: usize) -> f64 {
    if i < n / 2 {
        i as f64
    } else {
        i as f64 - n as f64
    }
}

/// Spektrum `ω̂` (Länge `N²`, zeilenweise).
fn spectrum<const N: usize>(w: &Vorticity<N>) -> Vec<C> {
    let mut buf: Vec<C> = w.as_slice().iter().map(|&x| C::new(x, 0.0)).collect();
    fft_nd(&mut buf, &[N, N]).expect("N muss eine Zweierpotenz sein");
    buf
}

/// Zurück in den Ortsraum, Realteil.
fn physical<const N: usize>(mut buf: Vec<C>) -> Vec<f64> {
    ifft_nd(&mut buf, &[N, N]).expect("N muss eine Zweierpotenz sein");
    buf.into_iter().map(|c| c.re).collect()
}

/// `i·k·z`
fn times_ik(z: C, k: f64) -> C {
    C::new(-k * z.im, k * z.re)
}

/// Geschwindigkeitsfeld `(u, v)` zur Wirbelstärke.
pub fn velocity<const N: usize>(w: &Vorticity<N>) -> (Vec<f64>, Vec<f64>) {
    let hat = spectrum(w);
    let (mut u, mut v) = (hat.clone(), hat);
    for i in 0..N {
        for j in 0..N {
            let (kx, ky) = (wavenumber(j, N), wavenumber(i, N));
            let k2 = kx * kx + ky * ky;
            let idx = i * N + j;
            let psi = if k2 == 0.0 {
                C::new(0.0, 0.0)
            } else {
                C::new(u[idx].re / k2, u[idx].im / k2)
            };
            u[idx] = times_ik(psi, ky);
            v[idx] = times_ik(C::new(-psi.re, -psi.im), kx);
        }
    }
    (physical::<N>(u), physical::<N>(v))
}

/// Rechte Seite `∂ω/∂t = −u·∇ω + ν∇²ω`.
pub fn rhs<const N: usize>(w: &Vorticity<N>, nu: f64) -> Vorticity<N> {
    let hat = spectrum(w);
    let cutoff = N as f64 / 3.0;
    let zero = C::new(0.0, 0.0);
    let (mut u, mut v, mut wx, mut wy, mut lap) = (
        hat.clone(),
        hat.clone(),
        hat.clone(),
        hat.clone(),
        hat.clone(),
    );
    for i in 0..N {
        for j in 0..N {
            let (kx, ky) = (wavenumber(j, N), wavenumber(i, N));
            let k2 = kx * kx + ky * ky;
            let idx = i * N + j;
            let z = hat[idx];
            lap[idx] = C::new(-k2 * z.re, -k2 * z.im);
            // 2/3-Regel: hohe Moden aus den Faktoren des Produkts entfernen
            if kx.abs() >= cutoff || ky.abs() >= cutoff {
                (u[idx], v[idx], wx[idx], wy[idx]) = (zero, zero, zero, zero);
                continue;
            }
            let psi = if k2 == 0.0 {
                zero
            } else {
                C::new(z.re / k2, z.im / k2)
            };
            u[idx] = times_ik(psi, ky);
            v[idx] = times_ik(C::new(-psi.re, -psi.im), kx);
            wx[idx] = times_ik(z, kx);
            wy[idx] = times_ik(z, ky);
        }
    }
    let (u, v, wx, wy, lap) = (
        physical::<N>(u),
        physical::<N>(v),
        physical::<N>(wx),
        physical::<N>(wy),
        physical::<N>(lap),
    );
    Tensor2::from_flat_fn(|k| -(u[k] * wx[k] + v[k] * wy[k]) + nu * lap[k])
}

/// Ein RK4-Zeitschritt.
pub fn step<const N: usize>(w: &Vorticity<N>, dt: f64, nu: f64) -> Vorticity<N> {
    rk4_step(|_, w: &Vorticity<N>| rhs(w, nu), &0.0, w, &dt)
}

/// Kinetische Energie pro Fläche, `½⟨u² + v²⟩`.
pub fn energy<const N: usize>(w: &Vorticity<N>) -> f64 {
    let (u, v) = velocity(w);
    u.iter().zip(&v).map(|(a, b)| a * a + b * b).sum::<f64>() / (2.0 * (N * N) as f64)
}

/// Enstrophie, `½⟨ω²⟩`.
pub fn enstrophy<const N: usize>(w: &Vorticity<N>) -> f64 {
    w.as_slice().iter().map(|x| x * x).sum::<f64>() / (2.0 * (N * N) as f64)
}

/// Mittelwert der Wirbelstärke (bleibt erhalten).
pub fn mean<const N: usize>(w: &Vorticity<N>) -> f64 {
    w.as_slice().iter().sum::<f64>() / (N * N) as f64
}

/// Erzeugt ein Feld aus einer Funktion `f(x, y)`.
pub fn from_fn<const N: usize>(f: impl Fn(f64, f64) -> f64) -> Vorticity<N> {
    Tensor2::from_flat_fn(|k| {
        let (i, j) = (k / N, k % N);
        f(TAU * j as f64 / N as f64, TAU * i as f64 / N as f64)
    })
}

/// Taylor-Green-Wirbel: `ω = 2k·cos(kx)·cos(ky)`. Exakte Lösung: Die Form
/// bleibt, die Amplitude fällt mit `e^(−2νk²t)`, weil der nichtlineare Term
/// hier genau verschwindet.
pub fn taylor_green<const N: usize>(k: f64) -> Vorticity<N> {
    from_fn(|x, y| 2.0 * k * (k * x).cos() * (k * y).cos())
}

/// Doppelte Scherschicht (Bell, Colella, Glaz 1989): zwei gegenläufige
/// Strahlen mit kleiner Störung; sie rollen sich zu Wirbeln auf.
pub fn double_shear_layer<const N: usize>(thickness: f64, perturbation: f64) -> Vorticity<N> {
    let sech2 = |s: f64| 1.0 / s.cosh().powi(2);
    from_fn(|x, y| {
        let shear = if y <= PI {
            -sech2((y - PI / 2.0) / thickness) / thickness
        } else {
            sech2((3.0 * PI / 2.0 - y) / thickness) / thickness
        };
        shear + perturbation * x.cos()
    })
}
