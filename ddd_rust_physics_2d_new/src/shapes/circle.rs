use crate::{
    containers::shape::EntityShape,
    dynamics::{acceleration::Acceleration, velocity::Velocity},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    pub entity_type: EntityShape,
    x: f64,
    y: f64,
    radius: f64,
    angle: f64,
    velocity: Velocity,
    acceleration: Acceleration,
}

impl Circle {
    pub fn new(
        x: f64,
        y: f64,
        radius: f64,
        angle: Option<f64>,
        velocity: Velocity,
        acceleration: Acceleration,
    ) -> Circle {
        let angle_or_default: f64 = match angle {
            None => 0.0,
            Some(angle) => angle,
        };

        Circle {
            entity_type: EntityShape::Circle,
            x,
            y,
            radius,
            angle: angle_or_default,
            velocity,
            acceleration,
        }
    }
    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
    pub fn get_area(&self) -> f64 {
        let base = self.radius;
        std::f64::consts::PI * base.powi(2)
    }
    pub fn get_angle(&self) -> f64 {
        self.angle
    }
    pub fn get_velocity(&self) -> Velocity {
        self.velocity
    }
    pub fn get_acceleration(&self) -> Acceleration {
        self.acceleration
    }
}

#[cfg(test)]
mod tests {
    use super::{Acceleration, Velocity};
    #[test]
    fn it_circle() {
        use crate::shapes::circle::Circle;
        let velocity = Velocity::new(5.0, 5.0, 0.0);

        let acceleration = Acceleration::new(0.0, 0.0, 0.0);

        let my_circle = Circle::new(10.0, 20.0, 5.9, None, velocity, acceleration);

        let radius: f64 = 5.9;

        assert_eq!(my_circle.get_position(), (10.0, 20.0));
        assert_eq!(my_circle.get_radius(), 5.9);
        assert_eq!(my_circle.get_area(), std::f64::consts::PI * radius.powi(2));
        assert_eq!(my_circle.get_angle(), 0.0);
    }
}
