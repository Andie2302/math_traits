//! Prüfung gegen exakte Lösung und Erhaltungsgrößen.

use navier_stokes::*;

const N: usize = 32;

/// Taylor-Green: Die exakte Lösung klingt mit `e^(−2νk²t)` ab.
#[test]
fn taylor_green_decays_exactly() {
    let (nu, dt, steps, k) = (0.1, 0.01, 50, 1.0);
    let w0: Vorticity<N> = taylor_green(k);
    let mut w = w0;
    for _ in 0..steps {
        w = step(&w, dt, nu);
    }
    let factor = (-2.0 * nu * k * k * dt * steps as f64).exp();
    let max_err =
        w.0.iter()
            .flatten()
            .zip(w0.0.iter().flatten())
            .map(|(a, b)| (a - b * factor).abs())
            .fold(0.0, f64::max);
    assert!(max_err < 1e-10, "max. Fehler {max_err}");
}

/// Geschwindigkeitsfeld des Taylor-Green-Wirbels: Aus `ψ = cos x·cos y` folgt
/// `u = ∂ψ/∂y = −cos x·sin y` und `v = −∂ψ/∂x = sin x·cos y`.
#[test]
fn velocity_from_streamfunction() {
    let w: Vorticity<N> = taylor_green(1.0);
    let (u, v) = velocity(&w);
    for i in 0..N {
        for j in 0..N {
            let (x, y) = (
                core::f64::consts::TAU * j as f64 / N as f64,
                core::f64::consts::TAU * i as f64 / N as f64,
            );
            assert!((u[i * N + j] + x.cos() * y.sin()).abs() < 1e-12);
            assert!((v[i * N + j] - x.sin() * y.cos()).abs() < 1e-12);
        }
    }
}

/// Ohne Viskosität bleiben Energie und Enstrophie erhalten (bis auf den
/// Zeitschrittfehler), der Mittelwert der Wirbelstärke exakt.
#[test]
fn inviscid_conservation() {
    let mut w: Vorticity<N> =
        from_fn(|x, y| (x + 0.3).sin() * (2.0 * y).cos() + 0.5 * (2.0 * x - y).cos());
    let (e0, z0) = (energy(&w), enstrophy(&w));
    for _ in 0..100 {
        w = step(&w, 0.005, 0.0);
    }
    assert!(((energy(&w) - e0) / e0).abs() < 1e-6, "Energie");
    assert!(((enstrophy(&w) - z0) / z0).abs() < 1e-4, "Enstrophie");
    assert!(mean(&w).abs() < 1e-12);
}

/// Mit Viskosität nimmt die Energie ab.
#[test]
fn viscosity_dissipates_energy() {
    let mut w: Vorticity<N> = double_shear_layer(core::f64::consts::PI / 15.0, 0.05);
    let e0 = energy(&w);
    for _ in 0..20 {
        w = step(&w, 0.005, 1e-3);
    }
    assert!(energy(&w) < e0);
}
