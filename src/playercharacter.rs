
use macrosquad::Math::Vec2; 
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html




struct PlayerCharacter {

    position: Vec2,  // euclidian coordinates in the game world
    velocity: i32,   // m/s
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/* TODO: NEEDS DELTATIME IN COMPUTATIONS */
///
/// Use this function to move the PlayerCharacter around in the world accoarding to arrow key input. 
///  
fn translate_playercharacter(pc: PlayerCharacter, d: Direction) -> PlayerCharacter {
    match d {
        Direction::Up    => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x,
                                        y: pc.position.y + 1 * pc.velocity,
                                    },
                                    velocity: pc.velocity
                                },
        Direction::Down  => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x,
                                        y: pc.position.y - 1 * pc.velocity,
                                    },
                                    velocity: pc.velocity
                                },
        Direction::Left  => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x - 1 * pc.velocity,
                                        y: pc.position.y, 
                                    },
                                    velocity: pc.velocity
                                },
        Direction::Right => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x + 1 * pc.velocity,
                                        y: pc.position.y, 
                                    },
                                    velocity: pc.velocity
                                },
    }
}