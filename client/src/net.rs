use std::{net::{SocketAddr, UdpSocket}, io};

use bincode::{deserialize, serialize};

use crate::entity_graphic::EntityGraphic;

/// pocedure to send client information to server
pub fn send_net_message(socket: &UdpSocket, data: &str, destination_addr: &str) {
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

pub fn proccess_net_data(ser_msg: [u8; 1476], graphics: Vec<EntityGraphic>) {
    let first_byte: &[u8; 1] = &[ser_msg[0]; 1];
    let packet_code = deserialize::<&str>(first_byte).expect("failed to deserialize!");
    // match packet_code
}