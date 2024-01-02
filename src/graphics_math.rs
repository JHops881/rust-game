use macroquad:: {
    math::Vec2,
    window:: {
        screen_width,
        screen_height
    }
};

use crate::player_character::PlayerCharacter;




/// converts coordinates on the screen to world coordinates
pub fn convert_to_world_coords(screen_coords: Vec2, center: &PlayerCharacter, tile_width: f32) -> Vec2 {

    Vec2 {

        x:   (screen_coords.x - screen_width() / 2.0)
           / tile_width
           + center.get_position().x,

        y: - (screen_coords.y - screen_height() / 2.0)
           / tile_width
           + center.get_position().y,
    }

}




/// use to convert the game world position of a character to a position on the screen.
pub fn convert_to_screen_coords(world_coords: Vec2, center: &PlayerCharacter, tile_width: f32) -> Vec2 {

    Vec2 {

        x:   (world_coords.x - center.get_position().x)
           * tile_width 
           + screen_width() / 2.0,

        y:   (world_coords.y - center.get_position().y)
           * -1.0
           * tile_width
           + screen_height() / 2.0,
    }
}
