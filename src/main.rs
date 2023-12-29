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
        let playerpos: String = "(".to_string() + &player.get_position().x.to_string() + ", " + &player.get_position().y.to_string() + ")";
        
        draw_text(&playerpos, screen_width() / 2_f32, screen_height() / 2_f32, 12_f32, WHITE);

        // Allow exiting
        if is_key_down(KeyCode::Escape) { 
            std::process::exit(0);
        }

        // arrow movement
        if is_key_down(KeyCode::Right) {
            player.translate(Direction::Right);
        }
        if is_key_down(KeyCode::Left) {
            player.translate(Direction::Left);
        }
        if is_key_down(KeyCode::Up) {
            player.translate(Direction::Up);
        }
        if is_key_down(KeyCode::Down) {
            player.translate(Direction::Down);
        }

        // health and mana debug
        if is_key_released(KeyCode::U) {
            player.hurt(12.4);
            println!("Ouch! Your health is now {}", player.get_health());
        }
        if is_key_down(KeyCode::J) {
            player.heal(6.7);
            println!("Thanks! Your health is now {}", player.get_health());
        }

        next_frame().await
    }
}


