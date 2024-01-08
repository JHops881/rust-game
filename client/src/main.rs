pub mod global_variables;
pub mod graphics_math;
pub mod graphics_procedure;
pub mod entity_graphic;
pub mod main_character_singleton;
pub mod server_update_object;
pub mod entity_type;

////////////////////////////////// IMPORTS ////////////////////////////////////

use std::{time::{UNIX_EPOCH, SystemTime}, net::SocketAddr, io};

use global_variables::{MAIN_CHARACTER_INSTANCE, ZERO_FLOAT};
use entity_graphic::EntityGraphic;
use macroquad:: {miniquad::{window::set_fullscreen, KeyCode},
    window:: {
        clear_background,
        next_frame
    }, 
    color::BLACK, math::Vec2, input::is_key_down,
};
use server_update_object::ServerUpdateObject;
use std::net::UdpSocket;
use bincode::{serialize, deserialize};

use crate::entity_type::EntityType;


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

            
            if is_key_down(KeyCode::G) {
                let data: &str = "<ACT: U>";
                send_net_message(&socket, data, server_addr);
                
            }
            

            // ######### RECEIVE SERVER UPDATE TO CLIENT ######### //
            let mut msg_buf: [u8; 1476] = [0; 1476];
            let mut new_update: bool = false;
            let msg_buf_option: Option<[u8; 1476]> = recieve_net_data(&socket);
            match msg_buf_option {
                Some(data) => {
                    msg_buf = data;
                    new_update = true;
                },
                None => (),
            }
            
            // ######### UPDATE CLIENT ACCOARDING TO STATE ######### //

            if new_update {
                proccess_net_data(msg_buf,);
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
fn recieve_net_data(socket: &UdpSocket) -> Option<[u8; 1476]> {
    // buffer with the max size of how many bytes to read in from a datagram.
    let mut buf = [0; 1476];
    // receive
    let result: io::Result<(usize, SocketAddr)> = socket.recv_from(&mut buf);
    match result {
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}

fn proccess_net_data(ser_msg: [u8; 1476], graphics: Vec<EntityGraphic>) {
    let first_byte: &[u8; 1] = &[ser_msg[0]; 1];
    let packet_code = deserialize::<&str>(first_byte).expect("failed to deserialize!");
    match packet_code
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
