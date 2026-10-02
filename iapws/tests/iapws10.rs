//! Prüfwerte aus IAPWS G8-10 (feuchte Luft) und G9-12 (Virialkoeffizienten).

use iapws::iapws10::*;

fn assert_close(name: &str, got: f64, want: f64) {
    let rel = ((got - want) / want).abs();
    assert!(
        rel < 1e-8,
        "{name}: berechnet {got:.10e}, Tabelle {want:.10e}, rel. Fehler {rel:.1e}"
    );
}

/// Reihenfolge der Ableitungen wie in den Tabellen:
/// f, f_T, f_ρ, f_TT, f_ρT, f_ρρ
fn check_tr(name: &str, d: math_traits::autodiff::Partials2<f64>, want: [f64; 6]) {
    let got = [d.f, d.fx, d.fy, d.fxx, d.fxy, d.fyy];
    let labels = ["f", "f_T", "f_ρ", "f_TT", "f_ρT", "f_ρρ"];
    for ((g, w), l) in got.iter().zip(want).zip(labels) {
        assert_close(&format!("{name} {l}"), *g, w);
    }
}

/// (A, T, ρ) der drei Prüfzustände
const STATES: [(f64, f64, f64); 3] = [
    (0.892_247_719, 200.0, 1.634_796_57e-5),
    (0.977_605_798, 300.0, 1.146_142_16),
    (0.825_565_291, 400.0, 0.793_354_063e1),
];

/// Tabelle 14: trockene Luft und Wasserdampf getrennt.
#[test]
fn table_14_components() {
    #[rustfmt::skip]
    let air = [
        [-0.740_041_144e3, -0.304_774_177e1, 0.393_583_654e7, -0.357_677_878e-2, 0.196_791_837e5, -0.269_828_549e12],
        [-0.916_103_453e2, -0.108_476_220, 0.768_326_795e2, -0.239_319_940e-2, 0.256_683_306, -0.685_917_373e2],
        [0.895_561_286e2, 0.193_271_394, 0.175_560_114e2, -0.181_809_877e-2, 0.442_769_673e-1, -0.267_635_928e1],
    ];
    #[rustfmt::skip]
    let water = [
        [-0.202_254_351e3, -0.123_787_544e2, 0.523_995_674e8, -0.694_877_601e-2, 0.262_001_885e6, -0.297_466_671e14],
        [-0.143_157_426e3, -0.851_598_213e1, 0.538_480_619e4, -0.480_817_011e-2, 0.181_489_502e2, -0.210_184_992e6],
        [-0.285_137_534e3, -0.705_288_048e1, 0.129_645_039e3, -0.411_710_659e-2, 0.361_784_086, -0.965_539_462e2],
    ];
    for (i, (a, t, rho)) in STATES.into_iter().enumerate() {
        check_tr(&format!("Luft T={t}"), air_derivatives(t, a * rho), air[i]);
        check_tr(
            &format!("Dampf T={t}"),
            water_derivatives(t, (1.0 - a) * rho),
            water[i],
        );
    }
}

/// Tabelle 7: Kreuz-Virialkoeffizienten.
#[test]
fn table_7_virial() {
    for (t, baw, caaw, caww) in [
        (
            200.0,
            -0.784_874_278e-4,
            0.105_493_575e-8,
            -0.349_872_634e-5,
        ),
        (
            300.0,
            -0.295_672_747e-4,
            0.801_977_741e-9,
            -0.115_552_784e-6,
        ),
        (
            400.0,
            -0.100_804_610e-4,
            0.672_018_172e-9,
            -0.200_806_021e-7,
        ),
    ] {
        let (b, c1, c2) = cross_virial(math_traits::num::Num(t));
        assert_close(&format!("B_aw({t})"), b.0, baw);
        assert_close(&format!("C_aaw({t})"), c1.0, caaw);
        assert_close(&format!("C_aww({t})"), c2.0, caww);
    }
}

/// Tabelle 15: Wechselwirkungsanteil bei T = 200 K.
#[test]
fn table_15_mix() {
    let (a, t, rho) = STATES[0];
    let m = mix_derivatives(a, t, rho);
    assert_close("f_mix", m.f, -0.786_231_899e-6);
    assert_close("f_mix,A", m.grad[0], 0.641_550_398e-5);
    assert_close("f_mix,T", m.grad[1], 0.456_438_658e-8);
    assert_close("f_mix,ρ", m.grad[2], -0.480_937_188e-1);
    assert_close("f_mix,AA", m.hess[0][0], 0.163_552_956e-4);
    assert_close("f_mix,AT", m.hess[0][1], -0.372_455_576e-7);
    assert_close("f_mix,Aρ", m.hess[0][2], 0.392_437_132);
    assert_close("f_mix,TT", m.hess[1][1], -0.378_875_706e-10);
    assert_close("f_mix,Tρ", m.hess[1][2], 0.279_209_778e-3);
    assert_close("f_mix,ρρ", m.hess[2][2], -0.192_042_557e-1);
}

