//! IAPWS-95: Zustandsgleichung für gewöhnliches Wasser (Revision 2018).
//!
//! Die Gleichung beschreibt die spezifische Helmholtz-Energie
//! `f(ρ, T) / (R·T) = φ(δ, τ) = φ°(δ, τ) + φʳ(δ, τ)` mit `δ = ρ/ρc`, `τ = Tc/T`.
//! Alle Zustandsgrößen folgen aus `φ` und seinen Ableitungen bis zur
//! 2. Ordnung. Diese kommen hier per AutoDiff ([`partials_2`]); kein einziger
//! Ableitungsterm ist von Hand geschrieben.
//!
//! `φ°` und `φʳ` sind generisch über [`Real`] geschrieben: Dieselbe Formel
//! rechnet mit `f64`, mit dualen Zahlen (Ableitungen) oder komplexen Zahlen.

use math_traits::autodiff::{Partials2, partials_2};
use math_traits::num::{Num, Real};

/// Kritische Temperatur in K.
pub const TC: f64 = 647.096;
/// Kritische Dichte in kg/m³.
pub const RHOC: f64 = 322.0;
/// Spezifische Gaskonstante in kJ/(kg·K).
pub const R: f64 = 0.461_518_05;

// --- Idealgas-Anteil (Tabelle 1) ------------------------------------------------------

const N0: [f64; 8] = [
    -8.320_446_483_749_7,
    6.683_210_527_593_2,
    3.006_32,
    0.012_436,
    0.973_15,
    1.279_5,
    0.969_56,
    0.248_73,
];
const GAMMA0: [f64; 5] = [
    1.287_289_67,
    3.537_342_22,
    7.740_737_08,
    9.244_377_96,
    27.507_510_5,
];

/// `φ°(δ, τ) = ln δ + n₁ + n₂τ + n₃ ln τ + Σ nᵢ ln(1 − e^(−γᵢτ))`
pub fn phi0<N: Real>(delta: N, tau: N) -> N {
    let mut phi = delta.ln() + tau * N0[1] + tau.ln() * N0[2] + N0[0];
    for (n, g) in N0[3..].iter().zip(GAMMA0) {
        phi = phi + (-(tau * -g).exp() + 1.0).ln() * *n;
    }
    phi
}

// --- Residualanteil (Tabelle 2) --------------------------------------------------------

/// Terme 1–7: `nᵢ δ^dᵢ τ^tᵢ`
const POLY: [(f64, i32, f64); 7] = [
    (0.125_335_479_355_23e-1, 1, -0.5),
    (0.789_576_347_228_28e1, 1, 0.875),
    (-0.878_032_033_035_61e1, 1, 1.0),
    (0.318_025_093_454_18, 2, 0.5),
    (-0.261_455_338_593_58, 2, 0.75),
    (-0.781_997_516_879_81e-2, 3, 0.375),
    (0.880_894_931_021_34e-2, 4, 1.0),
];

