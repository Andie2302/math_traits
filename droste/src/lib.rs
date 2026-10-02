//! Droste-Effekt nach Escher und Lenstra: der „Logarithmus eines Bildes“.
//!
//! Ausgangspunkt ist ein **selbstähnliches** Bild: Es enthält sich selbst,
//! verkleinert um den Faktor `s`. Im komplexen Logarithmus `w = ln z` wird
//! daraus ein Gitter mit den Perioden `ln s` (verkleinern) und `2πi` (einmal
//! herum). Die Gerade von einem Punkt zur gleichen Stelle in der nächsten
//! Kopie ist `ln s + 2πi`. Wird sie auf `2πi` gedreht und gestreckt,
//!
//! ```text
//! w' = w · γ,   γ = 1 + ln s / (2πi),
//! ```
//!
//! dann führt eine Umdrehung im Ergebnis genau eine Ebene tiefer ins Bild:
//! die Spirale aus Eschers „Bildgalerie“. Zurück geht es mit `z' = exp(w')`.
//!
//! Die gesamte Rechnung läuft über `Num<Complex<f64>>` aus `math_traits`.

use math_traits::impls::cayley_dickson::Complex;
use math_traits::num::Num;

type C = Num<Complex<f64>>;

fn c(re: f64, im: f64) -> C {
    Num(Complex::new(re, im))
}

/// RGB-Bild, zeilenweise von oben nach unten.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[u8; 3]>,
}

impl Image {
    /// Erzeugt ein Bild aus einer Funktion der Bildkoordinaten.
    /// `z` läuft über `[-1, 1]` in der kürzeren Richtung, mit `y` nach oben.
    pub fn from_fn(width: usize, height: usize, mut f: impl FnMut(C) -> [u8; 3]) -> Self {
        let half = width.min(height) as f64 / 2.0;
        let pixels = (0..height)
            .flat_map(|row| (0..width).map(move |col| (row, col)))
            .map(|(row, col)| {
                let x = (col as f64 + 0.5 - width as f64 / 2.0) / half;
                let y = (height as f64 / 2.0 - row as f64 - 0.5) / half;
                f(c(x, y))
            })
            .collect();
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Bilineare Abtastung an der Stelle `z` (gleiche Koordinaten wie
    /// [`Image::from_fn`]). Außerhalb des Bildes: Randfarbe.
    pub fn sample(&self, z: C) -> [u8; 3] {
        let half = self.width.min(self.height) as f64 / 2.0;
        let fx = z.0.re * half + self.width as f64 / 2.0 - 0.5;
        let fy = self.height as f64 / 2.0 - z.0.im * half - 0.5;
        let x0 = fx.floor().clamp(0.0, (self.width - 1) as f64) as usize;
        let y0 = fy.floor().clamp(0.0, (self.height - 1) as f64) as usize;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        let (tx, ty) = (
            (fx - x0 as f64).clamp(0.0, 1.0),
            (fy - y0 as f64).clamp(0.0, 1.0),
        );
        let px = |x: usize, y: usize| self.pixels[y * self.width + x];
        let mut out = [0u8; 3];
        for (k, o) in out.iter_mut().enumerate() {
            let top = px(x0, y0)[k] as f64 * (1.0 - tx) + px(x1, y0)[k] as f64 * tx;
            let bottom = px(x0, y1)[k] as f64 * (1.0 - tx) + px(x1, y1)[k] as f64 * tx;
            *o = (top * (1.0 - ty) + bottom * ty).round() as u8;
        }
        out
    }

    /// Speichert das Bild als PNG.
    pub fn save_png(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let file = std::io::BufWriter::new(std::fs::File::create(path)?);
        let mut enc = png::Encoder::new(file, self.width as u32, self.height as u32);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);
        let mut writer = enc.write_header().map_err(std::io::Error::other)?;
        writer
            .write_image_data(self.pixels.as_flattened())
            .map_err(std::io::Error::other)
    }
}

/// `max(|x|, |y|)`: Abstand vom Zentrum in Quadrat-Ringen, damit der ganze
/// rechteckige Rahmen und nicht nur ein Kreisring zum Bild gehört.
fn ring(z: C) -> f64 {
    z.0.re.abs().max(z.0.im.abs())
}

/// Bringt `z` per Multiplikation mit `s` bzw. `1/s` in den Grundring
/// `1/s ≤ ring(z) < 1`. Dort liegt das eigentliche Bild, alles andere sind
/// Kopien.
pub fn reduce(mut z: C, s: f64) -> C {
    if z.0.re == 0.0 && z.0.im == 0.0 {
        return z;
    }
    while ring(z) >= 1.0 {
        z = z / s;
    }
    while ring(z) < 1.0 / s {
        z = z * s;
    }
    z
}

/// Ein selbstähnliches Testbild: farbiges Schachbrett im Grundring mit
/// dunklem Rahmen innen und außen, unendlich oft ineinander geschachtelt.
pub fn test_pattern(size: usize, s: f64) -> Image {
    Image::from_fn(size, size, |z| {
        let z = reduce(z, s);
        let r = ring(z);
        // Rahmen am äußeren und inneren Rand des Rings
        if r > 0.94 || r < 1.0 / s * 1.06 {
            return [30, 30, 40];
        }
        let (x, y) = (z.0.re, z.0.im);
        let tile = ((x * 6.0).floor() + (y * 6.0).floor()) as i64;
        let hue = (y.atan2(x) / std::f64::consts::TAU + 0.5) * 6.0;
        let base = hsv(hue, 0.65, if tile.rem_euclid(2) == 0 { 0.95 } else { 0.7 });
        base.map(|v| (v * 255.0) as u8)
    })
}

fn hsv(h: f64, s: f64, v: f64) -> [f64; 3] {
    let i = h.floor() as i64;
    let f = h - i as f64;
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - s * f), v * (1.0 - s * (1.0 - f)));
    match i.rem_euclid(6) {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q],
    }
}

