//! Psychrometrie: feuchte Luft bei gegebenem Druck, streng nach IAPWS-10.
//!
//! Alle Größen folgen aus Gleichgewichten der chemischen Potentiale, nicht
//! aus Näherungsformeln (Magnus o. ä.):
//!
//! * **Sättigung**: `μ_W(A, T, p) = g_Kondensat(T, p)`, Kondensat ist
//!   flüssiges Wasser (IAPWS-95) oder Eis (IAPWS-06).
//! * **Relative Feuchte** (WMO): `RH = x_V / x_V,sat` über Molenbrüche.
//! * **Taupunkt / Reifpunkt**: Temperatur, bei der die Luft gerade gesättigt ist.
//! * **Feuchtkugel** (adiabatische Sättigung): Energiebilanz pro kg trockener
//!   Luft, `h/A + (W_s − W)·h_c(T_w) = h_s/A_s`. Unter 0 °C mit Eis
//!   (Eiskugel).
//!
//! Einheiten: `T` in K, `p` in MPa, Enthalpien in kJ/kg, `A` = Massenanteil
//! trockener Luft in kg/kg, `W = (1 − A)/A` = Wassergehalt in kg/kg.

use crate::iapws10::{HumidAir, MA, MW, humid_air, humid_derivatives};
use crate::{iapws06, iapws95};
use math_traits::solve::bisect;

/// Molare Gaskonstante in kJ/(mol·K) für die Startdichte.
const R_MOL: f64 = 8.314_472e-3;

/// Womit die Luft im Gleichgewicht steht.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condensate {
    /// flüssiges Wasser (IAPWS-95)
    Liquid,
    /// Eis Ih (IAPWS-06)
    Ice,
}

impl Condensate {
    /// Unterhalb des Tripelpunkts Eis, sonst Wasser.
    pub fn stable_at(t: f64) -> Self {
        if t < iapws06::TT {
            Condensate::Ice
        } else {
            Condensate::Liquid
        }
    }

    /// Spezifische Gibbs-Energie und Enthalpie des Kondensats in kJ/kg.
    fn g_h(self, t: f64, p: f64) -> Option<(f64, f64)> {
        match self {
            Condensate::Liquid => {
                let rho = iapws95::density(t, p, 1000.0)?;
                Some((iapws95::gibbs(t, rho), iapws95::state(t, rho).h))
            }
            Condensate::Ice => {
                let s = iapws06::ice(t, p * 1e6);
                Some((s.g / 1000.0, s.h / 1000.0))
            }
        }
    }
}

