#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration {
    ax: f64,
    ay: f64,
    aangular: f64,
}

impl Acceleration {
    pub fn new(ax: f64, ay: f64, aangular: f64) -> Acceleration {
        Acceleration { ax, ay, aangular }
    }

    pub fn get_linear_acceleration(&self) -> (f64, f64) {
        (self.ax, self.ay)
    }

    pub fn get_angular_acceleration(&self) -> f64 {
        self.aangular
    }
}
