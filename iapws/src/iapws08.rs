//! IAPWS-08: Zustandsgleichung für Meerwasser (Salz-Anteil, TEOS-10).
//!
//! Meerwasser = Wasser + Salz-Anteil im Gibbs-Potential:
//!
//! ```text
//! g(S, T, p) = g_W(T, p) + g_S(S, T, p)
//! ```
//!
//! `g_W` kommt aus IAPWS-95 (über die Dichte bei `(T, p)`), `g_S` ist ein
//! Polynom in `ξ = sqrt(S/S*)`, `τ = (T − T₀)/40 K`, `π = (p − p₀)/10⁸ Pa`
//! mit einem Term `ξ² ln ξ`.
//!
//! Die Koeffizienten von `g_S` sind **maschinell** aus dem offiziellen
//! TEOS-10-Quellcode (GSW-C, `gsw_gibbs`) übernommen, nicht abgetippt. Die
//! Horner-Form bleibt dabei erhalten. Die Ableitung nach `S` aus GSW-C ist
//! ebenfalls übernommen, als unabhängige Gegenprobe zur AutoDiff-Ableitung
//! (siehe Tests).
//!
//! Herkunft der Koeffizienten: GSW-C, <https://github.com/TEOS-10/GSW-C>,
//! © SCOR/IAPSO WG127, BSD-3-Clause-Lizenz.

use math_traits::autodiff::{Dual, Partials2, partials_2};
use math_traits::num::Num;
use math_traits::signature::{
    Additive, BinaryOp, HasIdentity, HasInverse, HasLn, HasSqrt, Multiplicative, ScalarMul,
};

/// Referenz-Salzgehalt des Standardozeans in g/kg.
pub const SSO: f64 = 35.165_04;
/// `1 / (40 g/kg · 35.16504/35)`: Umrechnung `S` (g/kg) → `ξ²`.
const SFAC: f64 = 0.024_882_667_558_461_5;
/// 0 °C in K.
pub const T0: f64 = 273.15;
/// Normaldruck in Pa.
pub const P0: f64 = 101_325.0;

/// Salz-Anteil `g_S` in J/kg, mit `sa` in g/kg (`> 0`), `t` in °C und `p` als
/// Meeresdruck `p − p₀` in dbar. Generisch über `T`: `f64` für Werte,
/// duale Zahlen für Ableitungen.
pub fn saline<T>(sa: Num<T>, t: Num<T>, p: Num<T>) -> Num<T>
where
    T: Copy
        + BinaryOp<Additive>
        + HasInverse<Additive>
        + BinaryOp<Multiplicative>
        + HasIdentity<Multiplicative>
        + ScalarMul<f64>
        + HasLn
        + HasSqrt,
{
    let x2 = sa * SFAC;
    let x = x2.sqrt();
    let y = t * 0.025;
    let z = p * 1e-4;
    let g08 = x2
        * (1416.27648484197
            + z * (-3310.49154044839
                + z * (384.794152978599
                    + z * (-96.5324320107458 + (15.8408172766824 - 2.62480156590992 * z) * z)))
            + x * (-2432.14662381794
                + x * (2025.80115603697
                    + y * (543.835333000098
                        + y * (-68.5572509204491
                            + y * (49.3667694856254
                                + y * (-17.1397577419788 + 2.49697009569508 * y)))
                        - 22.6683558512829 * z)
                    + x * (-1091.66841042967 - 196.028306689776 * y
                        + x * (374.60123787784 - 48.5891069025409 * x + 36.7571622995805 * y)
                        + 36.0284195611086 * z)
                    + z * (-54.7919133532887 + (-4.08193978912261 - 30.1755111971161 * z) * z))
                + z * (199.459603073901
                    + z * (-52.2940909281335 + (68.0444942726459 - 3.41251932441282 * z) * z))
                + y * (-493.407510141682
                    + z * (-175.292041186547 + (83.1923927801819 - 29.483064349429 * z) * z)
                    + y * (-43.0664675978042
                        + z * (383.058066002476
                            + z * (-54.1917262517112 + 25.6398487389914 * z))
                        + y * (-10.0227370861875 - 460.319931801257 * z
                            + y * (0.875600661808945 + 234.565187611355 * z)))))
            + y * (168.072408311545
                + z * (729.116529735046
                    + z * (-343.956902961561
                        + z * (124.687671116248
                            + z * (-31.656964386073 + 7.04658803315449 * z))))
                + y * (880.031352997204
                    + y * (-225.267649263401
                        + y * (91.4260447751259
                            + y * (-21.6603240875311 + 2.13016970847183 * y)
                            + z * (-297.728741987187
                                + (74.726141138756 - 36.4872919001588 * z) * z))
                        + z * (694.244814133268
                            + z * (-204.889641964903
                                + (113.561697840594 - 11.1282734326413 * z) * z)))
                    + z * (-860.764303783977
                        + z * (337.409530269367
                            + z * (-178.314556207638
                                + (44.2040358308 - 7.92001547211682 * z) * z))))));
    g08 + x2 * (5812.81456626732 + 851.226734946706 * y) * x.ln()
}

