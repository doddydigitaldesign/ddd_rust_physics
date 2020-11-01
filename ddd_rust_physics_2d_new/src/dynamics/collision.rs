use crate::{
    containers::{entity::Entity, shape::EntityShape},
    utils::intersection::{circle_intersection, IntersectionResult},
};

use crate::{
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    utils::intersection::Intersection,
};

#[derive(Debug, PartialEq)]
pub struct Collision {
    body1: Entity,
    body2: Entity,
}

impl Collision {
    pub fn new(body1: Entity, body2: Entity) -> Collision {
        Collision { body1, body2 }
    }
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
        (
            self.body1.physical.get_velocity(),
            self.body2.physical.get_velocity(),
        )
    }

    pub fn get_accelerations(&self) -> (Acceleration, Acceleration) {
        (
            self.body1.physical.get_acceleration(),
            self.body2.physical.get_acceleration(),
        )
    }

    pub fn get_velocities_elastic(
        velocity1: Velocity,
        mass1: f64,
        velocity2: Velocity,
        mass2: f64,
    ) -> (Velocity, Velocity) {
        let (v1_x, v1_y) = velocity1.get_velocity();
        let (v2_x, v2_y) = velocity2.get_velocity();

        let v1_prime_x =
            ((mass1 - mass2) / (mass1 + mass2)) * v1_x + ((2.0 * mass2) / (mass1 + mass2)) * v2_x;
        let v1_prime_y =
            ((mass1 - mass2) / (mass1 + mass2)) * v1_y + ((2.0 * mass2) / (mass1 + mass2)) * v2_y;

        let v2_prime_x =
            ((2.0 * mass1) / (mass1 + mass2)) * v1_x - ((mass1 - mass2) / (mass1 + mass2)) * v2_x;
        let v2_prime_y =
            ((2.0 * mass1) / (mass1 + mass2)) * v1_y - ((mass1 - mass2) / (mass1 + mass2)) * v2_y;

        (
            Velocity::new(v1_prime_x, v1_prime_y, velocity1.get_angular_velocity()),
            Velocity::new(v2_prime_x, v2_prime_y, velocity2.get_angular_velocity()),
        )
    }

    pub fn get_new_velocities(&self) -> (Velocity, Velocity) {
        let body1 = &self.body1;
        let body2 = &self.body2;

        // Assuming completely elastic collision
        let body1_velocity = body1.physical.get_velocity();
        let body2_velocity = body2.physical.get_velocity();

        let is_collision = self.is_collision();

        if is_collision == false
            || body1.entity.shape == EntityShape::None
            || body2.entity.shape == EntityShape::None
        {
            (body1_velocity, body2_velocity)
        } else {
            let m1: f64 = 1.0;
            let m2: f64 = 1.0;

            let velocities_prime =
                Collision::get_velocities_elastic(body1_velocity, m1, body2_velocity, m2);

            velocities_prime
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Acceleration, Collision, Velocity};
    use crate::{
        containers::entity::Entity, containers::physical::Physical, containers::shape::EntityShape,
        containers::shape::Shape, shapes::circle::Circle, utils::point::Point,
    };
    #[test]
    fn it_finds_circle_poc() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let phys1 = Physical::from_values(Point::new(0.0, 0.0), 1.0, v1, a1);
        let phys2 = Physical::from_values(Point::new(5.0, 0.0), 1.0, v2, a2);

        let c1 = Entity::new(
            Shape {
                circle: Some(Circle::new(2.5)),
                shape: EntityShape::Circle,
                rectangle: None,
            },
            Some(phys1),
        )
        .unwrap();
        let c2 = Entity::new(
            Shape {
                shape: EntityShape::Circle,
                rectangle: None,
                circle: Some(Circle::new(2.5)),
            },
            Some(phys2),
        )
        .unwrap();

        let collision = Collision::new(c1, c2);
        let contacts = collision.get_contacts();
        let is_intersection = collision.is_collision();
        let result = contacts.result;
        if is_intersection {
            assert_eq!(
                result.unwrap(),
                (Point::new(2.5, 2.5), Point::new(2.5, 2.5))
            )
        }
    }

    #[test]
    fn it_circle_velocities() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let phys1 = Physical::from_values(Point::new(0.0, 0.0), 1.0, v1, a1);
        let phys2 = Physical::from_values(Point::new(5.0, 0.0), 1.0, v2, a2);

        let c1 = Entity::new(
            Shape {
                circle: Some(Circle::new(2.5)),
                shape: EntityShape::Circle,
                rectangle: None,
            },
            Some(phys1),
        )
        .unwrap();
        let c2 = Entity::new(
            Shape {
                shape: EntityShape::Circle,
                rectangle: None,
                circle: Some(Circle::new(2.5)),
            },
            Some(phys2),
        )
        .unwrap();

        let collision = Collision::new(c1, c2);

        assert_eq!(collision.get_velocities(), (v1, v2));
    }

    #[test]
    fn it_circle_new_velocities() {
        let v1 = Velocity::new(5.0, 5.0, 0.0);
        let v2 = Velocity::new(-5.0, 5.0, 0.0);

        let a1 = Acceleration::new(0.0, 0.0, 0.0);
        let a2 = Acceleration::new(0.0, 0.0, 0.0);

        let phys1 = Physical::from_values(Point::new(0.0, 0.0), 1.0, v1, a1);
        let phys2 = Physical::from_values(Point::new(5.0, 0.0), 1.0, v2, a2);

        let c1 = Entity::new(
            Shape {
                circle: Some(Circle::new(2.5)),
                shape: EntityShape::Circle,
                rectangle: None,
            },
            Some(phys1),
        )
        .unwrap();
        let c2 = Entity::new(
            Shape {
                shape: EntityShape::Circle,
                rectangle: None,
                circle: Some(Circle::new(2.5)),
            },
            Some(phys2),
        )
        .unwrap();

        let collision = Collision::new(c1, c2);

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

        let phys1 = Physical::from_values(Point::new(0.0, 0.0), 1.0, v1, a1);
        let phys2 = Physical::from_values(Point::new(10.0, 0.0), 1.0, v2, a2);

        let c1 = Entity::new(
            Shape {
                circle: Some(Circle::new(2.5)),
                shape: EntityShape::Circle,
                rectangle: None,
            },
            Some(phys1),
        )
        .unwrap();
        let c2 = Entity::new(
            Shape {
                shape: EntityShape::Circle,
                rectangle: None,
                circle: Some(Circle::new(2.5)),
            },
            Some(phys2),
        )
        .unwrap();

        let collision = Collision::new(c1, c2);

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

        let phys1 = Physical::from_values(Point::new(0.0, 0.0), 1.0, v1, a1);
        let phys2 = Physical::from_values(Point::new(5.0, 0.0), 1.0, v2, a2);

        let c1 = Entity::new(
            Shape {
                circle: Some(Circle::new(2.5)),
                shape: EntityShape::Circle,
                rectangle: None,
            },
            Some(phys1),
        )
        .unwrap();
        let c2 = Entity::new(
            Shape {
                shape: EntityShape::Circle,
                rectangle: None,
                circle: Some(Circle::new(2.5)),
            },
            Some(phys2),
        )
        .unwrap();

        let collision = Collision::new(c1, c2);

        assert_eq!(collision.get_accelerations(), (a1, a2));
    }
}
