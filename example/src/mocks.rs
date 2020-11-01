use physics2d::{
    containers::{entity::Entity, physical::Physical, shape::EntityShape, shape::Shape},
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    shapes::{circle::Circle, rectangle::Rectangle},
    utils::point::Point,
};

pub enum PlayerShape {
    Circle,
    Rectangle,
}
pub struct PlayerDeps {
    pub(crate) shape: PlayerShape,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) vx: f64,
    pub(crate) vy: f64,
    pub(crate) ax: f64,
    pub(crate) ay: f64,
    pub(crate) mass: f64,
}
pub fn create_mock_player(deps: PlayerDeps) -> Option<Entity> {
    let position = Point::new(deps.x, deps.y);
    let velocity = Velocity::new(deps.vx, deps.vy, 0.0);
    let acceleration = Acceleration::new(deps.ax, deps.ay, 0.0);
    let physical = Physical::from_values(position, deps.mass, velocity, acceleration);

    let player_shape = match deps.shape {
        PlayerShape::Circle => Shape {
            circle: Some(Circle::new(1.0)),
            rectangle: None,
            shape: EntityShape::Circle,
        },
        PlayerShape::Rectangle => Shape {
            circle: None,
            rectangle: Some(Rectangle::new(1.0, 0.5)),
            shape: EntityShape::Rectangle,
        },
    };
    let player_entity = Entity::new(player_shape, Some(physical));

    player_entity
}