/// `∂g_S/∂S` in J/(kg·(g/kg)), direkt aus GSW-C (handgeschriebene Ableitung).
pub fn saline_ds_gsw(sa: f64, t: f64, p: f64) -> f64 {
    let x2 = SFAC * sa;
    let x = x2.sqrt();
    let y = t * 0.025;
    let z = p * 1e-4;
    let g08 = 8645.36753595126
        + z * (-6620.98308089678
            + z * (769.588305957198
                + z * (-193.0648640214916 + (31.6816345533648 - 5.24960313181984 * z) * z)))
        + x * (-7296.43987145382
            + x * (8103.20462414788
                + y * (2175.341332000392
                    + y * (-274.2290036817964
                        + y * (197.4670779425016
                            + y * (-68.5590309679152 + 9.98788038278032 * y)))
                    - 90.6734234051316 * z)
                + x * (-5458.34205214835 - 980.14153344888 * y
                    + x * (2247.60742726704 - 340.1237483177863 * x + 220.542973797483 * y)
                    + 180.142097805543 * z)
                + z * (-219.1676534131548 + (-16.32775915649044 - 120.7020447884644 * z) * z))
            + z * (598.378809221703
                + z * (-156.8822727844005 + (204.1334828179377 - 10.23755797323846 * z) * z))
            + y * (-1480.222530425046
                + z * (-525.876123559641 + (249.57717834054571 - 88.449193048287 * z) * z)
                + y * (-129.1994027934126
                    + z * (1149.174198007428 + z * (-162.5751787551336 + 76.9195462169742 * z))
                    + y * (-30.0682112585625 - 1380.9597954037708 * z
                        + y * (2.626801985426835 + 703.695562834065 * z)))))
        + y * (1187.3715515697959
            + z * (1458.233059470092
                + z * (-687.913805923122
                    + z * (249.375342232496 + z * (-63.313928772146 + 14.09317606630898 * z))))
            + y * (1760.062705994408
                + y * (-450.535298526802
                    + y * (182.8520895502518
                        + y * (-43.3206481750622 + 4.26033941694366 * y)
                        + z * (-595.457483974374
                            + (149.452282277512 - 72.9745838003176 * z) * z))
                    + z * (1388.489628266536
                        + z * (-409.779283929806
                            + (227.123395681188 - 22.2565468652826 * z) * z)))
                + z * (-1721.528607567954
                    + z * (674.819060538734
                        + z * (-356.629112415276 + (88.4080716616 - 15.84003094423364 * z) * z)))));
    0.5 * SFAC * (g08 + (11625.62913253464 + 1702.453469893412 * y) * x.ln())
}

type D = Dual<Dual<f64>>;

/// `g_S` und alle Ableitungen bis zur 2. Ordnung nach `(T, p)` in SI
/// (`T` in K, `p` in Pa), bei festem Salzgehalt `sa` (g/kg).
pub fn saline_tp(sa: f64, t: f64, p: f64) -> Partials2<f64> {
    partials_2(
        |tt: D, pp: D| {
            let celsius = Num(tt) - T0;
            let dbar = (Num(pp) - P0) * 1e-4;
            saline(Num::lift(sa), celsius, dbar).0
        },
        t,
        p,
    )
}

