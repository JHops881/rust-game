pub mod global_variables;
pub mod graphics_math;
pub mod graphics_procedure;
pub mod graphics_entity;
pub mod main_character_singleton;
pub mod server_update_object;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::{time::{UNIX_EPOCH, SystemTime}, net::SocketAddr, io};

use global_variables::{MAIN_CHARACTER_INSTANCE, ZERO_FLOAT};
use graphics_entity::{GraphicsEntity, GameEntity};
use macroquad:: {miniquad::{window::set_fullscreen, KeyCode},
    window:: {
        clear_background,
        next_frame
    }, 
    color::BLACK, math::Vec2, input::{is_key_down, is_key_pressed},
};
use server_update_object::ServerUpdateObject;
use std::net::UdpSocket;
use bincode::{serialize, deserialize};


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

    /* ============ NETWORK ============ */

    // let mut connection_ipv4: String = String::new();
    // let mut session_id:      String = String::new();

    let server_addr: &str = "127.0.0.1:42110";

    // bind to any available local port
    let socket = UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");
    dbg!(&socket);
    socket.set_nonblocking(true).expect("failed to make socket non-blocking");

    let join_message: &str = "<JOIN>";
    send_net_message(&socket, join_message, server_addr);

    // To enable linear interpolation, the client must always be one update behind the server.
    // It stores it in next_server_update and uses the position values to interpolate the
    // the positions of the graphics entities on the client's side. 
    let mut this_server_update: ServerUpdateObject;
    let mut next_server_update: ServerUpdateObject;
    // must initialize. Ignore. 
    this_server_update = ServerUpdateObject {
        x: ZERO_FLOAT,
        y: ZERO_FLOAT,
    };
    next_server_update = ServerUpdateObject {
        x: ZERO_FLOAT,
        y: ZERO_FLOAT,
    };

    /* ======== WINDOW SETTINGS ======== */

    set_fullscreen(true);

    /* ============ GRAPHICS =========== */

    
    let mut graphics_entities: Vec<GraphicsEntity> = Vec::new();
    graphics_entities.push(
        GraphicsEntity { 
            this_world_pos: Vec2 {
                x: ZERO_FLOAT,
                y: ZERO_FLOAT,
            },
            next_world_pos: Vec2 {
                x: ZERO_FLOAT,
                y: ZERO_FLOAT,
            },
            entity_type: GameEntity::Ghoul,
        }
    );


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

            
            if is_key_down(KeyCode::G) {
                let data: &str = "<ACT: U>";
                send_net_message(&socket, data, server_addr);
                
            }
            

            // ######### RECEIVE SERVER UPDATE TO CLIENT ######### //

            let msg_buf: Option<[u8; 1476]> = recieve_net_message(&socket);
            match msg_buf {
                Some(msg) => {
                    // cache incoming update in `next_server_update` and take the past cached
                    // update and make it the current. We do this to enable linear interpolation.
                    this_server_update = next_server_update;
                    next_server_update = ServerUpdateObject {
                        x: ZERO_FLOAT,
                        y: proccess_net_message(msg).parse::<f32>().expect("failed to parse into f32"),
                    }
                },
                None => (),
            }
            
            // ######### UPDATE CLIENT ACCOARDING TO STATE ######### //

            graphics_entities[0].this_world_pos = Vec2{ x: this_server_update.x, y: this_server_update.y };
            graphics_entities[0].next_world_pos = Vec2{ x: next_server_update.x, y: next_server_update.y };
    

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
            g_e.draw(t);
        }

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
fn recieve_net_message(socket: &UdpSocket) -> Option<[u8; 1476]> {
    // buffer with the max size of how many bytes to read in from a datagram.
    let mut buf = [0; 1476];
    // receive
    let result: io::Result<(usize, SocketAddr)> = socket.recv_from(&mut buf);
    match result {
        Ok(_) => Some(buf),
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