/// Tabelle 13: feuchte Luft, alle Ableitungen und Zustandsgrößen.
#[test]
fn table_13_humid_air() {
    // f, f_A, f_T, f_ρ, f_AA, f_AT, f_Aρ, f_TT, f_Tρ, f_ρρ
    #[rustfmt::skip]
    let derivs = [
        [-0.682_093_392e3, -0.572_680_404e3, -0.405_317_966e1, 0.374_173_101e7, 0.920_967_684e3,
         0.915_653_743e1, -0.213_442_099e7, -0.394_011_921e-2, 0.187_087_034e5, -0.228_880_603e12],
        [-0.927_718_178e2, -0.263_453_864, -0.296_711_481, 0.761_242_496e2, 0.624_886_233e4,
         0.822_733_446e1, -0.450_004_399e2, -0.244_742_952e-2, 0.254_456_302, -0.664_465_525e2],
        [0.240_345_570e2, 0.311_096_733e3, -0.106_891_931e1, 0.158_878_781e2, 0.113_786_423e4,
         0.702_631_471e1, -0.727_972_651e1, -0.222_449_294e-2, 0.414_350_772e-1, -0.201_886_184e1],
    ];
    // p, h, g, s, cp, w, μ_W
    #[rustfmt::skip]
    let props = [
        [0.999_999_998e-6, 0.189_712_231e3, -0.620_923_701e3, 0.405_317_966e1, 0.109_387_397e1, 0.291_394_959e3, -0.109_950_917e3],
        [0.1, 0.834_908_383e2, -0.552_260_595e1, 0.296_711_481, 0.102_681_324e1, 0.349_234_196e3, -0.526_505_193e1],
        [1.0, 0.577_649_408e3, 0.150_081_684e3, 0.106_891_931e1, 0.123_552_454e1, 0.416_656_820e3, -0.106_748_981e3],
    ];
    for (i, (a, t, rho)) in STATES.into_iter().enumerate() {
        let h = humid_derivatives(a, t, rho);
        let got = [
            h.f,
            h.grad[0],
            h.grad[1],
            h.grad[2],
            h.hess[0][0],
            h.hess[0][1],
            h.hess[0][2],
            h.hess[1][1],
            h.hess[1][2],
            h.hess[2][2],
        ];
        let labels = [
            "f", "f_A", "f_T", "f_ρ", "f_AA", "f_AT", "f_Aρ", "f_TT", "f_Tρ", "f_ρρ",
        ];
        for ((g, w), l) in got.iter().zip(derivs[i]).zip(labels) {
            assert_close(&format!("T={t} {l}"), *g, w);
        }
        let s = humid_air(a, t, rho);
        let got = [s.p, s.h, s.g, s.s, s.cp, s.w, s.mu_water];
        let labels = ["p", "h", "g", "s", "cp", "w", "μ_W"];
        for ((g, w), l) in got.iter().zip(props[i]).zip(labels) {
            assert_close(&format!("T={t} {l}"), *g, w);
        }
    }
}

/// Referenzzustand von IAPWS-10: trockene Luft bei 273.15 K und 101325 Pa
/// hat h = 0 und s = 0. Dazu Werte aus Lemmon et al. (2000), Tabelle A2
/// (dort nur 4–5 Stellen angegeben).
#[test]
fn dry_air_reference_and_lemmon_table() {
    let rho = air_density(273.15, 0.101_325).unwrap();
    let st = dry_air(273.15, rho);
    assert!(
        st.h.abs() < 1e-8 && st.s.abs() < 1e-10,
        "h = {}, s = {}",
        st.h,
        st.s
    );

    // (T, p, ρ in mol/dm³, cv, cp in J/(mol·K), w in m/s)
    for (t, p, rho_m, cv, cp, w) in [
        (100.0, 0.101_325, 0.124_49, 21.09, 30.13, 198.2),
        (500.0, 0.2, 0.048_077, 21.51, 29.84, 446.6),
        (2000.0, 10.0, 0.590_94, 27.93, 36.25, 878.5),
    ] {
        let rho = air_density(t, p).unwrap();
        let st = dry_air(t, rho);
        let m = 28.965_46; // g/mol
        assert!((rho / m - rho_m).abs() / rho_m < 1e-4, "ρ bei {t} K");
        assert!(
            (st.cv * m - cv).abs() < 0.006,
            "cv bei {t} K: {}",
            st.cv * m
        );
        assert!(
            (st.cp * m - cp).abs() < 0.006,
            "cp bei {t} K: {}",
            st.cp * m
        );
        assert!((st.w - w).abs() < 0.06, "w bei {t} K: {}", st.w);
    }
}
