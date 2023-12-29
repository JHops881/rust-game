
struct Vec2<T> {
    x: T,
    y: T
}

struct PlayerCharacter {

    position: Vec2<i32>, // euclidian coordinates
    velocity: i32        // m/s
}

enum ArrowKeyInput {
    Up,
    Down,
    Left,
    Right
}

fn translate_playercharacter(pc: PlayerCharacter, direction: ArrowKeyInput) -> PlayerCharacter {
    match direction {
        ArrowKeyInput::Up    => pc.position = ,
        ArrowKeyInput::Down  => return player,
        ArrowKeyInput::Left  => return player,
        ArrowKeyInput::Right => return player,
    }
}