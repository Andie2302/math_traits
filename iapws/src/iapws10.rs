//! IAPWS G8-10: feuchte Luft, mit trockener Luft nach Lemmon et al. (2000).
//!
//! **Trockene Luft** ist hier ein Pseudo-Reinstoff mit eigener
//! Helmholtz-Gleichung `f_A(T, ρ_A)` (Lemmon, Jacobsen, Penoncello, Friend,
//! J. Phys. Chem. Ref. Data 29, 331 (2000)), mit den Referenzkonstanten aus
//! IAPWS-10 (h = s = 0 bei 273.15 K und 101325 Pa).
//!
//! **Feuchte Luft** mit Massenanteil `A` trockener Luft und Dichte `ρ`:
//!
//! ```text
//! f_AV(A, T, ρ) = (1 − A)·f_V(T, (1 − A)ρ) + A·f_A(T, Aρ) + f_mix(A, T, ρ)
//! ```
//!
//! mit `f_V` aus IAPWS-95 und der Wechselwirkung `f_mix` über die
//! Kreuz-Virialkoeffizienten `B_aw`, `C_aaw`, `C_aww`.
//!
//! Die Veröffentlichung gibt für die Ableitungen von `f_AV` und `f_mix`
//! rund 30 Formeln an (T11–T20, T45–T63). Hier steht **keine** davon: Alle
//! Ableitungen nach `(A, T, ρ)` kommen per AutoDiff aus [`hessian`].
//!
//! Koeffizienten nach den Originalveröffentlichungen, abgeglichen mit dem
//! Python-Paket `iapws` (J. J. Gómez Romera).

use crate::iapws95;
use math_traits::autodiff::{Dual, Hessian, Partials2, hessian, partials_2};
use math_traits::num::{Num, Real};

/// Molmasse trockener Luft in kg/mol.
pub const MA: f64 = 28.965_46e-3;
/// Molmasse von Wasser in kg/mol.
pub const MW: f64 = 18.015_268e-3;
/// Molare Gaskonstante für die Mischung (IAPWS-10) in J/(mol·K).
const R_MIX: f64 = 8.314_472;

// --- Trockene Luft (Lemmon 2000, Referenz nach IAPWS-10) --------------------------------

/// Reduzierende Temperatur in K.
pub const TJ: f64 = 132.631_2;
/// Reduzierende Dichte in kg/m³ (10.4477 mol/dm³).
pub const RHOJ: f64 = 10.4477 * 28.965_46;
/// Spezifische Gaskonstante trockener Luft in kJ/(kg·K) (Lemmon: R = 8.31451).
pub const RA: f64 = 8.314_51 / 28.965_46;

/// `(nᵢ, Exponent von τ)` im Idealgas-Anteil
const AIR0_POW: [(f64, f64); 6] = [
    (0.605_719_4e-7, -3.0),
    (-0.210_274_769e-4, -2.0),
    (-0.158_860_716e-3, -1.0),
    (9.745_025_174_394_8, 0.0),
    (10.098_614_742_891_2, 1.0),
    (-0.195_363_42e-3, 1.5),
];

/// `φ°(δ, τ)` der trockenen Luft.
pub fn air_phi0<N: Real>(delta: N, tau: N) -> N {
    let mut phi = delta.ln() + tau.ln() * 2.490_888_032;
    for (n, t) in AIR0_POW {
        phi = phi + tau.powf(t) * n;
    }
    for (n, g) in [(0.791_309_509, 25.363_65), (0.212_236_768, 16.907_41)] {
        phi = phi + (-(tau * -g).exp() + 1.0).ln() * n;
    }
    // −0.197938904 · ln(2/3 + e^(87.31279·τ))
    phi + ((tau * 87.312_79).exp() + 2.0 / 3.0).ln() * -0.197_938_904
}

/// Terme `n δ^d τ^t`
const AIR_POLY: [(f64, i32, f64); 10] = [
    (0.118_160_747_229, 1, 0.0),
    (0.713_116_392_079, 1, 0.33),
    (-0.161_824_192_067e1, 1, 1.01),
    (0.714_140_178_971e-1, 2, 0.0),
    (-0.865_421_396_646e-1, 3, 0.0),
    (0.134_211_176_704, 3, 0.15),
    (0.112_626_704_218e-1, 4, 0.0),
    (-0.420_533_228_842e-1, 4, 0.2),
    (0.349_008_431_982e-1, 4, 0.35),
    (0.164_957_183_186e-3, 6, 1.35),
];

