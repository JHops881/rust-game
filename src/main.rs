use macroquad::prelude::*;

pub mod playercharacter;
use playercharacter::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    // initialize the player's character
    let mut player = PlayerCharacter {
        position: Vec2 {
            x: 0.0,
            y: 0.0,
        },
        velocity: 1.0,
    };
    // debug
    println!("The Player has started at X: {}, Y: {}", &player.position.x, &player.position.y);
    
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

        // arrow movement
        if is_key_down(KeyCode::Right) {
            PlayerCharacter::translate(&mut player, Direction::Right);
            // debug
            println!("The Player has moved to X: {}, Y: {}", &player.position.x, &player.position.y);
        }
        if is_key_down(KeyCode::Left) {
            PlayerCharacter::translate(&mut player, Direction::Left);
            // debug
            println!("The Player has moved to X: {}, Y: {}", &player.position.x, &player.position.y);
        }
        if is_key_down(KeyCode::Up) {
            PlayerCharacter::translate(&mut player, Direction::Up);
            // debug
            println!("The Player has moved to X: {}, Y: {}", &player.position.x, &player.position.y);
        }
        if is_key_down(KeyCode::Down) {
            PlayerCharacter::translate(&mut player, Direction::Down);
            // debug
            println!("The Player has moved to X: {}, Y: {}", &player.position.x, &player.position.y);
        }

        next_frame().await
    }
}


