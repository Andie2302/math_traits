//! Kleine neuronale Netze zum Verstehen (Lektion 3).
//!
//! * [`Mlp`]: Schichten aus Neuronen, `y = σ(W·x + b)`, die letzte Schicht
//!   ohne Aktivierung.
//! * Lernen per Gradientenabstieg: Fehler messen, mit Rückwärts-AutoDiff
//!   ([`math_traits::reverse`]) alle Steigungen auf einmal berechnen, jedes
//!   Gewicht ein Stück bergab schieben.
//! * Rauschen auf den Eingängen beim Training (robuster) und Streuung der
//!   Ausgaben bei verrauschten Eingaben (Unsicherheit).

use math_traits::reverse::{Tape, Var};

/// Ein kleiner, reproduzierbarer Zufallsgenerator (xorshift64*).
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    /// Gleichverteilt in `[0, 1)`.
    pub fn uniform(&mut self) -> f64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Normalverteilt mit Mittelwert 0 und Streuung 1 (Box-Muller).
    pub fn gaussian(&mut self) -> f64 {
        let u = self.uniform().max(1e-300);
        let v = self.uniform();
        (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
    }
}

/// Mehrschichtiges Netz („Multi-Layer Perceptron“).
#[derive(Clone, Debug)]
pub struct Mlp {
    /// Neuronen pro Schicht, inklusive Eingang, z. B. `[2, 4, 1]`.
    pub sizes: Vec<usize>,
    /// Alle Gewichte und Bias-Werte hintereinander.
    pub params: Vec<f64>,
}

impl Mlp {
    /// Zufällige Startgewichte, gleichverteilt in `±1/sqrt(Eingänge)`.
    pub fn new(sizes: &[usize], rng: &mut Rng) -> Self {
        let mut params = Vec::new();
        for w in sizes.windows(2) {
            let scale = 1.0 / (w[0] as f64).sqrt();
            for _ in 0..(w[0] + 1) * w[1] {
                params.push((2.0 * rng.uniform() - 1.0) * scale);
            }
        }
        Self {
            sizes: sizes.to_vec(),
            params,
        }
    }

    /// Vorwärtsrechnung auf dem Band: versteckte Schichten mit Sigmoid,
    /// die letzte linear.
    fn forward<'t>(&self, p: &[Var<'t, f64>], x: &[Var<'t, f64>]) -> Vec<Var<'t, f64>> {
        let mut act = x.to_vec();
        let mut k = 0;
        let layers = self.sizes.len() - 1;
        for (l, w) in self.sizes.windows(2).enumerate() {
            let (n_in, n_out) = (w[0], w[1]);
            act = (0..n_out)
                .map(|_| {
                    // Summe = Σ Gewicht·Eingang + Bias
                    let mut sum = p[k + n_in];
                    for (i, a) in act.iter().enumerate() {
                        sum = sum + p[k + i] * *a;
                    }
                    k += n_in + 1;
                    if l + 1 < layers { sum.sigmoid() } else { sum }
                })
                .collect();
        }
        act
    }

    /// Ausgabe zur Eingabe `x`.
    pub fn predict(&self, x: &[f64]) -> Vec<f64> {
        let tape = Tape::new();
        let p: Vec<_> = self.params.iter().map(|&v| tape.constant(v)).collect();
        let x: Vec<_> = x.iter().map(|&v| tape.constant(v)).collect();
        self.forward(&p, &x).iter().map(|v| v.value()).collect()
    }

    /// Mittlerer quadratischer Fehler über die Beispiele und alle Steigungen.
    /// `noise` ist die Streuung des Rauschens auf den Eingängen (0 = ohne).
    pub fn loss_and_gradient(
        &self,
        data: &[(Vec<f64>, Vec<f64>)],
        noise: f64,
        rng: &mut Rng,
    ) -> (f64, Vec<f64>) {
        let tape = Tape::new();
        let p: Vec<_> = self.params.iter().map(|&v| tape.var(v)).collect();
        let mut loss = tape.constant(0.0);
        for (x, t) in data {
            let x: Vec<_> = x
                .iter()
                .map(|&v| tape.constant(v + noise * rng.gaussian()))
                .collect();
            for (y, &t) in self.forward(&p, &x).into_iter().zip(t) {
                loss = loss + (y - tape.constant(t)).square();
            }
        }
        let loss = loss * tape.constant(1.0 / data.len() as f64);
        let grad = tape.gradient(loss);
        (loss.value(), p.iter().map(|v| grad[v.index()]).collect())
    }

    /// Ein Lernschritt: jedes Gewicht `w ← w − η · Steigung`. Gibt den Fehler
    /// vor dem Schritt zurück.
    pub fn train_step(
        &mut self,
        data: &[(Vec<f64>, Vec<f64>)],
        lr: f64,
        noise: f64,
        rng: &mut Rng,
    ) -> f64 {
        let (loss, grad) = self.loss_and_gradient(data, noise, rng);
        for (w, g) in self.params.iter_mut().zip(grad) {
            *w -= lr * g;
        }
        loss
    }

    /// Mittelwert und Streuung der (ersten) Ausgabe, wenn man `x` `samples`-mal
    /// mit Rauschen der Stärke `noise` durchrechnet.
    pub fn spread(&self, x: &[f64], noise: f64, samples: usize, rng: &mut Rng) -> (f64, f64) {
        let ys: Vec<f64> = (0..samples)
            .map(|_| {
                let xn: Vec<f64> = x.iter().map(|v| v + noise * rng.gaussian()).collect();
                self.predict(&xn)[0]
            })
            .collect();
        let mean = ys.iter().sum::<f64>() / samples as f64;
        let var = ys.iter().map(|y| (y - mean).powi(2)).sum::<f64>() / samples as f64;
        (mean, var.sqrt())
    }
}

/// Die vier XOR-Beispiele.
pub fn xor_data() -> Vec<(Vec<f64>, Vec<f64>)> {
    vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ]
}
