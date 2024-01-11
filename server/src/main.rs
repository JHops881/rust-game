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

use bincode::{deserialize, serialize};
use constants::ZERO_ZERO;
use enemy_character::{EnemyCharacter, EnemyType};

use entity_factory::EntityFactory;
use game_world::GameWorld;

use macroquad::{
    color::BLACK,
    math::Vec2,
    window::{clear_background, next_frame},
};
use net::send_game_state_to_connected_players;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//////////////////////////////////// CODE /////////////////////////////////////

#[macroquad::main("Game Server GUI")]
async fn main() {
    let mut game_world: GameWorld = GameWorld::new();

    let mut connection_table: HashMap<Uuid, String> = HashMap::new();

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
            //let result = recieve_net_message(&socket);

            // process net data

            // update new game state
            game_world.fixed_update(target_time_frame as f32);

            send_game_state_to_connected_players(&socket, &connection_table, &game_world);

            accumulator -= target_time_frame;
        }

        clear_background(BLACK);
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
