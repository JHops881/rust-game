use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

use crate::constants::TILE_WIDTH;

/// Depends on Glabal Variables: `MAIN_CHARACTER_INSTANCE` and `TILE_WIDTH`
pub fn convert_to_world_coords(screen_coords: &Vec2, camera_pos: &Vec2) -> Vec2 {
    let x: f32 = (screen_coords.x - screen_width() / 2.0) / TILE_WIDTH + camera_pos.x;

    let y: f32 = -(screen_coords.y - screen_height() / 2.0) / TILE_WIDTH + camera_pos.y;
    Vec2 { x, y }
}

/// Depends on Glabal Variables: `MAIN_CHARACTER_INSTANCE` and `TILE_WIDTH`
pub fn convert_to_screen_coords(world_coords: &Vec2, camera_pos: &Vec2) -> Vec2 {
    let x: f32 = (world_coords.x - camera_pos.x) * TILE_WIDTH + screen_width() / 2.0;

    let y: f32 = (world_coords.y - camera_pos.y) * -1.0 * TILE_WIDTH + screen_height() / 2.0;
    Vec2 { x, y }
}
