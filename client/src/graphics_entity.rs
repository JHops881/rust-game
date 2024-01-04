use macroquad::color::{WHITE, GRAY, BLUE, GREEN, PURPLE, RED, YELLOW, PINK, BROWN, MAGENTA, Color};
use macroquad::math::Vec2;
use macroquad::shapes::draw_circle;

use crate::graphics_math::convert_to_screen_coords;





// Anything that exists in the game world environment is a game entity.
pub enum  GameEntity {

    PlayerCharacter,

    BasicAttack,
    KeneticPulse,
    Lightning,

    ArmsDealer,

    // Enemy Character Typesx
    Ghoul,
    Phantom,
    Drinker,
    Crawler,

    // Dropped Item Types
    Coin,
    LootBag,
    
}

// A `GraphicsEntity` represents the graphical component of anything that is a `GameEntity` 
pub struct GraphicsEntity { 

    pub world_pos: Vec2,
    pub entity_type: GameEntity

}

impl GraphicsEntity {

    pub fn get_x(&self) -> f32 {
        self.world_pos.x
    }


    pub fn get_y(&self) -> f32 {
        self.world_pos.y
    }

    pub fn get_position_vec2(&self) -> Vec2 {
        self.world_pos
    }



    pub fn draw(&self) {

        let color: Color = match self.entity_type {
            GameEntity::PlayerCharacter => WHITE,
            GameEntity::BasicAttack     => GRAY,
            GameEntity::KeneticPulse    => BLUE,
            GameEntity::Lightning       => PURPLE,
            GameEntity::ArmsDealer      => WHITE,
            GameEntity::Ghoul           => GREEN,
            GameEntity::Phantom         => RED,
            GameEntity::Drinker         => MAGENTA,
            GameEntity::Crawler         => BROWN,
            GameEntity::Coin            => YELLOW,
            GameEntity::LootBag         => PINK,
        };

        let world_coords: Vec2 = self.get_position_vec2();

        let screen_coord: Vec2 = convert_to_screen_coords(world_coords);

        draw_circle(screen_coord.x, screen_coord.y, 8.0, color);


    }

    
}