use std::time::{UNIX_EPOCH, SystemTime};

use global_variables::{MAIN_CHARACTER_INSTANCE, ZERO_FLOAT};
use graphics_entity::{GraphicsEntity, GameEntity};
use macroquad:: {miniquad::{window::set_fullscreen, KeyCode},
    window:: {
        clear_background,
        next_frame
    }, 
    color::BLACK, math::Vec2, input::is_key_down, prelude::rand
};
use server_update_object::ServerUpdateObject;


pub mod global_variables;
pub mod graphics_math;
pub mod graphics_procedure;
pub mod graphics_entity;
pub mod Graphics_Entity;
pub mod main_character_singleton;
pub mod server_update_object;
pub mod client_info_object;





/*

Okay, so lets think about this.. The server is going to send us a block of data that contains the
the locations and types of graphics entities that are relevant to us.

it is our job to the decide which ones to take, add them to a collection, (like a list)
draw them


The client will always be one game tick behind the server so that it can use the "next" game
tick to lineraly interpolate the clients visuals. 


*/


#[macroquad::main("BasicShapes")]

async fn main() {


    let mut connection_ipv4: String = String::new();
    let mut session_id:      String = String::new();

    set_fullscreen(true);

    //let main_player: GraphicsEntity = GraphicsEntity {world_pos: MAIN_CHARACTER_INSTANCE.get_position_vec2(), entity_type: GameEntity::PlayerCharacter};

    let mut ghoul: GraphicsEntity = GraphicsEntity { 
        this_world_pos: Vec2 {
            x: ZERO_FLOAT,
            y: ZERO_FLOAT,
        },
        next_world_pos: Vec2 {
            x: ZERO_FLOAT,
            y: ZERO_FLOAT,
        },
        entity_type: GameEntity::Ghoul
    };

    let mut this_server_update: ServerUpdateObject;
    let mut next_server_update: ServerUpdateObject;

    next_server_update = simulate_server_update();

    let mut real_delta_time: f64;
    let mut last_update_time: f64 = system_time();
    let target_time_frame: f64 = /* 15.625; // 64 tick */ 1000.0; // 1 tick
    let mut accumulator: f64 = 0.0;

    // While game is running...
    loop {

        real_delta_time = system_time() - last_update_time;
        last_update_time += real_delta_time;
        accumulator += real_delta_time;

        // grab local input (macroquad will do this)

        // push local input to server

        
        while accumulator > target_time_frame {

            /* UPDATE */

            // recieve server state
            this_server_update = next_server_update;
            next_server_update = simulate_server_update();

            ghoul = GraphicsEntity { 
                this_world_pos: this_server_update.ghoul_position,
                next_world_pos: next_server_update.ghoul_position,
                entity_type: GameEntity::Ghoul
            };
            

            // Allow exiting
            if is_key_down(KeyCode::Escape) {
                std::process::exit(0);
            }

            accumulator -= target_time_frame;
        }

        /* DRAW */

        // Clear screen
        clear_background(BLACK);

        ghoul.draw(accumulator / target_time_frame);
      


        // Basic Play UI and Debug Information

        next_frame().await
    }
}

/// pocedure to send client information to server
fn send_client_info() {

}


fn simulate_server_update() -> ServerUpdateObject {
    ServerUpdateObject {
        ghoul_position: Vec2 {
            x: rand::gen_range(-20.0, 20.0),
            y: rand::gen_range(-20.0, 20.0),
        }
    }
}




/// Gives the current time in ms
pub fn system_time() -> f64 {

    // Get the current time
    let now = SystemTime::now();

    // Calculate the duration since the Unix epoch
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let milliseconds: f64 = duration_since_epoch.as_millis() as f64;

    return milliseconds;
}
