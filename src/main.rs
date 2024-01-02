pub mod player_character;

pub mod player_projectile;

pub mod enemy_character;

pub mod character_factory;

pub mod environment_singleton;

pub mod global_variables;

pub mod graphics_procedure;

pub mod graphics_math;


////////////////////////////////// IMPORTS ////////////////////////////////////


use std::time::{SystemTime, UNIX_EPOCH};

use enemy_character:: {
    EnemyCharacter,
    EnemyType
};

use global_variables::ENVIRONMENT_INSTANCE;
use graphics_procedure:: {
    draw_enemy_character,
    draw_gui,
    draw_player_character,
    draw_player_projectile,
};
use macroquad:: {
    color:: {
        BLACK,
        BLUE,
        GREEN,
        ORANGE,
        PINK,
        RED,
        WHITE,
        YELLOW
    },
    input::is_key_down,
    math::Vec2,
    miniquad:: {
        window::set_fullscreen,
        KeyCode
    },
    window:: {
        clear_background,
        next_frame
    },
};

use player_character::PlayerCharacter;


//////////////////////////////////// CODE /////////////////////////////////////


#[macroquad::main("BasicShapes")]

async fn main() {

    // initialize the player's character
    let mut player = PlayerCharacter::new();

    // initalize an enemy
    let mut ghoul = EnemyCharacter::new(EnemyType::Ghoul, Vec2 { x: 1.0, y: 1.0 });

    set_fullscreen(true);

    // gme loop crap
    // https://fulmanski.pl/zajecia/tippgk/zajecia_20162017/wyklad_cwiczenia_moje/game_loop_and_time.pdf
    let mut real_delta_time: f64;
    let mut last_update_time: f64 = system_time();
    let game_time_factor: f64 = 1.0;
    let target_time_frame: f64 = 8.33333; // 120 fps
    let mut accumulator: f64 = 0.0;

    // While game is running...
    loop {
        // delta time calculations
        real_delta_time = system_time() - last_update_time;
        last_update_time += real_delta_time;
        accumulator += real_delta_time;
        let game_delta_time: f32 = (real_delta_time * game_time_factor) as f32;

        // EXAMPLE GAME LOOP STRUCTURE
        // 1. input (handled by macroquad)
        // 2. update (we do this)
        // 3. draw   (we also do this)

        /* UPDATE */
        while accumulator > target_time_frame {
            // ghoul pahtfinding
            ghoul.move_towards_player(&player, target_time_frame as f32);

            player.update(target_time_frame as f32);

            // update projectiles
            let result = ENVIRONMENT_INSTANCE.lock();
            match result {
                Ok(mut env_inst) => 
                    for proj in env_inst.player_projectiles.iter_mut() {
                        proj.update(target_time_frame as f32);
                    }
                Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
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
        let result = ENVIRONMENT_INSTANCE.lock();
        match result {
            Ok(env_inst) => 
                for proj in env_inst.player_projectiles.iter() {
                    draw_player_projectile(proj, &player, BLUE);
                }
            Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
        }

        // Basic Play UI and Debug Information.
        draw_gui(&player);

        next_frame().await
    }
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
