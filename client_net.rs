use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
};

use bincode::{deserialize, serialize};
use macroquad::math::Vec2;
use netlib::{ClientToServerMessage, ServerToClientMessage};
use uuid::Uuid;

use crate::entity_graphic::EntityGraphic;

pub fn send_client_message(
    socket: &UdpSocket,
    data: &ClientToServerMessage,
    destination_addr: &str,
) {
    // serialize and encode
    let serialized_data = serialize(&data).expect("failed to serialize.");
    // send out
    let _ = socket.send_to(&serialized_data, destination_addr);
}

/// pocedure to server message to client (ONLY EVER 1476 BYTES MAX)
/// only returns an unserialized chunk of bytes
pub fn seceive_server_message(socket: &UdpSocket) -> Option<[u8; 1476]> {
    // buffer with the max size of how many bytes to read in from a datagram.
    let mut buf = [0; 1476];
    // receive
    let result: io::Result<(usize, SocketAddr)> = socket.recv_from(&mut buf);
    match result {
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}

/// changes the clients state based on the message
pub fn proccess_server_message(ser_msg: &[u8; 1476], graphics: &mut HashMap<Uuid, EntityGraphic>) {
    // deserialize
    let message: ServerToClientMessage =
        deserialize::<ServerToClientMessage>(ser_msg).expect("Failed to deserialize!");

    match message {
        ServerToClientMessage::EntityData { data } => {
            for entity in data.iter() {
                // check if we already have in on the client
                let is_cached: bool = graphics.contains_key(&entity.id);
                match is_cached {
                    true => {
                        let old_state = graphics // change an existing entity graphic.
                            .get(&entity.id)
                            .expect("Somehow, a graphic is missing a value!");
                        let _ = graphics.insert(
                            entity.id,
                            EntityGraphic::new( 
                                Vec2 {
                                    x: old_state.get_next_x(),
                                    y: old_state.get_next_y(),
                                },
                                Vec2 {
                                    x: entity.x,
                                    y: entity.y,
                                },
                                old_state.get_entity_type(),
                            ),
                        );
                    }
                    false => { // make a new one, we dont have this one yet!
                        let _ = graphics.insert(
                            entity.id,
                            EntityGraphic::new(
                                Vec2 {
                                    x: entity.x,
                                    y: entity.y,
                                },
                                Vec2 {
                                    x: entity.x,
                                    y: entity.y,
                                },
                                entity.entity_type,
                            ),
                        );
                    }
                }
            }
        }
        ServerToClientMessage::PlayerData { data } => (), // TODO: make
    }
}