use ddd_rust_physics_2d_new::{
    self,
    dynamics::{acceleration::Acceleration, velocity::Velocity},
};

fn main() {
    let velocity = Velocity::new(0.0, 0.0, 0.0);
    let acceleration = Acceleration::new(0.0, 0.0, 0.0);
    let circle = ddd_rust_physics_2d_new::shapes::circle::Circle::new(
        0.0,
        0.0,
        1.0,
        Some(0.0),
        velocity,
        acceleration,
    );
    let output = format!(
        "Circle - Position: {:?}, Velocity: {:?}, Acceleration: {:?}",
        circle.get_position(),
        circle.get_velocity().get_velocity(),
        circle.get_acceleration().get_linear_acceleration()
    );
    println!("{}", output);
}
