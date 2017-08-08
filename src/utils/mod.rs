mod serialize;
pub mod deserialize;

pub trait Lerp {
    fn lerp(&self, target: &Self, t: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(&self, target: &f64, t: f64) -> f64 {
        self + t * (target - self)
    }
}

impl Lerp for (f64, f64, f64) {
    fn lerp(&self, target: &(f64, f64, f64), t: f64) -> (f64, f64, f64) {
        (self.0.lerp(&target.0, t), self.1.lerp(&target.1, t),
            self.2.lerp(&target.2, t))
    }
}
