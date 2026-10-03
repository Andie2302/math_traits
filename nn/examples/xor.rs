//! Lektion 3: Ein Netz lernt XOR selbst.
use nn::{Mlp, Rng, xor_data};

fn train(lr: f64, noise: f64, steps: usize, report: bool) -> Mlp {
    let mut rng = Rng::new(42);
    let mut net = Mlp::new(&[2, 4, 1], &mut rng);
    let data = xor_data();
    for s in 0..=steps {
        let loss = net.train_step(&data, lr, noise, &mut rng);
        if report && [0, 10, 100, 500, 1000, 2000, 5000].contains(&s) {
            println!("  Schritt {s:>5}: Fehler {loss:.5}");
        }
    }
    net
}

fn main() {
    println!("== 1. Lernraten ==");
    for lr in [0.05, 0.5, 5.0] {
        println!("η = {lr}");
        train(lr, 0.0, 5000, true);
    }

    println!("\n== 2. Was hat das Netz gelernt? (η = 0.5) ==");
    let net = train(0.5, 0.0, 5000, false);
    for (x, t) in xor_data() {
        println!("  {x:?} -> {:.3}  (Ziel {})", net.predict(&x)[0], t[0]);
    }

    println!("\n== 3. Rauschen: ohne vs. mit (σ = 0.2) beim Training ==");
    let clean = train(0.5, 0.0, 5000, false);
    let noisy = train(0.5, 0.2, 5000, false);
    let mut rng = Rng::new(1);
    println!("  Testeingabe mit Rauschen 0.2: Mittel ± Streuung");
    println!(
        "  {:<12} {:>18} {:>18}",
        "Punkt", "ohne Rauschen", "mit Rauschen"
    );
    for x in [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [0.5, 0.5]] {
        let (m1, s1) = clean.spread(&x, 0.2, 2000, &mut rng);
        let (m2, s2) = noisy.spread(&x, 0.2, 2000, &mut rng);
        println!(
            "  {:<12} {m1:>9.3} ± {s1:.3}   {m2:>9.3} ± {s2:.3}",
            format!("{x:?}")
        );
    }
}
