use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

pub mod player_character;
use player_character::*;

pub mod player_projectile;
use player_projectile::*;

pub mod enemy_character;
use enemy_character::*;

pub mod environment;
use environment::*;



const TILE_WIDTH: f32 = 16.0; // pixels

#[macroquad::main("BasicShapes")]
async fn main() {

    // initialize the player's character
    let mut player = PlayerCharacter::new();

    // initalize an enemy
    let mut ghoul = EnemyCharacter::new(EnemyType::Ghoul, Vec2 { x: 1.0, y: 1.0 });

    // lets make and environment to put all our things in.
    let mut environment = Environment::new();

    set_fullscreen(true);

    // gme loop crap
    // https://fulmanski.pl/zajecia/tippgk/zajecia_20162017/wyklad_cwiczenia_moje/game_loop_and_time.pdf
    let mut real_delta_time:   f64;
    let mut last_update_time:  f64 = system_time();
    let     game_time_factor:  f64 = 1.0;
    let     target_time_frame: f64 = 8.33333; // 120 fps
    let mut accumulator:       f64 = 0.0;

    // While game is running...
    loop {

        // delta time calculations
        real_delta_time   = system_time() - last_update_time;
        last_update_time += real_delta_time;
        accumulator      += real_delta_time;
        let game_delta_time: f32 = (real_delta_time * game_time_factor) as f32;

        // EXAMPLE GAME LOOP STRUCTURE
        // 1. input (handled by macroquad)
        // 2. update (we do this)
        // 3. draw   (we also do this)

        /* UPDATE */
        while accumulator > target_time_frame {

            // ghoul pahtfinding
            ghoul.move_towards_player(&player, target_time_frame as f32);
    
            player.update(target_time_frame as f32, &mut environment);
            
            // update projectiles
            for proj in environment.player_projectiles.iter_mut() {
                proj.update(target_time_frame as f32);
            }
    
            // Allow exiting
            if is_key_down(KeyCode::Escape) {
                std::process::exit(0);
            }

            accumulator -= target_time_frame;
    
        }
       
        /* DRAW */

        // Clear screen
        clear_background(BLACK);

        // draw player
        draw_player_character(&player, &player, RED);

        // draw enemy
        draw_enemy_character(&ghoul, &player, WHITE);

        // draw projectiles
        for proj in environment.player_projectiles.iter() {
            draw_player_projectile(proj, &player, BLUE);
        }

        // Basic Play UI and Debug Information.
        draw_gui(&player);

        next_frame().await
    }
}

/* Graphics functions. These need to be moved later on or deleted. Although, they work for now. 7:51pm 12/29/2023 */
// Not my problem!
/// use to convert the game world position of a character to a position on the screen.
pub fn convert_to_screen_coords(
    world_coords: Vec2,
    center: &PlayerCharacter,
    tile_width: f32,
) -> Vec2 {
    Vec2 {
        x: (world_coords.x - center.get_position().x) * tile_width + screen_width() / 2.0,
        y: (world_coords.y - center.get_position().y) * -1.0 * tile_width + screen_height() / 2.0,
    }
}

// TODO: So we can click on the screen to shoot
pub fn convert_to_world_coords() {

}

// crappy functions that will draw characters as a circles on the screen. Horrendous... Oh Well!
pub fn draw_player_character(
    character: &PlayerCharacter,
    perspective_from: &PlayerCharacter,
    c: Color,
) {
    // get the position of the character as screen coordinates...
    let character_screen_position: Vec2 =
        convert_to_screen_coords(character.get_position(), &perspective_from, TILE_WIDTH);
    // draw call. Self explanitory.
    draw_circle(
        character_screen_position.x,
        character_screen_position.y,
        8.0,
        c,
    );
} // I can't get the generics/templating to work so I just made two different functions, ... Works for now ig. Jeez.

pub fn draw_enemy_character(
    character: &EnemyCharacter,
    perspective_from: &PlayerCharacter,
    c: Color,
) {
    let character_screen_position: Vec2 =
        convert_to_screen_coords(character.get_position(), &perspective_from, TILE_WIDTH);
    draw_circle(
        character_screen_position.x,
        character_screen_position.y,
        8.0,
        c,
    );
}

// draw player projectile
pub fn draw_player_projectile(
    projectile: &PlayerProjectile,
    perspective_from: &PlayerCharacter,
    c: Color,
) {
    let proj_screen_position: Vec2 =
        convert_to_screen_coords(projectile.get_position(), &perspective_from, TILE_WIDTH);
    draw_circle(
        proj_screen_position.x,
        proj_screen_position.y,
        8.0,
        c,
    );
}

// Let's put the indev GUI into a single procedure that we can call in the main loop ezpz.
// This LGTM!
pub fn draw_gui(player: &PlayerCharacter) {
    let ui_font_size: f32 = 16.0;
    let new_line: f32 = 18.0;
    let ui_start_location: f32 = 20.0;

    /* EXAMPLE

    \/ [top left of screen] \/

    "Position: (X, Y)"
    "Health: ###"
    "Mana: ###"

    /\   /\    /\   /\   /\  /\
    */

    // This may be unreadable. Just refer to the example above. It does that.
    let player_position_text: String = "Position: (".to_string()
        + &player.get_position().x.to_string()
        + ", "
        + &player.get_position().y.to_string()
        + ")";
    let player_health_text: String = "Health: ".to_string() + &player.get_health().to_string();
    let player_mana_text: String = "Mana: ".to_string() + &player.get_mana().to_string();

    draw_text(
        &player_position_text,
        ui_start_location,
        ui_start_location,
        ui_font_size,
        WHITE,
    );
    draw_text(
        &player_health_text,
        ui_start_location,
        ui_start_location + 1.0 * new_line,
        ui_font_size,
        WHITE,
    );
    draw_text(
        &player_mana_text,
        ui_start_location,
        ui_start_location + 2.0 * new_line,
        ui_font_size,
        WHITE,
    );
}

/// Gives the current time in ms
pub fn system_time() -> f64 {
    // Get the current time
    let now = SystemTime::now();

    // Calculate the duration since the Unix epoch
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Extract various components from the duration
    let seconds = duration_since_epoch.as_secs();
    let milliseconds: f64 = duration_since_epoch.as_millis() as f64;
    return milliseconds;
}