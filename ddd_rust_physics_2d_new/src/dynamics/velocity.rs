#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    vx: f64,
    vy: f64,
    vangular: f64,
}

impl Velocity {
    pub fn new(vx: f64, vy: f64, vangular: f64) -> Velocity {
        Velocity { vx, vy, vangular }
    }
    pub fn get_velocity(&self) -> (f64, f64) {
        (self.vx, self.vy)
    }
    pub fn get_speed(&self) -> f64 {
        let abs_total = self.vx.powi(2) + self.vy.powi(2);
        abs_total.sqrt()
    }
    pub fn get_angular_velocity(&self) -> f64 {
        self.vangular
    }
    pub fn get_kinetic_energy(&self) -> f64 {
        let (velocity_x, velocity_y) = self.get_velocity();
        0.5 * velocity_x.powi(2) + 0.5 * velocity_y.powi(2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_linear_velocity() {
        use crate::dynamics::velocity::Velocity;
        let my_vel = Velocity::new(10.0, 5.0, 0.0);
        let speed: f64 = 125.0;
        assert_eq!(my_vel.get_velocity(), (10.0, 5.0));
        assert_eq!(my_vel.get_speed(), speed.sqrt());
    }

    #[test]
    fn it_angular_velocity() {
        use crate::dynamics::velocity::Velocity;
        let my_vel = Velocity::new(0.0, 0.0, 10.0);

        assert_eq!(my_vel.get_angular_velocity(), 10.0);
    }
}
