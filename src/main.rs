
use macroquad::prelude::*;

pub mod character;
use character::*;

const TILE_WIDTH: f32 = 64.0; // pixels

#[macroquad::main("BasicShapes")]
async fn main() {

    // initialize the player's character
    let mut player = PlayerCharacter::new();
    let mut ghoul = EnemyCharacter::new(EnemyType::Ghoul, Vec2{x: 1.0, y:1.0});

    let mut allies:  Vec<&mut PlayerCharacter> = Vec::new(); // gonna do something with these later.
    let mut enemies: Vec<&mut EnemyCharacter> = Vec::new();  // Although, not really sure it's the best
                                                             // way to go about it... Oh Well!
    set_fullscreen(true);

    // gme loop crap
    // https://fulmanski.pl/zajecia/tippgk/zajecia_20162017/wyklad_cwiczenia_moje/game_loop_and_time.pdf
    let mut last_update_time: f32 = get_time() as f32;
    let game_time_factor: f32 = 1.0;

    loop {

        // delta time calculations
        let real_delta_time = get_time() as f32 - last_update_time;
        last_update_time = last_update_time + real_delta_time;
        let game_delta_time : f32 = real_delta_time * game_time_factor;


        // 1. input
        // 2. update
        // 3. draw

        // input
        // handled for us by macro quad. 

        // update
        // arrow moevemnt
        if is_key_down(KeyCode::D) {
            player.translate(Direction::Right, game_delta_time);
        }
        if is_key_down(KeyCode::A) {
            player.translate(Direction::Left, game_delta_time);
        }
        if is_key_down(KeyCode::W) {
            player.translate(Direction::Up, game_delta_time);
        }
        if is_key_down(KeyCode::S) {
            player.translate(Direction::Down, game_delta_time);
        }
        
        // sprinting mechanic implementation -From: Jason Ryan de la Masa
        if is_key_pressed(KeyCode::LeftShift) {
            player.begin_sprint();
        }
        if is_key_released(KeyCode::LeftShift) {
            player.end_sprint();
        }

        // Allow exiting
        if is_key_down(KeyCode::Escape) { 
            std::process::exit(0);
        }

        // draw
        // Clear screen
        clear_background(BLACK);

        // draw player 
        draw_player_character(&player, &player, RED);

        // draw enemy
        draw_enemy_character(&ghoul, &player, WHITE);
        
        // Basic Play UI and Debug Information.
        draw_gui(&player);
      

        next_frame().await
    }
}



/* Graphics functions. These need to be moved later on or deleted. Although, they work for now. 7:51pm 12/29/2023 */
// Not my problem!
/// use to convert the game world position of a character to a position on the screen. 
pub fn convert_to_screen_coords(world_coords: Vec2, center: &PlayerCharacter, tile_width: f32) -> Vec2 {
    Vec2 {
        x: (world_coords.x - center.get_position().x) * tile_width + screen_width() / 2.0,
        y: (world_coords.y - center.get_position().y) * -1.0 * tile_width + screen_height() / 2.0,
    }
}

// crappy functions that will draw characters as a circles on the screen. Horrendous... Oh Well!
pub fn draw_player_character(character: &PlayerCharacter, perspective_from: &PlayerCharacter, c: Color) {

    // get the position of the character as screen coordinates...
    let character_screen_position: Vec2 = convert_to_screen_coords(character.get_position(), &perspective_from, TILE_WIDTH);
    // draw call. Self explanitory. 
    draw_circle(character_screen_position.x, character_screen_position.y, 32.0, c);

} // I can't get the generics/templating to work so I just made two different functions, ... Works for now IG.

pub fn draw_enemy_character(character: &EnemyCharacter, perspective_from: &PlayerCharacter, c: Color) {
    let character_screen_position: Vec2 = convert_to_screen_coords(character.get_position(), &perspective_from, TILE_WIDTH);
    draw_circle(character_screen_position.x, character_screen_position.y, 32.0, c);
}


// Let's put the indev GUI into a single procedure that we can call in the main loop ezpz. 
// This LGTM!
pub fn draw_gui(player: &PlayerCharacter) {

    let ui_font_size: f32 = 16.0;
    let new_line: f32 = 18.0;
    let ui_start_location: f32 = 20.0;

    /* EXAMPLE
    
    Position: (X, Y)
    Health: ###
    Mana: ###

    */

    // This may be unreadable. Just refer to the example above. 
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
}