extern "C" {
    fn rand() -> i32;
}
pub struct RandState();
impl RandState {
    pub fn new() -> Self {
        Self()
    }

    pub fn random_double(&mut self) -> f64 {
        return (unsafe { rand() } as f64) / (32767 as f64 + 1.0);
    }
    pub fn random_double_range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.random_double()
    }
}
