
struct Vec2<T> {
    x: T,
    y: T,
}

struct PlayerCharacter {

    position: Vec2<f64>,  // euclidian coordinates in the game world
    velocity: i32,        // m/s
}

enum ArrowKeyInput {
    Up,
    Down,
    Left,
    Right,
}


// Use this function to move the PlayerCharacter around in the world accoarding to arrow key input. 
fn translate_playercharacter(pc: PlayerCharacter, direction: ArrowKeyInput) -> PlayerCharacter {
    match direction {
        ArrowKeyInput::Up    => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x,
                                        y: pc.position.y + 1 * pc.velocity,
                                    },
                                    velocity: pc.velocity
                                },
        ArrowKeyInput::Down  => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x,
                                        y: pc.position.y - 1 * pc.velocity,
                                    },
                                    velocity: pc.velocity
                                },
        ArrowKeyInput::Left  => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x - 1 * pc.velocity,
                                        y: pc.position.y, 
                                    },
                                    velocity: pc.velocity
                                },
        ArrowKeyInput::Right => PlayerCharacter {
                                    position: Vec2 {
                                        x: pc.position.x + 1 * pc.velocity,
                                        y: pc.position.y, 
                                    },
                                    velocity: pc.velocity
                                },
    }
}