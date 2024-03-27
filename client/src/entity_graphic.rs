use macroquad::color::{
    Color, BLUE, BROWN, GRAY, GREEN, MAGENTA, PINK, PURPLE, RED, WHITE, YELLOW,
};
use macroquad::math::Vec2;
use macroquad::shapes::draw_circle;
use netlib::EntityType;

use crate::graphics_math::convert_to_screen_coords;

/// This represents anything in the playbale game that exists in the game world and is drawn
/// Does not have and ID because it is intended to be put in a hashmap.
pub struct EntityGraphic {
    /// the world postion of the graphic in this update
    this_world_pos: Vec2,
    /// the world position of the graphic in the next update
    next_world_pos: Vec2,
    entity_type: EntityType,
}

impl EntityGraphic {
    pub fn new(this_world_pos: Vec2, next_world_pos: Vec2, entity_type: EntityType) -> Self {
        EntityGraphic {
            this_world_pos,
            next_world_pos,
            entity_type,
        }
    }

    /* THIS UPDATE */

    pub fn get_this_x(&self) -> f32 {
        self.this_world_pos.x
    }

    pub fn get_this_y(&self) -> f32 {
        self.this_world_pos.y
    }

    pub fn get_this_position_vec2(&self) -> Vec2 {
        self.this_world_pos
    }

    /* NEXT UPDATE */

    pub fn get_next_x(&self) -> f32 {
        self.next_world_pos.x
    }

    pub fn get_next_y(&self) -> f32 {
        self.next_world_pos.y
    }

    pub fn get_next_position_vec2(&self) -> Vec2 {
        self.next_world_pos
    }

    pub fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    /// displays graphical entity on the screen to the user. Logic is handled internally,
    /// there is no need to be concerned with what it is.
    pub fn draw(&self, t: f64, camera_pos: &Vec2) {
        let color: Color = match self.entity_type {
            EntityType::PlayerCharacter => WHITE,
            EntityType::BasicAttack => GRAY,
            EntityType::KeneticPulse => BLUE,
            EntityType::Lightning => PURPLE,
            EntityType::ArmsDealer => WHITE,
            EntityType::Ghoul => GREEN,
            EntityType::Phantom => RED,
            EntityType::Drinker => MAGENTA,
            EntityType::Crawler => BROWN,
            EntityType::Coin => YELLOW,
            EntityType::LootBag => PINK,
        };

        // linear interpolation.
        let lerp_coords: Vec2 = self.get_this_position_vec2()
            + (self.get_next_position_vec2() - self.get_this_position_vec2()) * t as f32;

        let screen_coord: Vec2 = convert_to_screen_coords(&lerp_coords, camera_pos);

        let entity_graphic_radius: f32 = 8.0; // chosen arbitrarily.. in pixels. 
        draw_circle(screen_coord.x, screen_coord.y, entity_graphic_radius, color);  
    }
}