/// Terme `n δ^d τ^t e^(−δ^c)` als `(n, d, t, c)`
const AIR_EXP: [(f64, i32, f64, i32); 9] = [
    (-0.101_365_037_912, 1, 1.6, 1),
    (-0.173_813_690_970, 3, 0.8, 1),
    (-0.472_103_183_731e-1, 5, 0.95, 1),
    (-0.122_523_554_253e-1, 6, 1.25, 1),
    (-0.146_629_609_713, 1, 3.6, 2),
    (-0.316_055_879_821e-1, 3, 6.0, 2),
    (0.233_594_806_142e-3, 11, 3.25, 2),
    (0.148_287_891_978e-1, 1, 3.5, 3),
    (-0.938_782_884_667e-2, 3, 15.0, 3),
];

/// `φʳ(δ, τ)` der trockenen Luft.
pub fn air_phir<N: Real>(delta: N, tau: N) -> N {
    let mut phi = N::constant(0.0);
    for (n, d, t) in AIR_POLY {
        phi = phi + delta.powi(d) * tau.powf(t) * n;
    }
    for (n, d, t, c) in AIR_EXP {
        phi = phi + delta.powi(d) * tau.powf(t) * (-delta.powi(c)).exp() * n;
    }
    phi
}

/// Spezifische Helmholtz-Energie trockener Luft in kJ/kg.
pub fn f_air<N: Real>(t: N, rho: N) -> N {
    let delta = rho / RHOJ;
    let tau = N::constant(TJ) / t;
    t * RA * (air_phi0(delta, tau) + air_phir(delta, tau))
}

/// Spezifische Helmholtz-Energie von Wasser(dampf) nach IAPWS-95 in kJ/kg.
pub fn f_water<N: Real>(t: N, rho: N) -> N {
    let delta = rho / iapws95::RHOC;
    let tau = N::constant(iapws95::TC) / t;
    t * iapws95::R * (iapws95::phi0(delta, tau) + iapws95::phir(delta, tau))
}

type D = Dual<Dual<f64>>;

/// `f_A` mit allen Ableitungen nach `(T, ρ)` in kJ/kg bzw. abgeleiteten Einheiten.
pub fn air_derivatives(t: f64, rho: f64) -> Partials2<f64> {
    partials_2(|t: D, r: D| f_air(Num(t), Num(r)).0, t, rho)
}

/// `f_V` (IAPWS-95) mit allen Ableitungen nach `(T, ρ)`.
pub fn water_derivatives(t: f64, rho: f64) -> Partials2<f64> {
    partials_2(|t: D, r: D| f_water(Num(t), Num(r)).0, t, rho)
}

// --- Wechselwirkung Luft–Wasser ------------------------------------------------------------

/// Kreuz-Virialkoeffizienten `(B_aw, C_aaw, C_aww)` in m³/mol bzw. m⁶/mol².
pub fn cross_virial<N: Real>(t: N) -> (N, N, N) {
    const A: [f64; 5] = [
        0.482_737e-3,
        0.105_678e-2,
        -0.656_394e-2,
        0.294_442e-1,
        -0.319_317e-1,
    ];
    const B: [f64; 4] = [-10.728_876, 34.780_2, -38.338_3, 33.406];
    const C: [(f64, f64); 3] = [(66.568_7, -0.237), (-238.834, -1.048), (-176.755, -3.183)];
    let tr = t / 100.0;
    let inv = N::constant(1.0) / tr;
    let mut baw = N::constant(0.0);
    for (c, d) in C {
        baw = baw + tr.powf(d) * c;
    }
    let (mut caaw, mut sum_b, mut p) = (N::constant(0.0), N::constant(0.0), N::constant(1.0));
    for (i, a) in A.iter().enumerate() {
        caaw = caaw + p * *a;
        if let Some(b) = B.get(i) {
            sum_b = sum_b + p * *b;
        }
        p = p * inv;
    }
    (baw * 1e-6, caaw * 1e-6, sum_b.exp() * -1e-6)
}

/// Wechselwirkungsanteil `f_mix(A, T, ρ)` in kJ/kg (Gleichung T45).
pub fn f_mix<N: Real>(a: N, t: N, rho: N) -> N {
    let (baw, caaw, caww) = cross_virial(t);
    let w = -a + 1.0;
    let virial = baw + rho * 0.75 * (a * caaw / MA + w * caww / MW);
    a * w * rho * t * virial * (2.0 * R_MIX / (MA * MW) / 1000.0)
}

/// Spezifische Helmholtz-Energie feuchter Luft `f_AV(A, T, ρ)` in kJ/kg.
pub fn f_humid<N: Real>(a: N, t: N, rho: N) -> N {
    let w = -a + 1.0;
    w * f_water(t, w * rho) + a * f_air(t, a * rho) + f_mix(a, t, rho)
}

