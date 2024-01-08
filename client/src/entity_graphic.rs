use macroquad::color::{WHITE, GRAY, BLUE, GREEN, PURPLE, RED, YELLOW, PINK, BROWN, MAGENTA, Color};
use macroquad::math::Vec2;
use macroquad::shapes::draw_circle;

use crate::entity_type::EntityType;
use crate::graphics_math::convert_to_screen_coords;






/// This represents anything in the playbale game that exists in the game world and is drawn
pub struct EntityGraphic {

    pub entity_type: EntityType,
    pub id:          i32,

    /// the world postion of the graphic in this update
    pub this_world_pos: Vec2,
    
    /// the world position of the graphic in the next update
    pub next_world_pos: Vec2,

    

}


impl EntityGraphic {

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
            EntityType::PlayerCharacter => WHITE,
            EntityType::BasicAttack     => GRAY,
            EntityType::KeneticPulse    => BLUE,
            EntityType::Lightning       => PURPLE,
            EntityType::ArmsDealer      => WHITE,
            EntityType::Ghoul           => GREEN,
            EntityType::Phantom         => RED,
            EntityType::Drinker         => MAGENTA,
            EntityType::Crawler         => BROWN,
            EntityType::Coin            => YELLOW,
            EntityType::LootBag         => PINK,
        };

        // linear interpolation. 


        let lerp_coords: Vec2 = self.get_this_position_vec2() + (self.get_next_position_vec2() - self.get_this_position_vec2()) * t as f32;

        // let lerp_coords: Vec2 = self.get_this_position_vec2();

        let screen_coord: Vec2 = convert_to_screen_coords(lerp_coords);

        draw_circle(screen_coord.x, screen_coord.y, 8.0, color);


    }

    
}