//! Prüfwerte aus der IAPWS-95-Veröffentlichung (Revision 2018).

use iapws::iapws95::*;

/// Die Tabellenwerte haben 9 signifikante Stellen.
fn assert_close(name: &str, got: f64, want: f64) {
    let rel = ((got - want) / want).abs();
    assert!(
        rel < 1e-8,
        "{name}: berechnet {got:.9e}, Tabelle {want:.9e}, rel. Fehler {rel:.1e}"
    );
}

/// Tabelle 6: φ° und φʳ mit Ableitungen bei T = 500 K, ρ = 838.025 kg/m³.
#[test]
fn table_6_derivatives() {
    let delta = 838.025 / RHOC;
    let tau = TC / 500.0;
    let o = ideal(delta, tau);
    assert_close("φ°", o.f, 0.204_797_733e1);
    assert_close("φ°_δ", o.fx, 0.384_236_747);
    assert_close("φ°_δδ", o.fxx, -0.147_637_878);
    assert_close("φ°_τ", o.fy, 0.904_611_106e1);
    assert_close("φ°_ττ", o.fyy, -0.193_249_185e1);
    assert_eq!(o.fxy, 0.0);

    let r = residual(delta, tau);
    assert_close("φʳ", r.f, -0.342_693_206e1);
    assert_close("φʳ_δ", r.fx, -0.364_366_650);
    assert_close("φʳ_δδ", r.fxx, 0.856_063_701);
    assert_close("φʳ_τ", r.fy, -0.581_403_435e1);
    assert_close("φʳ_ττ", r.fyy, -0.223_440_737e1);
    assert_close("φʳ_δτ", r.fxy, -0.112_176_915e1);
}

/// Tabelle 7: Einphasengebiet. Spalten: T, ρ, p, cv, w, s.
#[test]
fn table_7_single_phase() {
    #[rustfmt::skip]
    let rows: [(f64, f64, f64, f64, f64, f64); 11] = [
        (300.0, 0.996_556_0e3, 0.992_418_352e-1, 0.413_018_112e1, 0.150_151_914e4, 0.393_062_643),
        (300.0, 0.100_530_8e4, 0.200_022_515e2, 0.406_798_347e1, 0.153_492_501e4, 0.387_405_401),
        (300.0, 0.118_820_2e4, 0.700_004_704e3, 0.346_135_580e1, 0.244_357_992e4, 0.132_609_616),
        (500.0, 0.435e0, 0.999_679_423e-1, 0.150_817_541e1, 0.548_314_253e3, 0.794_488_271e1),
        (500.0, 0.453_2e1, 0.999_938_125, 0.166_991_025e1, 0.535_739_001e3, 0.682_502_725e1),
        (500.0, 0.838_025e3, 0.100_003_858e2, 0.322_106_219e1, 0.127_128_441e4, 0.256_690_918e1),
        (500.0, 0.108_456_4e4, 0.700_000_405e3, 0.307_437_693e1, 0.241_200_877e4, 0.203_237_509e1),
        (647.0, 0.358e3, 0.220_384_756e2, 0.618_315_728e1, 0.252_145_078e3, 0.432_092_307e1),
        (900.0, 0.241e0, 0.100_062_559, 0.175_890_657e1, 0.724_027_147e3, 0.916_653_194e1),
        (900.0, 0.526_15e2, 0.200_000_690e2, 0.193_510_526e1, 0.698_445_674e3, 0.659_070_225e1),
        (900.0, 0.870_769e3, 0.700_000_006e3, 0.266_422_350e1, 0.201_933_608e4, 0.417_223_802e1),
    ];
    for (t, rho, p, cv, w, s) in rows {
        let st = state(t, rho);
        let at = format!("T = {t} K, ρ = {rho} kg/m³");
        assert_close(&format!("p bei {at}"), st.p, p);
        assert_close(&format!("cv bei {at}"), st.cv, cv);
        assert_close(&format!("w bei {at}"), st.w, w);
        assert_close(&format!("s bei {at}"), st.s, s);
    }
}

/// Umkehrrechnung: Dichte aus T und p, dann zurück zum Druck.
#[test]
fn density_from_pressure() {
    for (t, p, guess) in [
        (300.0, 0.1, 1000.0),
        (500.0, 10.0, 800.0),
        (900.0, 0.1, 0.2),
    ] {
        let rho = density(t, p, guess).expect("Newton konvergiert");
        assert_close(&format!("p(ρ(T={t}, p={p}))"), state(t, rho).p, p);
    }
    // Wasser bei 300 K und 0.1 MPa hat ~996.5 kg/m³
    let rho = density(300.0, 0.0992418352, 1000.0).unwrap();
    assert!((rho - 996.556).abs() < 1e-3, "{rho}");
}

/// Plausibilität: cp > cv und h = u + p/ρ.
#[test]
fn consistency() {
    let st = state(400.0, 940.0);
    assert!(st.cp > st.cv);
    let h_from_u = st.u + st.p * 1000.0 / st.rho;
    assert!((st.h - h_from_u).abs() < 1e-9 * st.h.abs());
}

/// Tabelle 8: Zweiphasengebiet. Spalten: T, p, ρ', ρ'', h', h'', s', s''.
#[test]
fn table_8_saturation() {
    #[rustfmt::skip]
    let rows: [(f64, [f64; 7]); 3] = [
        (275.0, [0.698_451_167e-3, 0.999_887_406e3, 0.550_664_919e-2, 0.775_972_202e1, 0.250_428_995e4, 0.283_094_670e-1, 0.910_660_121e1]),
        (450.0, [0.932_203_564, 0.890_341_250e3, 0.481_200_360e1, 0.749_161_585e3, 0.277_441_078e4, 0.210_865_845e1, 0.660_921_221e1]),
        (625.0, [0.169_082_693e2, 0.567_090_385e3, 0.118_290_280e3, 0.168_626_976e4, 0.255_071_625e4, 0.380_194_683e1, 0.518_506_121e1]),
    ];
    for (t, [p, rl, rv, hl, hv, sl, sv]) in rows {
        let s = saturation(t).expect("Sättigung konvergiert");
        let at = format!("T = {t} K");
        assert_close(&format!("p bei {at}"), s.p, p);
        assert_close(&format!("ρ' bei {at}"), s.liquid.rho, rl);
        assert_close(&format!("ρ'' bei {at}"), s.vapor.rho, rv);
        assert_close(&format!("h' bei {at}"), s.liquid.h, hl);
        assert_close(&format!("h'' bei {at}"), s.vapor.h, hv);
        assert_close(&format!("s' bei {at}"), s.liquid.s, sl);
        assert_close(&format!("s'' bei {at}"), s.vapor.s, sv);
    }
}

/// Siedepunkt bei Normaldruck (101.325 kPa): 373.124 K.
#[test]
fn boiling_point() {
    let t = saturation_temperature(0.101_325).expect("Siedetemperatur");
    assert!((t - 373.124).abs() < 1e-3, "{t}");
    assert_eq!(saturation(TC + 1.0), None);
}
