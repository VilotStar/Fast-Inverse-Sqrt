pub fn fast_inv_sqrt(x: f64) -> f64 {
    let mut bits: u64 = x.to_bits();

    bits = 0x5f3759df - (bits >> 1);

    let y: f64 = f64::from_bits(bits);

    y * (1.5 - ((0.5 * x) * y * y))
}

pub fn normal_inv_sqrt(x: f64) -> f64 {
    1.0 / x.sqrt()
}