/// Terme 8–51: `nᵢ δ^dᵢ τ^tᵢ e^(−δ^cᵢ)` als `(n, c, d, t)`
const EXP: [(f64, i32, i32, f64); 44] = [
    (-0.668_565_723_079_65, 1, 1, 4.0),
    (0.204_338_109_509_65, 1, 1, 6.0),
    (-0.662_126_050_396_87e-4, 1, 1, 12.0),
    (-0.192_327_211_560_02, 1, 2, 1.0),
    (-0.257_090_430_034_38, 1, 2, 5.0),
    (0.160_748_684_862_51, 1, 3, 4.0),
    (-0.400_928_289_258_7e-1, 1, 4, 2.0),
    (0.393_434_226_032_54e-6, 1, 4, 13.0),
    (-0.759_413_770_881_44e-5, 1, 5, 9.0),
    (0.562_509_793_518_88e-3, 1, 7, 3.0),
    (-0.156_086_522_571_35e-4, 1, 9, 4.0),
    (0.115_379_964_229_51e-8, 1, 10, 11.0),
    (0.365_821_651_442_04e-6, 1, 11, 4.0),
    (-0.132_511_800_746_68e-11, 1, 13, 13.0),
    (-0.626_395_869_124_54e-9, 1, 15, 1.0),
    (-0.107_936_009_089_32, 2, 1, 7.0),
    (0.176_114_910_087_52e-1, 2, 2, 1.0),
    (0.221_322_951_675_46, 2, 2, 9.0),
    (-0.402_476_697_635_28, 2, 2, 10.0),
    (0.580_833_999_857_59, 2, 3, 10.0),
    (0.499_691_469_908_06e-2, 2, 4, 3.0),
    (-0.313_587_007_125_49e-1, 2, 4, 7.0),
    (-0.743_159_297_103_41, 2, 4, 10.0),
    (0.478_073_299_154_80, 2, 5, 10.0),
    (0.205_279_408_959_48e-1, 2, 6, 6.0),
    (-0.136_364_351_103_43, 2, 6, 10.0),
    (0.141_806_344_006_17e-1, 2, 7, 10.0),
    (0.833_265_048_807_13e-2, 2, 9, 1.0),
    (-0.290_523_360_095_85e-1, 2, 9, 2.0),
    (0.386_150_855_742_06e-1, 2, 9, 3.0),
    (-0.203_934_865_137_04e-1, 2, 9, 4.0),
    (-0.165_540_500_637_34e-2, 2, 9, 8.0),
    (0.199_555_719_795_41e-2, 2, 10, 6.0),
    (0.158_703_083_241_57e-3, 2, 10, 9.0),
    (-0.163_885_683_425_30e-4, 2, 12, 8.0),
    (0.436_136_157_238_11e-1, 3, 3, 16.0),
    (0.349_940_054_637_65e-1, 3, 4, 22.0),
    (-0.767_881_978_446_21e-1, 3, 4, 23.0),
    (0.224_462_773_320_06e-1, 3, 5, 23.0),
    (-0.626_897_104_146_85e-4, 4, 14, 10.0),
    (-0.557_111_185_656_45e-9, 6, 3, 50.0),
    (-0.199_057_183_544_08, 6, 6, 44.0),
    (0.317_774_973_307_38, 6, 6, 46.0),
    (-0.118_411_824_259_81, 6, 6, 50.0),
];

/// Terme 52–54: `nᵢ δ^dᵢ τ^tᵢ exp(−αᵢ(δ − εᵢ)² − βᵢ(τ − γᵢ)²)`
/// als `(n, d, t, α, β, γ, ε)`
const GAUSS: [(f64, i32, f64, f64, f64, f64, f64); 3] = [
    (-0.313_062_603_234_35e2, 3, 0.0, 20.0, 150.0, 1.21, 1.0),
    (0.315_461_402_377_81e2, 3, 1.0, 20.0, 150.0, 1.21, 1.0),
    (-0.252_131_543_416_95e4, 3, 4.0, 20.0, 250.0, 1.25, 1.0),
];

/// Ein nichtanalytischer Term `n Δ^b δ ψ` (Terme 55–56).
struct NonAnalytic {
    n: f64,
    a: f64,
    b: f64,
    big_b: f64,
    big_c: f64,
    big_d: f64,
    big_a: f64,
    beta: f64,
}

const NONANALYTIC: [NonAnalytic; 2] = [
    NonAnalytic {
        n: -0.148_746_408_567_24,
        a: 3.5,
        b: 0.85,
        big_b: 0.2,
        big_c: 28.0,
        big_d: 700.0,
        big_a: 0.32,
        beta: 0.3,
    },
    NonAnalytic {
        n: 0.318_061_108_784_44,
        a: 3.5,
        b: 0.95,
        big_b: 0.2,
        big_c: 32.0,
        big_d: 800.0,
        big_a: 0.32,
        beta: 0.3,
    },
];

/// `φʳ(δ, τ)`: Summe der 56 Terme aus Tabelle 2.
pub fn phir<N: Real>(delta: N, tau: N) -> N {
    let mut phi = N::constant(0.0);
    for (n, d, t) in POLY {
        phi = phi + delta.powi(d) * tau.powf(t) * n;
    }
    for (n, c, d, t) in EXP {
        phi = phi + delta.powi(d) * tau.powf(t) * (-delta.powi(c)).exp() * n;
    }
    for (n, d, t, alpha, beta, gamma, eps) in GAUSS {
        let de = delta - eps;
        let tg = tau - gamma;
        phi = phi + delta.powi(d) * tau.powf(t) * (de * de * -alpha - tg * tg * beta).exp() * n;
    }
    for t in &NONANALYTIC {
        let dm1_sq = (delta - 1.0) * (delta - 1.0);
        let theta = -tau + 1.0 + dm1_sq.powf(1.0 / (2.0 * t.beta)) * t.big_a;
        let big_delta = theta * theta + dm1_sq.powf(t.a) * t.big_b;
        let psi = (dm1_sq * -t.big_c - (tau - 1.0) * (tau - 1.0) * t.big_d).exp();
        phi = phi + big_delta.powf(t.b) * delta * psi * t.n;
    }
    phi
}

