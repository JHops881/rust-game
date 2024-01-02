use macroquad:: {
    color:: {
        Color,
        WHITE
    },
    math::Vec2,
    shapes::draw_circle,
    text::draw_text
};

use crate:: {
    graphics_math::*,
    player_character::PlayerCharacter,
    global_variables::TILE_WIDTH,
    enemy_character::EnemyCharacter,
    player_projectile::PlayerProjectile
};




// crappy functions that will draw characters as a circles on the screen. Horrendous... Oh Well!
pub fn draw_player_character(character: &PlayerCharacter, reference_point: &PlayerCharacter, color: Color) {

    // get the position of the character as screen coordinates...
    let character_screen_position: Vec2 =
        convert_to_screen_coords(character.get_position(), &reference_point, TILE_WIDTH);

    // draw call. Self explanitory.
    draw_circle(character_screen_position.x, character_screen_position.y, 8.0, color);
} 






// I can't get the generics/templating to work so I just made two different functions, ... Works for now ig. Jeez.
pub fn draw_enemy_character(character: &EnemyCharacter, reference_point: &PlayerCharacter, color: Color) {
    
    let character_screen_position: Vec2 =
        convert_to_screen_coords(character.get_position(), &reference_point, TILE_WIDTH);

    draw_circle(character_screen_position.x, character_screen_position.y, 8.0, color);
}






pub fn draw_player_projectile(projectile: &PlayerProjectile, reference_point: &PlayerCharacter, color: Color) {

    let proj_screen_position: Vec2 =
        convert_to_screen_coords(projectile.get_position(), &reference_point, TILE_WIDTH);

    draw_circle(proj_screen_position.x, proj_screen_position.y, 8.0, color);
}





/// Procedure that displays Position, Health, and Mana, stats in real time in the top left corener of the screen.
pub fn draw_gui(player: &PlayerCharacter) {

    let ui_font_size: f32 = 16.0;

    let new_line: f32 = 18.0;

    let ui_start_location: f32 = 20.0;

    //  "Position: (X, Y)"
    //  "Health: ###"
    //  "Mana: ###"

    // This may be unreadable. Just refer to the example above. It does that.
    let player_position_text: String = "Position: (".to_string()
        + &player.get_position().x.to_string()
        + ", "
        + &player.get_position().y.to_string()
        + ")";

    let player_health_text: String = "Health: ".to_string() + &player.get_health().to_string();

    let player_mana_text: String = "Mana: ".to_string() + &player.get_mana().to_string();

    draw_text(&player_position_text, ui_start_location, ui_start_location, ui_font_size, WHITE);
    draw_text(&player_health_text, ui_start_location, ui_start_location + 1.0 * new_line, ui_font_size, WHITE);
    draw_text(&player_mana_text, ui_start_location, ui_start_location + 2.0 * new_line, ui_font_size, WHITE);
}