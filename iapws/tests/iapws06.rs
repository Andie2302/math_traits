//! Prüfwerte aus der IAPWS-06-Veröffentlichung (Revision 2009), Tabelle 6.

use iapws::iapws06::*;

fn assert_close(name: &str, got: f64, want: f64) {
    let rel = ((got - want) / want).abs();
    assert!(
        rel < 1e-8,
        "{name}: berechnet {got:.11e}, Tabelle {want:.11e}, rel. Fehler {rel:.1e}"
    );
}

/// Spalten: T, p, g, ∂g/∂p, ∂g/∂T, ∂²g/∂p², ∂²g/∂T∂p, ∂²g/∂T², h, ρ, cp
#[test]
fn table_6() {
    #[rustfmt::skip]
    let rows: [(f64, f64, [f64; 9]); 3] = [
        (273.16, 611.657, [
            0.611_784_135, 0.109_085_812_737e-2, 0.122_069_433_940e4,
            -0.128_495_941_571e-12, 0.174_387_964_700e-6, -0.767_602_985_875e1,
            -0.333_444_253_966e6, 0.916_709_492_200e3, 0.209_678_431_622e4,
        ]),
        (273.152_519, 101_325.0, [
            0.101_342_740_690e3, 0.109_084_388_214e-2, 0.122_076_932_550e4,
            -0.128_485_364_928e-12, 0.174_362_219_972e-6, -0.767_598_233_365e1,
            -0.333_354_873_637e6, 0.916_721_463_419e3, 0.209_671_391_024e4,
        ]),
        (100.0, 100e6, [
            -0.222_296_513_088e6, 0.106_193_389_260e-2, 0.261_195_122_589e4,
            -0.941_807_981_761e-13, 0.274_505_162_488e-7, -0.866_333_195_517e1,
            -0.483_491_635_676e6, 0.941_678_203_297e3, 0.866_333_195_517e3,
        ]),
    ];
    for (t, p, [g, gp, gt, gpp, gtp, gtt, h, rho, cp]) in rows {
        let d = gibbs_derivatives(t, p);
        let s = ice(t, p);
        let at = format!("T = {t} K, p = {p} Pa");
        assert_close(&format!("g bei {at}"), d.f, g);
        assert_close(&format!("g_p bei {at}"), d.fy, gp);
        assert_close(&format!("g_T bei {at}"), d.fx, gt);
        assert_close(&format!("g_pp bei {at}"), d.fyy, gpp);
        assert_close(&format!("g_Tp bei {at}"), d.fxy, gtp);
        assert_close(&format!("g_TT bei {at}"), d.fxx, gtt);
        assert_close(&format!("h bei {at}"), s.h, h);
        assert_close(&format!("ρ bei {at}"), s.rho, rho);
        assert_close(&format!("cp bei {at}"), s.cp, cp);
    }
}

/// Zwei unabhängige Gleichungen, ein gemeinsamer Punkt: Bei Normaldruck
/// schmilzt Eis bei 273.152519 K.
#[test]
fn melting_point_from_two_equations() {
    let t = melting_temperature(P0).expect("Schmelztemperatur");
    assert!((t - 273.152_519).abs() < 1e-5, "{t}");
    // Am Tripelpunkt liegt sie bei 273.16 K
    let t_triple = melting_temperature(PT).expect("Tripelpunkt");
    assert!((t_triple - TT).abs() < 1e-4, "{t_triple}");
}
