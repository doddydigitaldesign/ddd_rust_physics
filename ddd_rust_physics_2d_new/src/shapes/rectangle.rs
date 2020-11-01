use crate::containers::shape::EntityShape;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Rectangle {
    pub entity_type: EntityShape,
    size_y: f64,
    size_x: f64,
}

impl Rectangle {
    pub fn new(size_x: f64, size_y: f64) -> Rectangle {
        Rectangle {
            entity_type: EntityShape::Rectangle,
            size_y,
            size_x,
        }
    }

    pub fn get_sizes(&self) -> (f64, f64) {
        (self.size_x, self.size_y)
    }

    pub fn set_sizes(&mut self, size_x: f64, size_y: f64) {
        self.size_x = size_x;
        self.size_y = size_y;
    }

    pub fn get_area(&self) -> f64 {
        self.size_x * self.size_y
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_rectangle() {
        use crate::shapes::rectangle::Rectangle;

        let size_x: f64 = 10.0;
        let size_y: f64 = 20.0;

        let my_rect = Rectangle::new(size_x, size_y);
        assert_eq!(my_rect.get_sizes(), (size_x, size_y));
    }
}