/// `φ°` mit allen Ableitungen bis zur 2. Ordnung nach `(δ, τ)`.
pub fn ideal(delta: f64, tau: f64) -> Partials2<f64> {
    partials_2(|d, t| phi0(Num(d), Num(t)).0, delta, tau)
}

/// `φʳ` mit allen Ableitungen bis zur 2. Ordnung nach `(δ, τ)`.
pub fn residual(delta: f64, tau: f64) -> Partials2<f64> {
    partials_2(|d, t| phir(Num(d), Num(t)).0, delta, tau)
}

/// Zustand bei gegebener Temperatur und Dichte.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// Temperatur in K
    pub t: f64,
    /// Dichte in kg/m³
    pub rho: f64,
    /// Druck in MPa
    pub p: f64,
    /// spezifische innere Energie in kJ/kg
    pub u: f64,
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

/// Alle Zustandsgrößen aus `φ` und seinen Ableitungen (Tabelle 3).
pub fn state(t: f64, rho: f64) -> State {
    let delta = rho / RHOC;
    let tau = TC / t;
    let o = ideal(delta, tau);
    let r = residual(delta, tau);

    let rt = R * t;
    let phi_t = o.fy + r.fy;
    let phi_tt = o.fyy + r.fyy;
    let a = 1.0 + delta * r.fx - delta * tau * r.fxy;
    let b = 1.0 + 2.0 * delta * r.fx + delta * delta * r.fxx;

    let cv = -R * tau * tau * phi_tt;
    State {
        t,
        rho,
        p: rho * rt * (1.0 + delta * r.fx) / 1000.0,
        u: rt * tau * phi_t,
        h: rt * (1.0 + tau * phi_t + delta * r.fx),
        s: R * (tau * phi_t - o.f - r.f),
        cv,
        cp: cv + R * a * a / b,
        w: (1000.0 * rt * (b - a * a / (tau * tau * phi_tt))).sqrt(),
    }
}

/// Dichte in kg/m³ zu Temperatur `t` (K) und Druck `p` (MPa), per Newton
/// ausgehend von `rho_guess`. Die Ableitung `∂p/∂ρ` kommt ebenfalls aus `φʳ`.
/// `None`, wenn Newton nicht konvergiert.
pub fn density(t: f64, p: f64, rho_guess: f64) -> Option<f64> {
    let tau = TC / t;
    let mut rho = rho_guess;
    for _ in 0..100 {
        let delta = rho / RHOC;
        let r = residual(delta, tau);
        let p_calc = rho * R * t * (1.0 + delta * r.fx) / 1000.0;
        let dp_drho = R * t * (1.0 + 2.0 * delta * r.fx + delta * delta * r.fxx) / 1000.0;
        let step = (p_calc - p) / dp_drho;
        rho -= step;
        if !rho.is_finite() || rho <= 0.0 {
            return None;
        }
        if step.abs() <= 1e-12 * rho {
            return Some(rho);
        }
    }
    None
}

// --- Sättigung (Zweiphasengleichgewicht) ---------------------------------------------------

/// Kritischer Druck in MPa.
pub const PC: f64 = 22.064;
/// Tripelpunkt-Temperatur in K.
pub const TT: f64 = 273.16;

/// Gesättigte Flüssigkeit und gesättigter Dampf bei derselben Temperatur.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Saturation {
    /// Sättigungsdruck in MPa
    pub p: f64,
    pub liquid: State,
    pub vapor: State,
}

