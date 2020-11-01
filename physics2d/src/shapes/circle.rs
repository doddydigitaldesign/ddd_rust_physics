use crate::containers::shape::EntityShape;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    pub entity_type: EntityShape,
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {
            entity_type: EntityShape::Circle,
            radius,
        }
    }
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    pub fn get_area(&self) -> f64 {
        let base = self.radius;
        PI * base.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use std::f64::consts::PI;
    #[test]
    fn it_gets_radius_and_area() {
        let radius: f64 = 5.9;
        let my_circle = Circle::new(radius);

        assert_eq!(my_circle.get_radius(), radius);
        assert_eq!(my_circle.get_area(), PI * radius.powi(2));
    }
    #[test]
    fn it_sets_radius() {
        let radius: f64 = 5.9;
        let mut my_circle = Circle::new(radius);

        let new_radius: f64 = 10.0;
        my_circle.set_radius(10.0);

        assert_eq!(my_circle.get_radius(), new_radius);
        assert_eq!(my_circle.get_area(), PI * new_radius.powi(2));
    }
}
