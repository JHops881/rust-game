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

/// pocedure to send client information to server
pub fn send_net_message(socket: &UdpSocket, data: &ClientToServerMessage, destination_addr: &str) {
    // serialize and encode
    let serialized_data = serialize(&data).expect("failed to serialize.");
    // send out
    let _ = socket.send_to(&serialized_data, destination_addr);
}

/// pocedure to server information to client (ONLY EVER 1476 BYTES MAX)
pub fn recieve_net_data(socket: &UdpSocket) -> Option<[u8; 1476]> {
    // buffer with the max size of how many bytes to read in from a datagram.
    let mut buf = [0; 1476];
    // receive
    let result: io::Result<(usize, SocketAddr)> = socket.recv_from(&mut buf);
    match result {
        Ok(_) => Some(buf),
        Err(_) => None,
    }
}

pub fn proccess_net_data(ser_msg: &[u8; 1476], graphics: &mut HashMap<Uuid, EntityGraphic>) {
    let message: ServerToClientMessage =
        deserialize::<ServerToClientMessage>(ser_msg).expect("Failed to deserialize!");
    match message {
        ServerToClientMessage::EntityData { data } => {
            for entity in data.iter() {
                let is_cached: bool = graphics.contains_key(&entity.id);
                match is_cached {
                    true => {
                        let old_state = graphics
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
                    false => {
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
        ServerToClientMessage::PlayerData { data } => (),
    }
}
