pub mod entity_graphic;
pub mod constants;
pub mod graphics_math;
pub mod graphics_procedure;
pub mod net;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::time::{SystemTime, UNIX_EPOCH};

use constants::ZERO_FLOAT;
use macroquad::{
    color::BLACK,
    input::is_key_down,
    math::Vec2,
    miniquad::{window::set_fullscreen, KeyCode},
    window::{clear_background, next_frame},
};
use std::net::UdpSocket;

use crate::{
    entity_graphic::EntityGraphic,
    net::{recieve_net_data, send_net_message},
};

//////////////////////////////////// CODE /////////////////////////////////////

/*

Okay, so lets think about this.. The server is going to send us a block of data that contains the
the locations and types of graphics entities that are relevant to us.

it is our job to the decide which ones to take, add them to a collection, (like a list)
draw them


The client will always be one game tick behind the server so that it can use the "next" game
tick to lineraly interpolate the clients visuals.


*/

#[macroquad::main("Game Client")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* ============ CLIENT PLAYER INIT ============ */

    let mut client_camera_position: Vec2 = Vec2 {
        x: ZERO_FLOAT,
        y: ZERO_FLOAT,
    }; // euclidian coordinates in the game world

    // Client's Player Data
    let mut client_player_name: String = String::new();

    let mut client_player_speed_stat: f32 = ZERO_FLOAT; // jogging speed
    let mut client_player_health_stat: f32 = ZERO_FLOAT; // max health
    let mut client_player_mana_stat: f32 = ZERO_FLOAT; // max mana
    let mut client_player_power_stat: f32 = ZERO_FLOAT; // attack power
    let mut client_player_vitality_stat: f32 = ZERO_FLOAT; // health regen in hp/s
    let mut client_player_wisdom_stat: f32 = ZERO_FLOAT; // mana regen in mana/s
    let mut client_player_dexterity_stat: f32 = ZERO_FLOAT; // attack speed attacks/s
    let mut client_player_defense_stat: f32 = ZERO_FLOAT; // flat damage reduction

    let mut client_player_current_speed: f32 = ZERO_FLOAT; // in m/s
    let mut client_player_current_health: f32 = ZERO_FLOAT;
    let mut client_player_current_mana: f32 = ZERO_FLOAT;

    let mut client_player_is_dead: bool = false;
    let mut client_player_is_sprinting: bool = false;

    let mut client_player_basic_cost: f32 = ZERO_FLOAT; // mana cost of basic attack
    let mut client_player_kenetic_pulse_cost: f32 = ZERO_FLOAT; // ... kenetic_pulse
    let mut client_player_lightning_cost: f32 = ZERO_FLOAT;

    let mut client_player_basic_power_multi: f32 = ZERO_FLOAT; // a value that deterimes how much damage
    let mut client_player_kenetic_pulse_power_multi: f32 = ZERO_FLOAT; // it will do. It's a multiplier.
    let mut client_player_lightning_power_multi: f32 = ZERO_FLOAT; // damage = power_stat * value

    /* ============ NETWORK ============ */

    // let mut connection_ipv4: String = String::new();
    // let mut session_id:      String = String::new();

    let server_addr: &str = "127.0.0.1:42110";

    // bind to any available local port
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    dbg!(&socket);
    socket
        .set_nonblocking(true)
        .expect("failed to make socket non-blocking");

    let join_message: &str = "<JOIN>";
    send_net_message(&socket, join_message, server_addr);

    // To enable linear interpolation, the client must always be one update behind the server.
    // It stores it in next_server_update and uses the position values to interpolate the
    // the positions of the graphics entities on the client's side.

    /* ======== WINDOW SETTINGS ======== */

    set_fullscreen(true);

    /* ============ GRAPHICS =========== */

    let mut graphics_entities: Vec<EntityGraphic> = Vec::new();

    /* =========== GAME LOOP =========== */

    // game loop variable instantiations etc.
    let mut real_delta_time: f64;
    let mut last_update_time: f64 = system_time();
    let target_time_frame: f64 = 15.625; // 64 tick
    let mut accumulator: f64 = 0.0;

    // While game is running...
    loop {
        // game loop value calculations.
        real_delta_time = system_time() - last_update_time;
        last_update_time += real_delta_time;
        accumulator += real_delta_time;

        // register client input (macroquad will do this)
        // imaginiary_procedure();

        // Fixed update loop. Refer to `target_time_frame` above for updates per second.
        while accumulator > target_time_frame {
            /* UPDATE */

            // ######### SEND CLIENT INFO TO SERVER ######### //

            if is_key_down(KeyCode::W) {
                let data: &str = "<ACT: U>";
                send_net_message(&socket, data, server_addr);
            }

            // ######### RECEIVE SERVER UPDATE TO CLIENT ######### //
            let mut _msg_buf: [u8; 1476] = [0; 1476];
            let mut _new_update: bool = false;
            let msg_buf_option: Option<[u8; 1476]> = recieve_net_data(&socket);
            match msg_buf_option {
                Some(data) => {
                    _msg_buf = data;
                    _new_update = true;
                }
                None => (),
            }

            // ######### UPDATE CLIENT ACCOARDING TO STATE ######### //

            //if new_update {
            //    proccess_net_data(msg_buf);
            //}

            accumulator -= target_time_frame;
        }

        /* RENDER CALLS */

        // Linear interpolation factor. It is the percentage of time passed
        // between two updates being the current update and the next update.
        // we know if definitively because the updates come in at a fixed rate.
        let t: f64 = accumulator / target_time_frame;

        // Clear screen
        clear_background(BLACK);

        for g_e in graphics_entities.iter() {
            g_e.draw(t, &client_camera_position);
        }

        next_frame().await
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
