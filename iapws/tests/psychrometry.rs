//! Psychrometrie nach IAPWS-10, verglichen mit Referenzwerten.

use iapws::psychrometry::*;

const P: f64 = 0.101_325; // MPa
const T0: f64 = 273.15;

/// Sättigungs-Wassergehalt bei 1 atm. Referenz: ASHRAE Handbook
/// Fundamentals, Tabelle 2 (dort mit anderen Virialansätzen, daher ±0.1 %).
#[test]
fn saturation_humidity_ratio_matches_ashrae() {
    for (tc, ws) in [(0.0, 0.003_789), (20.0, 0.014_758), (30.0, 0.027_329)] {
        let a = saturation_air_fraction(T0 + tc, P, Condensate::Liquid).unwrap();
        let got = humidity_ratio(a);
        assert!((got - ws).abs() / ws < 1e-3, "{tc} °C: {got} statt {ws}");
    }
}

/// Gesättigte Luft: Tau- und Feuchtkugeltemperatur sind gleich der Lufttemperatur.
#[test]
fn saturated_air_is_its_own_dew_and_wet_bulb() {
    let t = T0 + 20.0;
    let a = from_relative_humidity(1.0, t, P).unwrap();
    assert!((dew_point(a, P).unwrap() - t).abs() < 1e-6);
    assert!((wet_bulb(a, t, P).unwrap() - t).abs() < 1e-6);
    assert!((relative_humidity(a, t, P).unwrap() - 1.0).abs() < 1e-10);
}

/// 25 °C, 50 %: Taupunkt 13.86 °C, Feuchtkugel 17.9 °C (psychrometrisches
/// Diagramm, 1 atm).
#[test]
fn typical_room_air() {
    let t = T0 + 25.0;
    let a = from_relative_humidity(0.5, t, P).unwrap();
    assert!((relative_humidity(a, t, P).unwrap() - 0.5).abs() < 1e-10);
    let td = dew_point(a, P).unwrap() - T0;
    let tw = wet_bulb(a, t, P).unwrap() - T0;
    assert!((td - 13.86).abs() < 0.03, "Taupunkt {td}");
    assert!((tw - 17.9).abs() < 0.1, "Feuchtkugel {tw}");
    // Immer: Taupunkt ≤ Feuchtkugel ≤ Lufttemperatur
    assert!(td < tw && tw < 25.0);
}

/// Unter 0 °C: Reifpunkt über Eis und Eiskugel.
#[test]
fn below_freezing() {
    let t = T0 - 5.0;
    let a = from_relative_humidity(0.6, t, P).unwrap();
    let tf = frost_point(a, P).unwrap() - T0;
    let td = dew_point(a, P).unwrap() - T0;
    let tw = wet_bulb(a, t, P).unwrap() - T0;
    // Reifpunkt liegt über dem (unterkühlten) Taupunkt, Eiskugel dazwischen bis −5 °C
    assert!(
        td < tf && tf < tw && tw < -5.0,
        "Taupunkt {td}, Reifpunkt {tf}, Eiskugel {tw}"
    );
}
