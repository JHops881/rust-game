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
        
        // -- Basic Play UI and Debug Information. --
        let ui_font_size: f32 = 16.0;
        let new_line: f32 = 18.0;
        let ui_start_location: f32 = 20.0;

        /*
        
        Position: (X, Y)
        Health: ###
        Mana: ###

         */

        let player_position_text: String =
              "Position: (".to_string()
            + &player.get_position().x.to_string()
            + ", "
            + &player.get_position().y.to_string()
            + ")";
        let player_health_text: String = 
              "Health: ".to_string()
            + &player.get_health().to_string();
        let player_mana_text: String = 
              "Mana: ".to_string()
            + &player.get_mana().to_string();

        draw_text (
            &player_position_text,
            ui_start_location,
            ui_start_location,
            ui_font_size,
            WHITE,
        );
        draw_text (
            &player_health_text,
            ui_start_location,
            ui_start_location + 1.0 * new_line,
            ui_font_size,
            WHITE,
        );
        draw_text (
            &player_mana_text,
            ui_start_location,
            ui_start_location + 2.0 * new_line,
            ui_font_size,
            WHITE,
        );
        // -- End UI --


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


