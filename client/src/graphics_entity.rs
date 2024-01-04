use macroquad::color::{WHITE, GRAY, BLUE, GREEN, PURPLE, RED, YELLOW, PINK, BROWN, MAGENTA, Color};
use macroquad::math::Vec2;
use macroquad::shapes::draw_circle;

use crate::graphics_math::convert_to_screen_coords;





/// This represents each unique tangible animate or inanimate body that exists in the game. 
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

/// This represents anything in the playbale game that exists in the game world and is drawn
pub struct GraphicsEntity { 

    pub this_world_pos: Vec2,
    pub next_world_pos: Vec2,
    pub entity_type: GameEntity

}


impl GraphicsEntity {

    /* THIS UPDATE */

    /// returns current x coord
    pub fn get_this_x(&self) -> f32 {
        self.this_world_pos.x
    }

    /// returns current y coord
    pub fn get_this_y(&self) -> f32 {
        self.this_world_pos.y
    }

    /// returns vector with current x and y coord
    pub fn get_this_position_vec2(&self) -> Vec2 {
        self.this_world_pos
    }


    /* NEXT UPDATE */

    /// returns current x coord
    pub fn get_next_x(&self) -> f32 {
        self.next_world_pos.x
    }

    /// returns current y coord
    pub fn get_next_y(&self) -> f32 {
        self.next_world_pos.y
    }

    /// returns vector with current x and y coord
    pub fn get_next_position_vec2(&self) -> Vec2 {
        self.next_world_pos
    }



    /// displays graphical entity on the screen to the user. Logic is handled internally,
    /// there is no need to be concerned with what it is. 
    pub fn draw(&self, t: f64) {

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

        // linear interpolation. 


        let lerp_coords: Vec2 = self.get_this_position_vec2() + (self.get_next_position_vec2() - self.get_this_position_vec2()) * t as f32;

        // let lerp_coords: Vec2 = self.get_this_position_vec2();

        let screen_coord: Vec2 = convert_to_screen_coords(lerp_coords);

        draw_circle(screen_coord.x, screen_coord.y, 8.0, color);


    }

    
}