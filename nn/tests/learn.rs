use nn::{Mlp, Rng, xor_data};

#[test]
fn gradient_matches_finite_differences() {
    let mut rng = Rng::new(7);
    let net = Mlp::new(&[2, 3, 1], &mut rng);
    let data = xor_data();
    let (_, grad) = net.loss_and_gradient(&data, 0.0, &mut rng);
    let h = 1e-6;
    for (k, &g) in grad.iter().enumerate() {
        let mut plus = net.clone();
        plus.params[k] += h;
        let mut minus = net.clone();
        minus.params[k] -= h;
        let lp = plus.loss_and_gradient(&data, 0.0, &mut rng).0;
        let lm = minus.loss_and_gradient(&data, 0.0, &mut rng).0;
        let fd = (lp - lm) / (2.0 * h);
        assert!(
            (fd - grad[k]).abs() < 1e-7,
            "param {k}: {fd} vs {}",
            grad[k]
        );
    }
}

#[test]
fn xor_is_learned() {
    let mut rng = Rng::new(42);
    let mut net = Mlp::new(&[2, 4, 1], &mut rng);
    let data = xor_data();
    for _ in 0..5000 {
        net.train_step(&data, 0.5, 0.0, &mut rng);
    }
    for (x, t) in &data {
        let y = net.predict(x)[0];
        assert!((y - t[0]).abs() < 0.1, "{x:?} -> {y}");
    }
}
