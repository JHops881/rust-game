pub mod entity_graphic;
pub mod constants;
pub mod graphics_math;
pub mod client_net;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::collections::HashMap;

use constants::ZERO_FLOAT;
use macroquad::{
    color::BLACK,
    input::{is_key_down, is_key_pressed},
    math::Vec2,
    miniquad::{window::set_fullscreen, KeyCode},
    window::{clear_background, next_frame},
};
use netlib::{system_time, ClientToServerMessage, PlayerData};
use uuid::Uuid;
use std::net::UdpSocket;

use crate::{
    entity_graphic::EntityGraphic,
    client_net::{seceive_server_message, send_client_message, proccess_server_message},
};

//////////////////////////////////// CODE /////////////////////////////////////

#[macroquad::main("Game Client")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* ============ CLIENT PLAYER INIT ============ */

    let mut client_camera_position: Vec2 = Vec2 {
        x: ZERO_FLOAT,
        y: ZERO_FLOAT,
    }; // euclidian coordinates in the game world

    // Client's Player Data
    let mut client_player_data: PlayerData = PlayerData::new_default();

    /* ============ NETWORK ============ */

    let server_addr: &str = "127.0.0.1:42110";

    // we are gonna send data from any available localhost port to the server. 
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    dbg!(&socket);
    socket
        .set_nonblocking(true) // means that it wont wait to receive info if a recv_from call is made
        .expect("failed to make socket non-blocking"); // and nothing is there

    // Tell server client is joining.. What is this is dropped? TODO: FIX
    let message = ClientToServerMessage::package_connection_join();
    send_client_message(&socket, &message, server_addr);

    /* ======== WINDOW SETTINGS ======== */

    // set_fullscreen(true);

    /* ============ GRAPHICS =========== */

    // client local store of all graphical game elements
    let mut graphics_entities: HashMap<Uuid ,EntityGraphic> = HashMap::new();

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

            // allow exiting with esc key. Also will tell the server that the player is leaving.
            if is_key_pressed(KeyCode::Escape) {
                let message = ClientToServerMessage::package_connection_drop();
                send_client_message(&socket, &message, server_addr);
                std::process::exit(0);
            }

            // calculate the vector of movement of a player from clients movement controls. 
            let mut direction_vec: Vec2 = Vec2 { x: 0.0, y: 0.0 };
            if is_key_down(KeyCode::W) {
                direction_vec.y += 1.0;
            }
            if is_key_down(KeyCode::A) {
                direction_vec.x += -1.0;
            }
            if is_key_down(KeyCode::S) {
                direction_vec.y += -1.0;
            }
            if is_key_down(KeyCode::D) {
                direction_vec.x += 1.0;
            }

            // prevent scary divding by 0 math
            if (direction_vec.x != 0.0) | (direction_vec.y != 0.0) {
                direction_vec /= direction_vec.x.hypot(direction_vec.y); // reduce to unit vec
            }

            // send directional movement to server.
            let message = ClientToServerMessage::package_action_move(direction_vec);
            send_client_message(&socket, &message, server_addr);
            
            // ######### RECEIVE SERVER UPDATE TO CLIENT ######### //
            let mut new_serialized_messages: Vec<[u8; 1476]> = Vec::new();
            loop {
                let msg_buf_option: Option<[u8; 1476]> = seceive_server_message(&socket);
                match msg_buf_option {
                    Some(data) => {
                        new_serialized_messages.push(data);
                    }
                    None => break,
                }
            }

            // ######### UPDATE CLIENT ACCOARDING TO STATE ######### //

            for msg in new_serialized_messages.iter() {
                proccess_server_message(msg, &mut graphics_entities)
            }

            accumulator -= target_time_frame;
        }

        /* RENDER CALLS */

        // Linear interpolation factor. It is the percentage of time passed
        // between two updates being the current update and the next update.
        // we know if definitively because the updates come in at a fixed rate.
        let t: f64 = accumulator / target_time_frame;

        // make screen background black
        clear_background(BLACK);

        // draw game graphical elements
        for (_, g_e) in graphics_entities.iter() {
            g_e.draw(t, &client_camera_position);
        }

        next_frame().await
    }
}


