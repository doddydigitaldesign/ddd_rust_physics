use crate::shapes::{circle::Circle, rectangle::Rectangle};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityShape {
    None = 0,
    Circle = 1,
    Rectangle = 2,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Shape {
    pub shape: EntityShape,
    pub rectangle: Option<Rectangle>,
    pub circle: Option<Circle>,
}
