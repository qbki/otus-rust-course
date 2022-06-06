#[cfg(test)]
mod eq_floats_tests;

pub fn eq_floats(a: f64, b: f64) -> bool {
    f64::abs(a - b) < f64::EPSILON
}
