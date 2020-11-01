use ddd_rust_physics_2d_new::{
    self,
    containers::{
        entity::Entity, physical::Physical, plane::Plane, shape::EntityShape, shape::Shape,
    },
    dynamics::{acceleration::Acceleration, velocity::Velocity},
    shapes::circle::Circle,
    shapes::rectangle::Rectangle,
    utils::point::Point,
};

fn main() {
    let position1 = Point::new(10.0, 125.0);
    let velocity1 = Velocity::new(2.0, 1.0, 0.0);
    let acceleration1 = Acceleration::new(0.1, 0.05, 0.0);
    let mass1: f64 = 1.0;
    let physical1 = Physical::from_values(position1, mass1, velocity1, acceleration1);

    let player1_circle = Circle::new(1.0);
    let player1_entity = Entity::new(
        Shape {
            circle: Some(player1_circle),
            rectangle: None,
            shape: EntityShape::Circle,
        },
        Some(physical1),
    );

    let position2 = Point::new(910.0, 425.0);
    let velocity2 = Velocity::new(-2.0, -1.0, 0.0);
    let acceleration2 = Acceleration::new(0.2, 0.1, 0.0);
    let mass2: f64 = 0.5;
    let physical2 = Physical::from_values(position2, mass2, velocity2, acceleration2);

    let player2_rectangle = Rectangle::new(1.0, 0.5);
    let player2_entity = Entity::new(
        Shape {
            circle: None,
            rectangle: Some(player2_rectangle),
            shape: EntityShape::Rectangle,
        },
        Some(physical2),
    );

    let mut plane = Plane::new(1000.0, 1000.0);

    let player1_id = "player1".to_string();
    let player2_id = "player2".to_string();

    plane.add_entity(player1_id, player1_entity.unwrap());
    plane.add_entity(player2_id, player2_entity.unwrap());

    let player1 = plane.get_entity("player1".to_string());
    let player2 = plane.get_entity("player2".to_string());

    println!("Player 1 - {}", format!("{:?}", player1));
    println!("Player 2 - {}", format!("{:?}", player2));
}
