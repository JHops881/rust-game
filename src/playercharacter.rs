
use macroquad::math::Vec2; 
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html




pub struct PlayerCharacter {

    pub position: Vec2,  // euclidian coordinates in the game world
    pub velocity: f32,   // m/s
}

impl PlayerCharacter {
    /* TODO: NEEDS DELTATIME IN COMPUTATIONS */
    ///
    /// Use this procedure to move the PlayerCharacter around in the world accoarding to arrow key input. 
    ///  
    pub fn translate(pc: &mut PlayerCharacter, d: Direction) {
        match d {
            Direction::Right => pc.position.x = pc.position.x + 1.0 * pc.velocity,
            Direction::Left  => pc.position.x = pc.position.x - 1.0 * pc.velocity,
            Direction::Up    => pc.position.y = pc.position.y + 1.0 * pc.velocity,
            Direction::Down  => pc.position.y = pc.position.y - 1.0 * pc.velocity,
            
        }
    }
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

