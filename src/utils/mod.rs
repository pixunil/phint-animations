mod serialize;
pub mod deserialize;

pub fn lerp(start: f64, target: f64, t: f64) -> f64 {
    start + t * (target - start)
}
