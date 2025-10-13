use rand::Rng;
use rand::rngs::SmallRng;

pub fn exponential(rng: &mut SmallRng, mean: f64) -> f64 {
    let u: f64 = rng.random();
    -mean * u.ln()
}

pub fn normal(rng: &mut SmallRng, mean: f64, std_dev: f64) -> f64 {
    let u1: f64 = rng.random();
    let u2: f64 = rng.random();
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    mean + std_dev * z0
}

pub fn uniform(rng: &mut SmallRng, a: f64, b: f64) -> f64 {
    let u: f64 = rng.random();
    a + (b - a) * u
}

pub fn erlang(rng: &mut SmallRng, mean: f64, k: usize) -> f64 {
    let mut product = 1.0;
    for _ in 0..k {
        let u: f64 = rng.random();
        product *= u;
    }
    -mean / k as f64 * product.ln()
}