/// Dichte feuchter Luft in kg/m³ bei `(A, T, p)`, per Newton ab der Idealgas-Dichte.
pub fn density(a: f64, t: f64, p: f64) -> Option<f64> {
    let r_spec = R_MOL * (a / MA + (1.0 - a) / MW) / 1000.0;
    let mut rho = p * 1000.0 / (r_spec * 1000.0 * t);
    for _ in 0..100 {
        let h = humid_derivatives(a, t, rho);
        let (fd, fdd) = (h.grad[2], h.hess[2][2]);
        let step = (rho * rho * fd / 1000.0 - p) / ((2.0 * rho * fd + rho * rho * fdd) / 1000.0);
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

/// Feuchte Luft bei Luftanteil `a`, Temperatur `t` (K) und Druck `p` (MPa).
pub fn at_pressure(a: f64, t: f64, p: f64) -> Option<HumidAir> {
    density(a, t, p).map(|rho| humid_air(a, t, rho))
}

/// Molenbruch des Wasserdampfs zu Luftanteil `a`.
pub fn vapor_mole_fraction(a: f64) -> f64 {
    let n_air = a / MA;
    let n_water = (1.0 - a) / MW;
    n_water / (n_air + n_water)
}

/// Luftanteil zu Dampf-Molenbruch `x_v`.
pub fn air_fraction(x_v: f64) -> f64 {
    let (m_air, m_water) = ((1.0 - x_v) * MA, x_v * MW);
    m_air / (m_air + m_water)
}

/// Wassergehalt `W = (1 − A)/A` in kg Wasser pro kg trockener Luft.
pub fn humidity_ratio(a: f64) -> f64 {
    (1.0 - a) / a
}

/// Gesättigte Luft: Luftanteil `A_sat` bei `(T, p)` über `cond`.
///
/// Die Klammer für die Bisektion wird vom trockenen Ende her gesucht: Der
/// Dampf-Molenbruch wird verdoppelt, bis `μ_W` die Gibbs-Energie des
/// Kondensats übersteigt (oberhalb von 0.5 wird stattdessen der Abstand zu 1
/// halbiert). So bleibt die Suche in dem Bereich, in dem die
/// Gasphase existiert (stark übersättigter Dampf hat keine Gasdichte mehr).
pub fn saturation_air_fraction(t: f64, p: f64, cond: Condensate) -> Option<f64> {
    let (g_cond, _) = cond.g_h(t, p)?;
    let f = |a: f64| at_pressure(a, t, p).map(|s| s.mu_water - g_cond);
    let mut x_dry = 1e-9;
    let mut x = x_dry;
    loop {
        match f(air_fraction(x)) {
            Some(v) if v > 0.0 => break,
            Some(_) => x_dry = x,
            None => return None,
        }
        // Verdoppeln bis 0.5, danach den Abstand zu 1 halbieren
        x = if x < 0.25 { 2.0 * x } else { (1.0 + x) / 2.0 };
        if x > 1.0 - 1e-9 {
            return None;
        }
    }
    bisect(
        |a: &f64| f(*a).unwrap_or(f64::NAN),
        air_fraction(x),
        air_fraction(x_dry),
        &1e-15,
        200,
    )
}

/// Relative Feuchte (WMO, über flüssigem Wasser), 0…1.
pub fn relative_humidity(a: f64, t: f64, p: f64) -> Option<f64> {
    let a_sat = saturation_air_fraction(t, p, Condensate::Liquid)?;
    Some(vapor_mole_fraction(a) / vapor_mole_fraction(a_sat))
}

/// Luftanteil zu relativer Feuchte `rh` (0…1) bei `(T, p)`.
pub fn from_relative_humidity(rh: f64, t: f64, p: f64) -> Option<f64> {
    let a_sat = saturation_air_fraction(t, p, Condensate::Liquid)?;
    Some(air_fraction(rh * vapor_mole_fraction(a_sat)))
}

/// Taupunkt in K: Temperatur, bei der Luft mit Luftanteil `a` über flüssigem
/// Wasser gesättigt ist.
pub fn dew_point(a: f64, p: f64) -> Option<f64> {
    let f =
        |t: &f64| saturation_air_fraction(*t, p, Condensate::Liquid).map_or(f64::NAN, |s| s - a);
    bisect(f, 235.0, 370.0, &1e-10, 200)
}

/// Reifpunkt in K: Sättigung über Eis.
pub fn frost_point(a: f64, p: f64) -> Option<f64> {
    let f = |t: &f64| saturation_air_fraction(*t, p, Condensate::Ice).map_or(f64::NAN, |s| s - a);
    bisect(f, 190.0, iapws06::TT, &1e-10, 200)
}

/// Thermodynamische Feuchtkugeltemperatur in K (adiabatische Sättigung).
/// Unter 0 °C ist das die Eiskugeltemperatur.
pub fn wet_bulb(a: f64, t: f64, p: f64) -> Option<f64> {
    let air = at_pressure(a, t, p)?;
    let h_per_dry = air.h / a;
    let w = humidity_ratio(a);
    let residual = |tw: &f64| -> f64 {
        let cond = Condensate::stable_at(*tw);
        let calc = || -> Option<f64> {
            let a_s = saturation_air_fraction(*tw, p, cond)?;
            let (_, h_c) = cond.g_h(*tw, p)?;
            let sat = at_pressure(a_s, *tw, p)?;
            Some(h_per_dry + (humidity_ratio(a_s) - w) * h_c - sat.h / a_s)
        };
        calc().unwrap_or(f64::NAN)
    };
    bisect(residual, (t - 80.0).max(200.0), t, &1e-9, 200)
}
