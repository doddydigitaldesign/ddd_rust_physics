pub mod mocks;
use mocks::{PlayerDeps, PlayerShape};
use physics2d::{self, containers::plane::Plane};

fn main() {
    let player1_entity = mocks::create_mock_player(PlayerDeps {
        ax: 0.1,
        ay: 0.05,
        shape: PlayerShape::Circle,
        x: 10.0,
        y: 125.0,
        vx: 2.0,
        vy: 1.0,
        mass: 1.0,
    });
    let player2_entity = mocks::create_mock_player(PlayerDeps {
        ax: 0.2,
        ay: 0.1,
        shape: PlayerShape::Rectangle,
        x: 910.0,
        y: 425.0,
        vx: -2.0,
        vy: -1.0,
        mass: 0.5,
    });

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
