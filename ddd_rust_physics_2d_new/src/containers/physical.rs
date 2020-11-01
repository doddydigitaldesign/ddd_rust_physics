use crate::{
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    utils::point::Point,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Physical {
    acceleration: Acceleration,
    velocity: Velocity,
    position: Point,
    mass: f64,
}

impl Physical {
    pub fn new() -> Physical {
        Physical {
            acceleration: Acceleration::new(0.0, 0.0, 0.0),
            mass: 0.0,
            velocity: Velocity::new(0.0, 0.0, 0.0),
            position: Point::new(0.0, 0.0),
        }
    }

    pub fn from_values(
        position: Point,
        mass: f64,
        velocity: Velocity,
        acceleration: Acceleration,
    ) -> Physical {
        Physical {
            acceleration,
            mass,
            velocity,
            position,
        }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }

    pub fn get_position(&self) -> Point {
        self.position
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn get_velocity(&self) -> Velocity {
        self.velocity
    }
    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn get_acceleration(&self) -> Acceleration {
        self.acceleration
    }

    pub fn set_acceleration(&mut self, acceleration: Acceleration) {
        self.acceleration = acceleration;
    }
}
