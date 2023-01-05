use crate::types::Double;

pub fn diff_sq(first: Double, second: Double) -> Double {
    (first - second).powi(2)
}
