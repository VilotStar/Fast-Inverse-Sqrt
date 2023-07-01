pub fn fast_inv_sqrt_32(x: f32) -> f32 {
    let mut bits: u32 = x.to_bits();

    bits = 0x5f3759df - (bits >> 1);

    let mut y: f32 = f32::from_bits(bits);

    y = y * (1.5 - ((0.5 * x) * y * y));
    y * (1.5 - ((0.5 * x) * y * y))
}

pub fn fast_inv_sqrt_64(x: f64) -> f64 {
    let mut bits: u64 = x.to_bits();

    bits = 0x5fe6ec85e7de30da - (bits >> 1);

    let mut y: f64 = f64::from_bits(bits);

    y = y * (1.5 - ((0.5 * x) * y * y));
    y * (1.5 - ((0.5 * x) * y * y))
}

pub fn internal_inv_sqrt(x: f64) -> f64 {
    x.sqrt().recip()
}

pub fn normal_inv_sqrt(x: f64) -> f64 {
    1f64 / x.sqrt()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn inv_sqrt() {
        let fast = fast_inv_sqrt_64(36.0);
        let normal = normal_inv_sqrt(36.0);
        println!("{fast} {normal}")
    }
}
