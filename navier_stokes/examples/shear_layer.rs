//! Doppelte Scherschicht: zwei gegenläufige Strahlen rollen sich zu Wirbeln
//! auf (Kelvin-Helmholtz). Schreibt eine 2×2-Bildfolge der Wirbelstärke.
//!
//! `cargo run -p navier_stokes --example shear_layer --release -- [Verzeichnis]`

use navier_stokes::*;

const N: usize = 128;

/// Blau (negativ) – Weiß – Rot (positiv)
fn color(v: f64, vmax: f64) -> [u8; 3] {
    let s = (v / vmax).clamp(-1.0, 1.0);
    let (a, b) = if s >= 0.0 {
        ([255.0, 255.0, 255.0], [178.0, 24.0, 43.0])
    } else {
        ([255.0, 255.0, 255.0], [33.0, 102.0, 172.0])
    };
    let t = s.abs();
    [0, 1, 2].map(|k| (a[k] * (1.0 - t) + b[k] * t) as u8)
}

fn main() -> std::io::Result<()> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let thickness = std::f64::consts::PI / 15.0;
    // ν so gewählt, dass 128² die dünnsten Filamente noch auflöst (kleineres ν
    // braucht ein feineres Gitter, sonst entsteht Gitterrauschen).
    let (dt, nu) = (0.005, 6e-4);
    let mut w: Vorticity<N> = double_shear_layer(thickness, 0.05);
    let vmax = 0.6 / thickness;

    let times = [0.0, 4.0, 6.0, 8.0];
    let mut frames = Vec::new();
    let mut t = 0.0;
    for &target in &times {
        while t < target - 1e-9 {
            w = step(&w, dt, nu);
            t += dt;
        }
        println!(
            "t = {t:4.1}: Energie {:.6}, Enstrophie {:.3}",
            energy(&w),
            enstrophy(&w)
        );
        frames.push(w);
    }

    // 2×2-Raster mit 8 px Rand; y nach oben
    let (cell, gap) = (N * 3, 8);
    let size = 2 * cell + 3 * gap;
    let mut img = vec![[255u8; 3]; size * size];
    for (f, frame) in frames.iter().enumerate() {
        let (ox, oy) = (gap + (f % 2) * (cell + gap), gap + (f / 2) * (cell + gap));
        for py in 0..cell {
            for px in 0..cell {
                let (i, j) = (N - 1 - py / 3, px / 3);
                img[(oy + py) * size + ox + px] = color(frame.0[i][j], vmax);
            }
        }
    }
    let path = format!("{dir}/scherschicht.png");
    let file = std::io::BufWriter::new(std::fs::File::create(&path)?);
    let mut enc = png::Encoder::new(file, size as u32, size as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(img.as_flattened())?;
    println!("geschrieben: {path}");
    Ok(())
}
