use macroquad::prelude::*;

pub mod playercharacter;
use playercharacter::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    // initialize the player's character
    let mut player = PlayerCharacter::new();
    
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
        }
        if is_key_down(KeyCode::Left) {
            PlayerCharacter::translate(&mut player, Direction::Left);
        }
        if is_key_down(KeyCode::Up) {
            PlayerCharacter::translate(&mut player, Direction::Up);
        }
        if is_key_down(KeyCode::Down) {
            PlayerCharacter::translate(&mut player, Direction::Down);
        }

        // health and mana debug
        if is_key_released(KeyCode::U) {
            PlayerCharacter::damage(&mut player, 12.4);
            println!("Ouch! Your health is now {}", PlayerCharacter::get_health(&player));
        }
        if is_key_down(KeyCode::J) {
            PlayerCharacter::heal(&mut player, 6.7);
            println!("Thanks! Your health is now {}", PlayerCharacter::get_health(&player));
        }

        next_frame().await
    }
}


