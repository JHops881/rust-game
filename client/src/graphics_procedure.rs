
/* 


GUI: Needs fixing


/// Procedure that displays Position, Health, and Mana, stats in real time in the top left corener of the screen.
pub fn draw_gui() {

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

*/