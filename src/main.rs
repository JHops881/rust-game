pub mod tiles;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    
    set_fullscreen(true);
    loop {
        // Clear screen
        clear_background(BLACK);
        
        // Sample text
        draw_text("rust-game", screen_width() / 2_f32, screen_height() / 2_f32, 12_f32, WHITE);

        // Allow exiting
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        next_frame().await
    }
}


