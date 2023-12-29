pub mod tiles;

use macroquad::prelude::*;
use tiles::get_tile_set;
use tiles::{gen_map_chunk, draw_map_chunk};


/// Returns the configuration settings for the Macroquad window
/// see https://docs.rs/macroquad/latest/macroquad/attr.main.html
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Game".to_owned(),
        window_width: 640,
        window_height: 480,
        fullscreen: false,
        ..Default::default()
    }
}

/// Main function
#[macroquad::main(window_conf)]
async fn main() {
    let _tiles = get_tile_set();
    
    let map = gen_map_chunk(0.75);

    let mut looking_at: Vec2 = vec2(0., 0.);
    let mut looking_zoom = 1_f32;

    loop {
        // Clear screen
        clear_background(BLACK);

        // sample drawing of tiles
        draw_map_chunk(&map, &_tiles, &looking_at, &looking_zoom);

        // Allow exiting
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        if is_mouse_button_down(MouseButton::Left) {
            looking_at = mouse_delta_position() + looking_at;
        } else {
            // Discard the diff to avoid skipping around. This SHOULDN'T be
            // necessary...
            mouse_delta_position();
        }

        if mouse_wheel().1 > 0.1 {
            looking_zoom = looking_zoom * 2.;
        }

        if mouse_wheel().1 < 0.0 {
            looking_zoom = looking_zoom / 2.;
        }

        next_frame().await
    }
}



