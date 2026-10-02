//! IAPWS-06: Zustandsgleichung für Eis Ih (Revision 2009).
//!
//! Das Gibbs-Potential `g(T, p)` hat **komplexe** Koeffizienten:
//!
//! ```text
//! g(T, p) = g₀(p) − s₀·Tₜ·τ
//!         + Tₜ·Re Σₖ rₖ·[(tₖ − τ)ln(tₖ − τ) + (tₖ + τ)ln(tₖ + τ) − 2tₖ ln tₖ − τ²/tₖ]
//! ```
//!
//! mit `τ = T/Tₜ`, `π = p/pₜ`. Hier wird es wörtlich so ausgewertet, über
//! `Num<Complex<Dual<Dual<f64>>>>`: komplexe Zahlen über dualen Zahlen. Alle
//! Ableitungen nach `T` und `p` kommen per AutoDiff.

use math_traits::autodiff::{Dual, Partials2, partials_2};
use math_traits::impls::cayley_dickson::Complex;
use math_traits::num::Num;

/// Tripelpunkt-Temperatur in K.
pub const TT: f64 = 273.16;
/// Tripelpunkt-Druck in Pa.
pub const PT: f64 = 611.657;
/// Normaldruck in Pa.
pub const P0: f64 = 101_325.0;

const G0: [f64; 5] = [
    -0.632_020_233_335_886e6,
    0.655_022_213_658_955,
    -0.189_369_929_326_131e-7,
    0.339_746_123_271_053e-14,
    -0.556_464_869_058_991e-21,
];
/// Absolute Entropie-Konstante (passend zu IAPWS-95) in J/(kg·K).
const S0: f64 = -0.332_733_756_492_168e4;
const T1: (f64, f64) = (0.368_017_112_855_051e-1, 0.510_878_114_959_572e-1);
const R1: (f64, f64) = (0.447_050_716_285_388e2, 0.656_876_847_463_481e2);
const T2: (f64, f64) = (0.337_315_741_065_416, 0.335_449_415_919_309);
const R2: [(f64, f64); 3] = [
    (-0.725_974_574_329_220e2, -0.781_008_427_112_870e2),
    (-0.557_107_698_030_123e-4, 0.464_578_634_580_806e-4),
    (0.234_801_409_215_913e-10, -0.285_651_142_904_972e-10),
];

type D = Dual<Dual<f64>>;
type R = Num<D>;
type C = Num<Complex<D>>;

fn real(x: f64) -> R {
    Num::lift(x)
}

fn complex((re, im): (f64, f64)) -> C {
    Num(Complex::new(real(re).0, real(im).0))
}

fn embed(x: R) -> C {
    Num(Complex::new(x.0, real(0.0).0))
}

/// `(t − τ)ln(t − τ) + (t + τ)ln(t + τ) − 2t ln t − τ²/t`
fn bracket(t: C, tau: C) -> C {
    let (a, b) = (t - tau, t + tau);
    a * a.ln() + b * b.ln() - t * t.ln() * complex((2.0, 0.0)) - tau * tau / t
}

/// Spezifische Gibbs-Energie `g(T, p)` in J/kg, mit `T` in K und `p` in Pa.
fn gibbs(t: D, p: D) -> D {
    let tau = Num(t) / TT;
    let dpi = Num(p) / PT - P0 / PT;

    let mut g0 = real(0.0);
    let mut r2 = complex((0.0, 0.0));
    let mut pow = real(1.0);
    for (k, gk) in G0.iter().enumerate() {
        g0 += pow * *gk;
        if let Some(rk) = R2.get(k) {
            r2 += complex(*rk) * embed(pow);
        }
        pow = pow * dpi;
    }

    let tau_c = embed(tau);
    let sum = complex(R1) * bracket(complex(T1), tau_c) + r2 * bracket(complex(T2), tau_c);
    (g0 - tau * (S0 * TT) + Num(sum.0.re) * TT).0
}

/// `g` und alle Ableitungen bis zur 2. Ordnung nach `(T, p)` in SI-Einheiten.
pub fn gibbs_derivatives(t: f64, p: f64) -> Partials2<f64> {
    partials_2(gibbs, t, p)
}

/// Zustand von Eis Ih. Alle Größen in SI-Einheiten.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IceState {
    /// Temperatur in K
    pub t: f64,
    /// Druck in Pa
    pub p: f64,
    /// spezifische Gibbs-Energie in J/kg
    pub g: f64,
    /// Dichte in kg/m³
    pub rho: f64,
    /// spezifische Enthalpie in J/kg
    pub h: f64,
    /// spezifische innere Energie in J/kg
    pub u: f64,
    /// spezifische Helmholtz-Energie in J/kg
    pub f: f64,
    /// spezifische Entropie in J/(kg·K)
    pub s: f64,
    /// isobare Wärmekapazität in J/(kg·K)
    pub cp: f64,
    /// kubischer Ausdehnungskoeffizient in 1/K
    pub alpha: f64,
    /// Druckkoeffizient in Pa/K
    pub beta: f64,
    /// isotherme Kompressibilität in 1/Pa
    pub kappa_t: f64,
    /// isentrope Kompressibilität in 1/Pa
    pub kappa_s: f64,
}

/// Alle Zustandsgrößen aus `g` und seinen Ableitungen (Tabelle 3 der Veröffentlichung).
pub fn ice(t: f64, p: f64) -> IceState {
    let d = gibbs_derivatives(t, p);
    let (g, gt, gp, gtt, gtp, gpp) = (d.f, d.fx, d.fy, d.fxx, d.fxy, d.fyy);
    IceState {
        t,
        p,
        g,
        rho: 1.0 / gp,
        h: g - t * gt,
        u: g - t * gt - p * gp,
        f: g - p * gp,
        s: -gt,
        cp: -t * gtt,
        alpha: gtp / gp,
        beta: -gtp / gpp,
        kappa_t: -gpp / gp,
        kappa_s: (gtp * gtp - gtt * gpp) / (gp * gtt),
    }
}

/// Schmelztemperatur in K beim Druck `p` (Pa): dort, wo die Gibbs-Energien
/// von Eis (IAPWS-06) und Wasser (IAPWS-95) gleich sind.
pub fn melting_temperature(p: f64) -> Option<f64> {
    use crate::iapws95;
    use math_traits::solve::bisect;
    let difference = |t: &f64| {
        let rho = iapws95::density(*t, p / 1e6, 1000.0).unwrap_or(f64::NAN);
        let water = iapws95::gibbs(*t, rho) * 1000.0;
        ice(*t, p).g - water
    };
    bisect(difference, 250.0, TT + 0.01, &1e-10, 200)
}
