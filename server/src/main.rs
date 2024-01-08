pub mod player_character;

pub mod player_projectile;

pub mod enemy_character;

pub mod entity_factory;

pub mod environment_singleton;

pub mod global_variables;




////////////////////////////////// IMPORTS ////////////////////////////////////


use std::{time::{SystemTime, UNIX_EPOCH}, net::{UdpSocket, SocketAddr}, io};

use bincode::{serialize, deserialize};
use enemy_character:: {
    EnemyCharacter,
    EnemyType
};

use entity_factory::EntityFactory;
use global_variables::ENVIRONMENT_INSTANCE;

use macroquad::{math::Vec2, window::{clear_background, next_frame}, color::BLACK};


//////////////////////////////////// CODE /////////////////////////////////////

#[macroquad::main("Game Server GUI")]   
async fn main() {

    let mut connection_table: String = String::new();

    // bind to any available local port
    let socket = UdpSocket::bind("127.0.0.1:42110").expect("couldn't bind to address");
    socket.set_nonblocking(true).expect("failed to make socket non-blocking");

    // initalize an enemy
    EntityFactory::create_ghoul(Vec2 {x: 0.0, y: 0.0});

    // gme loop crap
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

            // get network message -> interperet it -> update game state
            let result = recieve_net_message(&socket);
            match result {
                Some((ser_msg, src)) => {
                    let message: String = proccess_net_message(ser_msg);
                    match message.as_str() {
                        "<ACT: U>" => {
                            let result = ENVIRONMENT_INSTANCE.lock();
                            match result {
                            Ok(mut env_inst) => env_inst.enemy_characters[0].translate(Vec2 {x: 0.0, y: 1.0}, target_time_frame as f32),
                            Err(poisoned)    => panic!("Mutex is poisoned: {:?}", poisoned),
                            }
                        },
                        "<JOIN>"   => {
                            connection_table = src.to_string();
                            println!("Joined!");
                            dbg!(&connection_table);
                        },
                        _ => (),
                    }
                },
                None => (),
            }

            /* UPDATE */
            
            let result = ENVIRONMENT_INSTANCE.lock();
            match result {
                Ok(mut env_inst) => env_inst.fixed_update(target_time_frame as f32), // this is it right here
                Err(poisoned)    => panic!("Mutex is poisoned: {:?}", poisoned),
            }

            let mut ghoul_pos: String = String::new();

            let result = ENVIRONMENT_INSTANCE.lock();
            match result {
                Ok(env_inst) => ghoul_pos = env_inst.enemy_characters[0].get_position().y.to_string(),
                Err(poisoned)    => panic!("Mutex is poisoned: {:?}", poisoned),
            }

            send_net_message(&socket, ghoul_pos.as_str(), connection_table.as_str());
            
            accumulator -= target_time_frame;

        }

        clear_background(BLACK);
        next_frame().await
    }
    
}


/// pocedure to send client information to server
fn send_net_message(socket: &UdpSocket, data: &str, destination_addr: &str) {
    // serialize and encode
    let serialized_data = serialize(&data).expect("failed to serialize.");
    // send out
    let result = socket.send_to(&serialized_data, destination_addr);
    match result {
        Ok(_) => (),
        Err(_) => (),
    }
}

/// pocedure to server information to client (ONLY EVER 1476 BYTES MAX)
fn recieve_net_message(socket: &UdpSocket) -> Option<([u8; 1476], SocketAddr)> {
    // buffer with the max size of how many bytes to read in from a datagram.
    let mut buf = [0; 1476];
    // receive
    let result: io::Result<(usize, SocketAddr)> = socket.recv_from(&mut buf);
    match result {
        Ok((_, src)) => Some((buf, src)),
        Err(_) => None,
    }
}

fn proccess_net_message(ser_msg: [u8; 1476]) -> String {
    let result = deserialize::<&str>(&ser_msg);
    return match result {
        Ok(data) => data.to_string(),
        Err(_) => panic!("FAILED TO DESERIALIZE"),
    }; 
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