/// `∂g_S/∂S` in J/(kg·(g/kg)) per AutoDiff, `T` in K, `p` in Pa.
pub fn saline_ds(sa: f64, t: f64, p: f64) -> f64 {
    let d = saline(
        Num(Dual::variable(sa)),
        Num(Dual::constant(t - T0)),
        Num(Dual::constant((p - P0) * 1e-4)),
    );
    d.0.eps
}

/// Zustand von Meerwasser. Alle Größen in SI-Einheiten.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seawater {
    /// absoluter Salzgehalt in g/kg
    pub sa: f64,
    /// Temperatur in K
    pub t: f64,
    /// Druck in Pa
    pub p: f64,
    /// spezifische Gibbs-Energie in J/kg
    pub g: f64,
    /// Dichte in kg/m³
    pub rho: f64,
    /// spezifische Entropie in J/(kg·K)
    pub s: f64,
    /// spezifische Enthalpie in J/kg
    pub h: f64,
    /// isobare Wärmekapazität in J/(kg·K)
    pub cp: f64,
    /// chemisches Potential des Wassers im Meerwasser in J/kg
    pub mu_water: f64,
}

/// Reines Wasser aus IAPWS-95 bei `(T, p)`: `(g, ∂g/∂T, ∂g/∂p, ∂²g/∂T²)` in SI.
/// `rho_guess` wählt die Phase (≈1000 für Flüssigkeit, klein für Dampf).
fn water(t: f64, p: f64, rho_guess: f64) -> Option<(f64, f64, f64, f64)> {
    use crate::iapws95;
    let rho = iapws95::density(t, p / 1e6, rho_guess)?;
    let st = iapws95::state(t, rho);
    Some((
        iapws95::gibbs(t, rho) * 1e3,
        -st.s * 1e3,
        1.0 / rho,
        -st.cp * 1e3 / t,
    ))
}

/// Meerwasser bei Salzgehalt `sa` (g/kg), Temperatur `t` (K), Druck `p` (Pa).
pub fn seawater(sa: f64, t: f64, p: f64) -> Option<Seawater> {
    let (gw, gw_t, gw_p, gw_tt) = water(t, p, 1000.0)?;
    let gs = saline_tp(sa, t, p);
    let g = gw + gs.f;
    let s = -(gw_t + gs.fx);
    Some(Seawater {
        sa,
        t,
        p,
        g,
        rho: 1.0 / (gw_p + gs.fy),
        s,
        h: g + t * s,
        cp: -t * (gw_tt + gs.fxx),
        mu_water: g - sa * saline_ds(sa, t, p),
    })
}

/// Gefriertemperatur in K: chemisches Potential des Wassers im Meerwasser
/// gleich der Gibbs-Energie von Eis (IAPWS-06).
pub fn freezing_temperature(sa: f64, p: f64) -> Option<f64> {
    use crate::iapws06::ice;
    use math_traits::solve::bisect;
    let difference =
        |t: &f64| seawater(sa, *t, p).map_or(f64::NAN, |sw| sw.mu_water - ice(*t, p).g);
    bisect(difference, 250.0, 273.16, &1e-10, 200)
}

/// Siedetemperatur in K: chemisches Potential des Wassers im Meerwasser
/// gleich der Gibbs-Energie von Wasserdampf (IAPWS-95).
pub fn boiling_temperature(sa: f64, p: f64) -> Option<f64> {
    use math_traits::solve::bisect;
    let difference = |t: &f64| {
        let vapor = water(*t, p, 0.5).map(|w| w.0);
        match (seawater(sa, *t, p), vapor) {
            (Some(sw), Some(gv)) => sw.mu_water - gv,
            _ => f64::NAN,
        }
    };
    bisect(difference, 350.0, 420.0, &1e-10, 200)
}
