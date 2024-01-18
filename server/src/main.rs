use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::Sender,
    time::{SystemTime, UNIX_EPOCH},
};

use macroquad::{
    color::{BLACK, BLUE, WHITE},
    input::{is_key_down, is_key_pressed},
    math::{DVec2, Vec2},
    text::draw_text,
    window::{clear_background, next_frame},
};

use netlib::{
    entity_component_system::{create_new_player, movement_system, EntityDatabase, Movement},
    system_time,
};

//////////////////////////////////// CODE /////////////////////////////////////

#[macroquad::main("Game Server GUI")]
async fn main() {
    let mut id_counter: i64 = 0;
    let mut entity_db: EntityDatabase = EntityDatabase::new();

    create_new_player(&mut id_counter, &mut entity_db);

    // let mut connection_table: HashMap<String, Uuid> = HashMap::new();

    // listen for information on port 42110
    let socket = UdpSocket::bind("127.0.0.1:42110").expect("couldn't bind to address");
    socket
        .set_nonblocking(true)
        .expect("failed to make socket non-blocking");

    let mut real_delta_time: f64;
    let mut last_update_time: f64 = system_time();
    let target_time_frame: f64 = 15.625; // 64 tick
    let dt_special: f64 = target_time_frame / 1000.0;
    let mut accumulator: f64 = 0.0;

    // While game is running...
    loop {
        // delta time calculations
        real_delta_time = system_time() - last_update_time;
        last_update_time += real_delta_time;
        accumulator += real_delta_time;

        while accumulator > target_time_frame {
            /* UPDATE */

            // // ######### RECEIVE CLIENTS' DATA ######### //
            // let mut new_serialized_messages: Vec<([u8; 1476], SocketAddr)> = Vec::new();
            // loop {
            //     let pair: Option<([u8; 1476], SocketAddr)> = recieve_client_message(&socket);
            //     match pair {
            //         Some(data) => {
            //             new_serialized_messages.push(data);
            //         }
            //         None => break,
            //     }
            // }

            // // process net messages
            // for (ser_msg, sender) in new_serialized_messages.iter() {
            //     proccess_client_message(ser_msg, *sender, &mut connection_table, &mut game_world);
            // }
            //
            // // update new game state
            // game_world.fixed_update(target_time_frame as f32);
            //
            // send_game_state_to_connected_players(&socket, &connection_table, &game_world);

            if is_key_down(macroquad::miniquad::KeyCode::D) {
                entity_db.movement.insert(
                    1,
                    Movement {
                        velocity: DVec2 { x: 1.0, y: 0.0 },
                        acceleration: 0.0,
                    },
                );
            } else {
                entity_db.movement.insert(
                    1,
                    Movement {
                        velocity: DVec2 { x: 0.0, y: 0.0 },
                        acceleration: 0.0,
                    },
                );
            }

            movement_system(&entity_db.movement, &mut entity_db.location, dt_special);

            dbg!(entity_db.movement.get(&1).unwrap().velocity);
            dbg!(entity_db.location.get(&1).unwrap().position);

            accumulator -= target_time_frame;
        }

        clear_background(BLACK);

        // // display connections
        // let font_size: f32 = 30.0;
        // let newline_offset: f32 = 30.0; // all in pixels
        // let x_pos: f32 = 12.0;
        // let y_pos: f32 = 25.0;
        //
        // draw_text("CURRENT CONNECTIONS:", x_pos, y_pos, font_size, BLUE);
        //
        // let mut connection_number: i32 = 1;
        // for (address, id) in connection_table.iter() {
        //     let connection_num_y_pos: f32 = y_pos + connection_number as f32 * newline_offset;
        //     draw_text(
        //         (id.to_string() + ", " + address.as_str()).as_str(),
        //         x_pos,
        //         connection_num_y_pos,
        //         font_size,
        //         WHITE,
        //     );
        //     connection_number += 1;
        // }

        next_frame().await
    }
}