/// `f_mix` mit Gradient und Hesse-Matrix nach `(A, T, ρ)`.
pub fn mix_derivatives(a: f64, t: f64, rho: f64) -> Hessian<f64, 3> {
    hessian(
        |[a, t, r]: [D; 3]| f_mix(Num(a), Num(t), Num(r)).0,
        [a, t, rho],
    )
}

/// `f_AV` mit Gradient und Hesse-Matrix nach `(A, T, ρ)`.
pub fn humid_derivatives(a: f64, t: f64, rho: f64) -> Hessian<f64, 3> {
    hessian(
        |[a, t, r]: [D; 3]| f_humid(Num(a), Num(t), Num(r)).0,
        [a, t, rho],
    )
}

/// Zustand feuchter Luft (Tabelle 5 und 12 der Veröffentlichung).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HumidAir {
    /// Massenanteil trockener Luft in kg/kg
    pub a: f64,
    /// Temperatur in K
    pub t: f64,
    /// Dichte in kg/m³
    pub rho: f64,
    /// Druck in MPa
    pub p: f64,
    /// spezifische Enthalpie in kJ/kg
    pub h: f64,
    /// spezifische Gibbs-Energie in kJ/kg
    pub g: f64,
    /// spezifische Entropie in kJ/(kg·K)
    pub s: f64,
    /// isobare Wärmekapazität in kJ/(kg·K)
    pub cp: f64,
    /// Schallgeschwindigkeit in m/s
    pub w: f64,
    /// chemisches Potential des Wassers in kJ/kg
    pub mu_water: f64,
}

/// Feuchte Luft bei Luftanteil `a`, Temperatur `t` (K) und Dichte `rho` (kg/m³).
pub fn humid_air(a: f64, t: f64, rho: f64) -> HumidAir {
    let h = humid_derivatives(a, t, rho);
    let (f, fa, ft, fd) = (h.f, h.grad[0], h.grad[1], h.grad[2]);
    let (ftt, fdt, fdd) = (h.hess[1][1], h.hess[1][2], h.hess[2][2]);
    let k = 2.0 * fd + rho * fdd;
    HumidAir {
        a,
        t,
        rho,
        p: rho * rho * fd / 1000.0,
        h: f - t * ft + rho * fd,
        g: f + rho * fd,
        s: -ft,
        cp: -t * ftt + t * rho * fdt * fdt / k,
        w: (rho * rho * 1000.0 * (ftt * fdd - fdt * fdt) / ftt + 2.0 * rho * fd * 1000.0).sqrt(),
        mu_water: f + rho * fd - a * fa,
    }
}

/// Zustand trockener Luft.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DryAir {
    /// Temperatur in K
    pub t: f64,
    /// Dichte in kg/m³
    pub rho: f64,
    /// Druck in MPa
    pub p: f64,
    /// spezifische Enthalpie in kJ/kg
    pub h: f64,
    /// spezifische Entropie in kJ/(kg·K)
    pub s: f64,
    /// isochore Wärmekapazität in kJ/(kg·K)
    pub cv: f64,
    /// isobare Wärmekapazität in kJ/(kg·K)
    pub cp: f64,
    /// Schallgeschwindigkeit in m/s
    pub w: f64,
}

/// Trockene Luft bei Temperatur `t` (K) und Dichte `rho` (kg/m³).
pub fn dry_air(t: f64, rho: f64) -> DryAir {
    let d = air_derivatives(t, rho);
    let (f, ft, fd, ftt, fdt, fdd) = (d.f, d.fx, d.fy, d.fxx, d.fxy, d.fyy);
    let k = 2.0 * fd + rho * fdd;
    DryAir {
        t,
        rho,
        p: rho * rho * fd / 1000.0,
        h: f - t * ft + rho * fd,
        s: -ft,
        cv: -t * ftt,
        cp: -t * ftt + t * rho * fdt * fdt / k,
        w: (rho * rho * 1000.0 * (ftt * fdd - fdt * fdt) / ftt + 2.0 * rho * fd * 1000.0).sqrt(),
    }
}

/// Dichte trockener Luft in kg/m³ bei `t` (K) und `p` (MPa), per Newton ab
/// der Idealgas-Dichte.
pub fn air_density(t: f64, p: f64) -> Option<f64> {
    let mut rho = p * 1000.0 / (RA * t);
    for _ in 0..100 {
        let d = air_derivatives(t, rho);
        let p_calc = rho * rho * d.fy / 1000.0;
        let dp = (2.0 * rho * d.fy + rho * rho * d.fyy) / 1000.0;
        let step = (p_calc - p) / dp;
        rho -= step;
        if !rho.is_finite() || rho <= 0.0 {
            return None;
        }
        if step.abs() <= 1e-13 * rho {
            return Some(rho);
        }
    }
    None
}