/// Die Droste-Abbildung eines Punktes: `z ↦ exp(γ · ln z)`, reduziert in den
/// Grundring. `None` für `z = 0` (dort liegt der unendliche Fluchtpunkt).
pub fn droste_map(z: C, s: f64, twist: bool) -> Option<C> {
    // γ = 1 + ln s / (2πi) = 1 − i·ln s / (2π)
    let gamma = if twist {
        c(1.0, -s.ln() / std::f64::consts::TAU)
    } else {
        c(1.0, 0.0)
    };
    z.try_ln().map(|w| reduce((w * gamma).exp(), s))
}

/// Der Droste-Effekt auf ein ganzes Bild.
///
/// * `s`: Selbstähnlichkeitsfaktor von `src` (Bild in sich, um `s` verkleinert)
/// * `zoom`: Vergrößerung des Ausschnitts (1 = ganzes Bild)
/// * `twist = false` ergibt nur die gerade Verschachtelung (zum Vergleich)
pub fn droste(src: &Image, width: usize, height: usize, s: f64, zoom: f64, twist: bool) -> Image {
    Image::from_fn(width, height, |z| {
        let z = z / zoom;
        src.sample(droste_map(z, s, twist).unwrap_or(z))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: C, b: C) -> bool {
        (a.0.re - b.0.re).abs() < 1e-9 && (a.0.im - b.0.im).abs() < 1e-9
    }

    /// Über der negativen reellen Achse springt `ln` um `2πi`. Dank γ wird
    /// daraus genau ein Faktor `s`, und `reduce` gleicht ihn aus: keine Naht.
    #[test]
    fn no_seam_at_branch_cut() {
        let s = 4.0;
        for r in [0.05, 0.3, 0.7, 0.99] {
            let above = droste_map(c(-r, 1e-13), s, true).unwrap();
            let below = droste_map(c(-r, -1e-13), s, true).unwrap();
            assert!(close(above, below), "r = {r}: {above:?} vs {below:?}");
        }
    }

    /// Selbstähnlichkeit: `z` und `z·s` landen auf derselben Stelle.
    #[test]
    fn invariant_under_scaling() {
        let s = 4.0;
        for z in [c(0.3, 0.1), c(-0.2, 0.45), c(0.01, -0.7)] {
            let a = droste_map(z, s, false).unwrap();
            let b = droste_map(z * s, s, false).unwrap();
            assert!(close(a, b));
            let z2 = reduce(z, s);
            assert!(ring(z2) >= 1.0 / s && ring(z2) < 1.0);
        }
    }
}
