use super::{
    physical::Physical,
    shape::{EntityShape, Shape},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Entity {
    pub shape: EntityShape,
    pub entity: Shape,
    pub physical: Physical,
}

impl Entity {
    pub fn new(entity: Shape, physical: Option<Physical>) -> Option<Entity> {
        let physical_or_default = match physical {
            None => Physical::new(),
            Some(phyisical) => phyisical,
        };

        match entity.shape {
            EntityShape::None => None,
            EntityShape::Circle => Some(Entity {
                shape: EntityShape::Circle,
                entity,
                physical: physical_or_default,
            }),
            EntityShape::Rectangle => Some(Entity {
                shape: EntityShape::Rectangle,
                entity,
                physical: physical_or_default,
            }),
        }
    }
}
