use std::collections::HashMap;

use super::entity::Entity;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    pub size: (f64, f64),
    entities: HashMap<String, Entity>,
}

impl Plane {
    pub fn new(size_x: f64, size_y: f64) -> Plane {
        Plane {
            size: (size_x, size_y),
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, key: String, entity: Entity) {
        self.entities.insert(key, entity);
    }

    pub fn get_entity(&self, key: String) -> Entity {
        let entity = self.entities.get(&key);
        match entity {
            Some(value) => value.clone(),
            None => panic!("Could not find entity"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        containers::shape::{EntityShape, Shape},
        dynamics::{acceleration::Acceleration, velocity::Velocity},
        shapes::circle::Circle,
    };

    use super::Entity;
    use super::Plane;
    #[test]
    fn it_works() {
        let plane = Plane::new(10.0, 10.0);

        assert_eq!(plane.size.0, 10.0);
        assert_eq!(plane.size.1, 10.0);
    }

    #[test]
    fn it_adds_entity() {
        let circle = Circle::new(
            0.0,
            0.0,
            1.0,
            Some(0.0),
            Velocity::new(0.0, 0.0, 0.0),
            Acceleration::new(0.0, 0.0, 0.0),
        );
        let mut plane = Plane::new(10.0, 10.0);
        let entity = Entity::new(Shape {
            shape: EntityShape::Circle,
            circle: Some(circle),
            rectangle: None,
        })
        .unwrap();
        let circle_id = "circle".to_string();
        plane.add_entity(circle_id, entity);

        assert_eq!(
            plane.get_entity("circle".to_string()).shape,
            EntityShape::Circle
        );
    }
}
