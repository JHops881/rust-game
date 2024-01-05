pub mod tiles;
pub mod chunks;

use macroquad::prelude::*;
use tiles::*;
use chunks::*;

/// Returns the configuration settings for the Macroquad window
/// see https://docs.rs/macroquad/latest/macroquad/attr.main.html
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: true,
        ..Default::default()
    }
}

/// Main function
#[macroquad::main(window_conf)]
async fn main() {

    init_tile_atlas();
    let chunk = Chunk::new(IVec2 { x: 0, y: 0 });
    let chunk2 = Chunk::new(IVec2 { x: 1, y: 0 });
    loop {
        // Clear screen
        clear_background(BLACK);

        chunk.draw();
        chunk2.draw();

        // Allow exiting
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        next_frame().await
    }
}



