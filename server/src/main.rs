pub mod player_character;

pub mod player_projectile;

pub mod enemy_character;

pub mod game_world;

pub mod spell;

pub mod net;

pub mod constants;

pub mod entity_component_system;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::Sender,
    time::{SystemTime, UNIX_EPOCH},
};

use constants::ZERO_ZERO;
use enemy_character::{EnemyCharacter, EnemyType};

use macroquad::{
    color::{BLACK, BLUE, WHITE},
    math::Vec2,
    text::draw_text,
    window::{clear_background, next_frame},
};
use net::{proccess_client_message, recieve_client_message, send_game_state_to_connected_players};
use netlib::system_time;
use uuid::Uuid;

//////////////////////////////////// CODE /////////////////////////////////////

#[macroquad::main("Game Server GUI")]
async fn main() {
    let mut game_world: GameWorld = GameWorld::new();

    let mut connection_table: HashMap<String, Uuid> = HashMap::new();

    // listen for information on port 42110
    let socket = UdpSocket::bind("127.0.0.1:42110").expect("couldn't bind to address");
    socket
        .set_nonblocking(true)
        .expect("failed to make socket non-blocking");

    // initalize an enemy
    EntityFactory::create_ghoul(ZERO_ZERO, &mut game_world);

    // game loop crap
    // https://fulmanski.pl/zajecia/tippgk/zajecia_20162017/wyklad_cwiczenia_moje/game_loop_and_time.pdf
    let mut real_delta_time: f64;
    let mut last_update_time: f64 = system_time();
    let target_time_frame: f64 = 15.625; // 64 tick
    let mut accumulator: f64 = 0.0;

    // While game is running...
    loop {
        // delta time calculations
        real_delta_time = system_time() - last_update_time;
        last_update_time += real_delta_time;
        accumulator += real_delta_time;

        while accumulator > target_time_frame {
            /* UPDATE */

            // ######### RECEIVE CLIENTS' DATA ######### //
            let mut new_serialized_messages: Vec<([u8; 1476], SocketAddr)> = Vec::new();
            loop {
                let pair: Option<([u8; 1476], SocketAddr)> = recieve_client_message(&socket);
                match pair {
                    Some(data) => {
                        new_serialized_messages.push(data);
                    }
                    None => break,
                }
            }

            // process net messages
            for (ser_msg, sender) in new_serialized_messages.iter() {
                proccess_client_message(ser_msg, *sender, &mut connection_table, &mut game_world);
            }

            // update new game state
            game_world.fixed_update(target_time_frame as f32);

            send_game_state_to_connected_players(&socket, &connection_table, &game_world);

            accumulator -= target_time_frame;
        }

        clear_background(BLACK);

        // display connections
        let font_size: f32 = 30.0;
        let newline_offset: f32 = 30.0; // all in pixels
        let x_pos: f32 = 12.0;
        let y_pos: f32 = 25.0;

        draw_text("CURRENT CONNECTIONS:", x_pos, y_pos, font_size, BLUE);

        let mut connection_number: i32 = 1;
        for (address, id) in connection_table.iter() {
            let connection_num_y_pos: f32 = y_pos + connection_number as f32 * newline_offset;
            draw_text(
                (id.to_string() + ", " + address.as_str()).as_str(),
                x_pos,
                connection_num_y_pos,
                font_size,
                WHITE,
            );
            connection_number += 1;
        }

        next_frame().await
    }
}