/// Hilfsgleichungen (Wagner & Pruß 1993) für Startwerte: `(p_σ, ρ', ρ'')`.
fn auxiliary(t: f64) -> (f64, f64, f64) {
    let th = 1.0 - t / TC;
    const A: [f64; 6] = [
        -7.859_517_83,
        1.844_082_59,
        -11.786_649_7,
        22.680_741_1,
        -15.961_871_9,
        1.801_225_02,
    ];
    const B: [f64; 6] = [
        1.992_740_64,
        1.099_653_42,
        -0.510_839_303,
        -1.754_934_79,
        -45.517_035_2,
        -6.746_944_50e5,
    ];
    const C: [f64; 6] = [
        -2.031_502_40,
        -2.683_029_40,
        -5.386_264_92,
        -17.299_160_5,
        -44.758_658_1,
        -63.920_106_3,
    ];
    let ea = [1.0, 1.5, 3.0, 3.5, 4.0, 7.5];
    let eb = [
        1.0 / 3.0,
        2.0 / 3.0,
        5.0 / 3.0,
        16.0 / 3.0,
        43.0 / 3.0,
        110.0 / 3.0,
    ];
    let ec = [
        2.0 / 6.0,
        4.0 / 6.0,
        8.0 / 6.0,
        18.0 / 6.0,
        37.0 / 6.0,
        71.0 / 6.0,
    ];
    let sum =
        |k: &[f64; 6], e: &[f64; 6]| k.iter().zip(e).map(|(k, e)| k * th.powf(*e)).sum::<f64>();
    let p = PC * (TC / t * sum(&A, &ea)).exp();
    let rho_l = RHOC * (1.0 + sum(&B, &eb));
    let rho_v = RHOC * sum(&C, &ec).exp();
    (p, rho_l, rho_v)
}

/// Druck (kPa), `∂p/∂ρ` und spezifische Gibbs-Energie (kJ/kg) bei `(T, ρ)`.
fn p_dp_g(t: f64, rho: f64) -> (f64, f64, f64) {
    let delta = rho / RHOC;
    let tau = TC / t;
    let r = residual(delta, tau);
    let o = ideal(delta, tau);
    let rt = R * t;
    let p = rho * rt * (1.0 + delta * r.fx);
    let dp = rt * (1.0 + 2.0 * delta * r.fx + delta * delta * r.fxx);
    let g = rt * (1.0 + o.f + r.f + delta * r.fx);
    (p, dp, g)
}

/// Sättigungszustand bei der Temperatur `t` (K), gültig für `TT ≤ t < TC`.
///
/// Gelöst wird `p(ρ') = p(ρ'')` und `g(ρ') = g(ρ'')` per Newton. Die
/// Jacobi-Matrix folgt aus `∂g/∂ρ = (1/ρ)·∂p/∂ρ` (Gibbs-Duhem bei festem `T`).
pub fn saturation(t: f64) -> Option<Saturation> {
    use math_traits::signature::Contract;
    use math_traits::tensor::{Tensor1, Tensor2, matrix::inverse};
    if !(TT..TC).contains(&t) {
        return None;
    }
    let (_, mut rl, mut rv) = auxiliary(t);
    for _ in 0..100 {
        let (pl, dpl, gl) = p_dp_g(t, rl);
        let (pv, dpv, gv) = p_dp_g(t, rv);
        let f = Tensor1([pl - pv, gl - gv]);
        let j = Tensor2([[dpl, -dpv], [dpl / rl, -dpv / rv]]);
        let step = inverse(&j)?.contract(&f);
        rl -= step.0[0];
        rv -= step.0[1];
        if !(rl.is_finite() && rv.is_finite()) || rv <= 0.0 || rl <= rv {
            return None;
        }
        if step.0[0].abs() <= 1e-13 * rl && step.0[1].abs() <= 1e-13 * rv {
            let liquid = state(t, rl);
            let vapor = state(t, rv);
            return Some(Saturation {
                p: (liquid.p + vapor.p) / 2.0,
                liquid,
                vapor,
            });
        }
    }
    None
}

/// Sättigungsdruck in MPa bei der Temperatur `t` (K).
pub fn saturation_pressure(t: f64) -> Option<f64> {
    saturation(t).map(|s| s.p)
}

/// Siedetemperatur in K beim Druck `p` (MPa), per Bisektion über
/// [`saturation_pressure`]. Gültig zwischen Tripelpunkt und kritischem Punkt.
pub fn saturation_temperature(p: f64) -> Option<f64> {
    use math_traits::solve::bisect;
    bisect(
        |t: &f64| saturation_pressure(*t).map_or(f64::NAN, |ps| ps - p),
        TT,
        TC - 1e-3,
        &1e-10,
        200,
    )
}

/// Spezifische Gibbs-Energie `g = f + p/ρ` in kJ/kg bei `(T, ρ)`.
pub fn gibbs(t: f64, rho: f64) -> f64 {
    p_dp_g(t, rho).2
}
