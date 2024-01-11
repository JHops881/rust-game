use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use bincode::{deserialize, serialize};
use netlib::{ClientToServerMessage, ServerToClientMessage};

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

pub fn proccess_net_data(ser_msg: [u8; 1476], graphics: &mut Vec<EntityGraphic>) {
    let message: ServerToClientMessage =
        deserialize::<ServerToClientMessage>(&ser_msg).expect("Failed to deserialize!");
    match message {
        ServerToClientMessage::EntityData { data } => (),
        ServerToClientMessage::PlayerData { data } => (),
    }
}
