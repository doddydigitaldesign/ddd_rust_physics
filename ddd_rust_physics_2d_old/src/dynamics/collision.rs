use crate::utils::intersection::{circle_intersection, IntersectionResult};

use crate::{
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    shapes::circle::Circle,
    utils::intersection::Intersection,
};

#[derive(Debug, PartialEq)]
pub struct Collision<B> {
    body1: B,
    body2: B,
}

impl<B> Collision<B> {
    pub fn new(body1: B, body2: B) -> Collision<B> {
        Collision { body1, body2 }
    }
}

impl Collision<Circle> {
    pub fn is_collision(&self) -> bool {
        match self.get_contacts().result_type {
            IntersectionResult::NoIntersection => false,
            IntersectionResult::Intersection => true,
        }
    }

    pub fn get_contacts(&self) -> Intersection {
        circle_intersection(self.body1, self.body2)
    }

    pub fn get_velocities(&self) -> (Velocity, Velocity) {
        (self.body1.get_velocity(), self.body2.get_velocity())
    }

    pub fn get_accelerations(&self) -> (Acceleration, Acceleration) {
        (self.body1.get_acceleration(), self.body2.get_acceleration())
    }

    pub fn get_new_velocities(&self) -> (Velocity, Velocity) {
        let body1 = self.body1.get_velocity();
        let body2 = self.body2.get_velocity();

        // Assuming completely elastic collision
        let body1_velocity = body1.get_velocity();
        let body2_velocity = body2.get_velocity();

        let is_collision = self.is_collision();

        if is_collision == false {
            (body1, body2)
        } else {
            let (v1_x, v1_y) = body1_velocity;
            let (v2_x, v2_y) = body2_velocity;

            let m1 = self.body1.get_area();
            let m2 = self.body2.get_area();

            let v1_prime_x = ((m1 - m2) / (m1 + m2)) * v1_x + ((2.0 * m2) / (m1 + m2)) * v2_x;
            let v1_prime_y = ((m1 - m2) / (m1 + m2)) * v1_y + ((2.0 * m2) / (m1 + m2)) * v2_y;

            let v2_prime_x = ((2.0 * m1) / (m1 + m2)) * v1_x - ((m1 - m2) / (m1 + m2)) * v2_x;
            let v2_prime_y = ((2.0 * m1) / (m1 + m2)) * v1_y - ((m1 - m2) / (m1 + m2)) * v2_y;

            (
                Velocity::new(v1_prime_x, v1_prime_y, body1.get_angular_velocity()),
                Velocity::new(v2_prime_x, v2_prime_y, body2.get_angular_velocity()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Acceleration, Circle, Collision, Velocity};
    use crate::utils::point::Point;
    #[test]
    fn it_finds_circle_poc() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
        let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);
        let contacts = collision.get_contacts();
        let is_intersection = collision.is_collision();
        let result = contacts.result;
        if is_intersection {
            assert_eq!(
                result.unwrap(),
                (Point::new(7.5, 7.5), Point::new(7.5, 7.5))
            )
        }
    }

    #[test]
    fn it_circle_velocities() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
        let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);

        assert_eq!(collision.get_velocities(), (v1, v2));
    }

    #[test]
    fn it_circle_new_velocities() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let c1 = Circle::new(5.0, 0.0, 2.5, None, v1, a1);
        let c2 = Circle::new(0.0, 0.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);

        let (v1_prime, v2_prime) = collision.get_new_velocities();

        assert_eq!(v1_prime, v2);
        assert_eq!(v2_prime, v1);
    }

    #[test]
    fn it_circle_new_velocities_no_collision() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let c1 = Circle::new(5.0, 0.0, 2.5, None, v1, a1);
        let c2 = Circle::new(-5.0, 0.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);

        let (v1_prime, v2_prime) = collision.get_new_velocities();

        assert_eq!(v1_prime, v1);
        assert_eq!(v2_prime, v2);
    }

    #[test]
    fn it_circle_accelerations() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(1.0, 1.0, 1.0);
        let a2 = Acceleration::new(1.0, 1.0, 1.0);

        let c1 = Circle::new(10.0, 10.0, 2.5, None, v1, a1);
        let c2 = Circle::new(5.0, 5.0, 2.5, None, v2, a2);

        let collision: Collision<Circle> = Collision::new(c1, c2);

        assert_eq!(collision.get_accelerations(), (a1, a2));
    }
}
