pub struct Rectangle {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
    angle: f64,
}

impl Rectangle {
    pub fn new (x: f64, y: f64, height: f64, width: f64, angle: Option<f64>) -> Rectangle {
        let angle_or_default: f64 = match angle {
            None => 0.0,
            Some(angle) => angle,
        };

        Rectangle {
            x,
            y,
            height,
            width,
            angle: angle_or_default
        }
    }
    pub fn get_position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
    pub fn get_size(&self) -> (f64, f64) {
        (self.height, self.width)
    }
    pub fn get_angle(&self) -> f64 {
        self.angle
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_rectangle() {
        use crate::shapes::rectangle::Rectangle;
        let my_rect = Rectangle::new(10.0, 20.0, 5.0, 7.0, None);
        assert_eq!(my_rect.get_position(), (10.0, 20.0));
        assert_eq!(my_rect.get_size(), (5.0, 7.0));
        assert_eq!(my_rect.get_angle(), 0.0);
    }
}