pub mod tiles;

use macroquad::prelude::*;
use tiles::*;


/// Returns the configuration settings for the Macroquad window
/// see https://docs.rs/macroquad/latest/macroquad/attr.main.html
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

/// Main function
#[macroquad::main(window_conf)]
async fn main() {

    init_tile_atlas();
    let map = gen_map_chunk(0.50);
    loop {
        // Clear screen
        clear_background(BLACK);

        draw_map_test(&map);

        // Allow exiting
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        next_frame().await
    }
}



