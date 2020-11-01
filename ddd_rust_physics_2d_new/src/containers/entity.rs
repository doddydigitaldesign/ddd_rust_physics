use super::shape::{EntityShape, Shape};

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub shape: EntityShape,
    pub entity: Shape,
}

impl Entity {
    pub fn new(entity: Shape) -> Option<Entity> {
        match entity.shape {
            EntityShape::None => None,
            EntityShape::Circle => Some(Entity {
                shape: EntityShape::Circle,
                entity,
            }),
            EntityShape::Rectangle => Some(Entity {
                shape: EntityShape::Rectangle,
                entity,
            }),
        }
    }
}
