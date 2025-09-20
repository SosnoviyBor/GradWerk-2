pub fn exponential(mean: f64) -> f64 {
    let u: f64 = rand::random();
    -mean * u.ln()
}

pub fn normal(mean: f64, std_dev: f64) -> f64 {
    let u1: f64 = rand::random();
    let u2: f64 = rand::random();
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    mean + std_dev * z0
}

pub fn uniform(a: f64, b: f64) -> f64 {
    let u: f64 = rand::random();
    a + (b - a) * u
}

pub fn erlang(mean: f64, k: u32) -> f64 {
    let mut product = 1.0;
    for _ in 0..k {
        let u: f64 = rand::random();
        product *= u;
    }
    -mean / k as f64 * product.ln()
}