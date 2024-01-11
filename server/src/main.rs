pub mod player_character;

pub mod player_projectile;

pub mod enemy_character;

pub mod entity_factory;

pub mod game_world;

pub mod spell;

pub mod net;

pub mod constants;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
    time::{SystemTime, UNIX_EPOCH},
};

use constants::ZERO_ZERO;
use enemy_character::{EnemyCharacter, EnemyType};

use entity_factory::EntityFactory;
use game_world::GameWorld;

use macroquad::{
    color::{BLACK, BLUE, WHITE},
    math::Vec2,
    window::{clear_background, next_frame}, text::draw_text,
};
use net::{send_game_state_to_connected_players, recieve_net_message, proccess_net_message};
use netlib::system_time;
use uuid::Uuid;

//////////////////////////////////// CODE /////////////////////////////////////

#[macroquad::main("Game Server GUI")]
async fn main() {
    let mut game_world: GameWorld = GameWorld::new();

    let mut connection_table: HashMap<String, Uuid> = HashMap::new();

    // bind to any available local port
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

            // Get Net Data
            let result = recieve_net_message(&socket);

            // process net data
            match result {
                Some((ser_msg, sender)) => proccess_net_message(ser_msg, sender, &mut connection_table, &mut game_world),
                None => (),
            }

            // update new game state
            game_world.fixed_update(target_time_frame as f32);

            send_game_state_to_connected_players(&socket, &connection_table, &game_world);

            accumulator -= target_time_frame;
        }

        clear_background(BLACK);

        // display connections
        let font_size: f32 = 30.0;
        let newline_offset: f32 = 30.0;
        let x_pos: f32 = 12.0;
        let y_pos: f32 = 25.0;

        draw_text("CURRENT CONNECTIONS:", x_pos, y_pos, font_size, BLUE);
        
        let mut connection_number: i32 = 1;
        for (address, id) in connection_table.iter() {
            let connection_y_pos: f32 = y_pos + connection_number as f32 * newline_offset;
            draw_text((id.to_string() + ", " + address.as_str()).as_str(), x_pos, connection_y_pos, font_size, WHITE);
            connection_number += 1;
        }

        next_frame().await
    }
}
