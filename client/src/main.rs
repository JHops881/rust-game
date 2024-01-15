pub mod entity_graphic;
pub mod constants;
pub mod graphics_math;
pub mod graphics_procedure;
pub mod net;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::{time::{SystemTime, UNIX_EPOCH}, collections::HashMap};

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
    net::{seceive_server_message, send_client_message, proccess_server_message},
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
    let mut client_player_data: PlayerData = PlayerData::new_default();

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

    // Tell server client is joining
    let message = ClientToServerMessage::package_connection_join();
    send_client_message(&socket, &message, server_addr);

    // To enable linear interpolation, the client must always be one update behind the server.
    // It stores it in next_server_update and uses the position values to interpolate the
    // the positions of the graphics entities on the client's side.

    /* ======== WINDOW SETTINGS ======== */

    // set_fullscreen(true);

    /* ============ GRAPHICS =========== */

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

            if is_key_pressed(KeyCode::Escape) {
                let message = ClientToServerMessage::package_connection_drop();
                send_client_message(&socket, &message, server_addr);
                std::process::exit(0);
            }
            if is_key_down(KeyCode::W) {
                let message = ClientToServerMessage::package_action_move(Vec2 { x: 0.0, y: 1.0 });
                send_client_message(&socket, &message, server_addr);
            }
            
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

        // Clear screen
        clear_background(BLACK);

        for (_, g_e) in graphics_entities.iter() {
            g_e.draw(t, &client_camera_position);
        }

        next_frame().await
    }
}


