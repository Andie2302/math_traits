//! IAPWS-08 / TEOS-10: Meerwasser.

use iapws::iapws08::*;

/// Die AutoDiff-Ableitung der maschinell übernommenen Gleichung stimmt mit
/// der handgeschriebenen Ableitung aus GSW-C überein. Das prüft Übernahme
/// und AutoDiff gegeneinander.
#[test]
fn autodiff_matches_handwritten_gsw_derivative() {
    for (sa, t_c, p_dbar) in [
        (35.16504, 0.0, 0.0),
        (10.0, 25.0, 500.0),
        (40.0, -1.5, 4000.0),
        (0.5, 35.0, 100.0),
    ] {
        let t = T0 + t_c;
        let p = P0 + p_dbar * 1e4;
        let ad = saline_ds(sa, t, p);
        let gsw = saline_ds_gsw(sa, t_c, p_dbar);
        let rel = ((ad - gsw) / gsw).abs();
        assert!(
            rel < 1e-12,
            "S={sa}, t={t_c}°C, p={p_dbar} dbar: AD {ad}, GSW {gsw}"
        );
    }
}

/// Referenzzustand von TEOS-10: Standardozean (35.16504 g/kg, 0 °C,
/// 101325 Pa) hat h = 0 und s = 0, also auch g = 0.
#[test]
fn standard_ocean_reference_state() {
    let sw = seawater(SSO, T0, P0).unwrap();
    assert!(sw.g.abs() < 1e-5, "g = {}", sw.g);
    assert!(sw.h.abs() < 1e-5, "h = {}", sw.h);
    assert!(sw.s.abs() < 1e-7, "s = {}", sw.s);
    // Dichte und Wärmekapazität des Standardozeans
    assert!((sw.rho - 1028.1072).abs() < 1e-4, "ρ = {}", sw.rho);
    assert!((sw.cp - 3986.48579).abs() < 1e-3, "cp = {}", sw.cp);
}

/// Salz senkt den Gefrierpunkt und hebt den Siedepunkt.
#[test]
fn freezing_and_boiling() {
    let tf = freezing_temperature(SSO, P0).unwrap() - T0;
    assert!((-1.95..-1.88).contains(&tf), "Gefrierpunkt {tf} °C");
    let tb = boiling_temperature(SSO, P0).unwrap();
    let pure = iapws::iapws95::saturation_temperature(P0 / 1e6).unwrap();
    let elevation = tb - pure;
    assert!(
        (0.4..0.7).contains(&elevation),
        "Siedepunkterhöhung {elevation} K"
    );
}
