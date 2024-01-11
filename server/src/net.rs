use std::{
    collections::HashMap,
    io,
    net::{SocketAddr, UdpSocket},
};

use bincode::{deserialize, serialize};
use netlib::{ClientToServerMessage, Connection, Entity, PlayerData, ServerToClientMessage};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{entity_factory::EntityFactory, game_world::GameWorld};

///////////////////////////////////////////////////////////////////////////////

pub fn send_game_state_to_connected_players(
    socket: &UdpSocket,
    connection_table: &HashMap<String, Uuid>,
    game_world: &GameWorld,
) {
    let mut entity_data: Vec<Entity> = Vec::new();

    /* TODO: FIX THIS!!!!
       An entity is 25 bytes and we can only send 1476 at a time.
       Need a way to break up the packets when sending more than
       58 entities.
    */
    let mut _entity_count: u8 = 0;
    for enemy in game_world.enemy_characters.iter() {
        entity_data.push(Entity {
            id: enemy.get_uuid(),
            x: enemy.get_position().x,
            y: enemy.get_position().y,
            entity_type: enemy.get_entity_type(),
        });
        _entity_count += 1;
    }
    for proj in game_world.player_projectiles.iter() {
        entity_data.push(Entity {
            id: proj.get_uuid(),
            x: proj.get_position().x,
            y: proj.get_position().y,
            entity_type: proj.get_entity_type(),
        });
        _entity_count += 1;
    }
    for player in game_world.player_characters.iter() {
        entity_data.push(Entity {
            id: player.get_uuid(),
            x: player.get_position().x,
            y: player.get_position().y,
            entity_type: player.get_entity_type(),
        });
        _entity_count += 1;
    }

    let message: ServerToClientMessage = ServerToClientMessage::package_entity_data(entity_data);
    let serialized_message: Vec<u8> = serialize(&message).expect("failed to serialize.");

    // send out to every connection on the server
    for (address, id) in connection_table {
        let _ = socket.send_to(&serialized_message, address);
        // we also need to send the correct player data to each connection for
        // each client's main character.
        for player in game_world.player_characters.iter() {
            if player.get_uuid() == *id {
                let player_data: PlayerData = PlayerData {
                    x: player.get_position().x,
                    y: player.get_position().y,
                    id: player.get_uuid(),
                    name: player.get_name(),
                    speed_stat: player.get_speed_stat(),
                    health_stat: player.get_health_stat(),
                    mana_stat: player.get_mana_stat(),
                    power_stat: player.get_power_stat(),
                    vitality_stat: player.get_vitality_stat(),
                    wisdom_stat: player.get_wisdom_stat(),
                    dexterity_stat: player.get_dexterity_stat(),
                    defense_stat: player.get_defense_stat(),
                    current_speed: player.get_current_speed(),
                    current_health: player.get_health(),
                    current_mana: player.get_mana(),
                    is_dead: player.is_dead(),
                    is_sprinting: player.is_sprinting(),
                    basic_cost: player.get_basic_cost(),
                    kenetic_pulse_cost: player.get_kenetic_pulse_cost(),
                    lightning_cost: player.get_lightning_cost(),
                    basic_power_multi: player.get_basic_multi(),
                    kenetic_pulse_power_multi: player.get_kenetic_pulse_multi(),
                    lightning_power_multi: player.get_lightning_multi(),
                };

                let message: ServerToClientMessage =
                    ServerToClientMessage::package_player_data(player_data);
                let serialized_message: Vec<u8> =
                    serialize(&message).expect("failed to serialize.");
                let _ = socket.send_to(&serialized_message, address);
            }
        }
    }
}

/// pocedure to server information to client (ONLY EVER 1476 BYTES MAX)
pub fn recieve_net_message(socket: &UdpSocket) -> Option<([u8; 1476], SocketAddr)> {
    // buffer with the max size of how many bytes to read in from a datagram.
    let mut buf = [0; 1476];
    // receive
    let result: io::Result<(usize, SocketAddr)> = socket.recv_from(&mut buf);
    match result {
        Ok((_, src)) => Some((buf, src)),
        Err(_) => None,
    }
}

/// TODO: DOC COMMENT
pub fn proccess_net_message(
    ser_msg: [u8; 1476],
    sender: SocketAddr,
    connection_table: &mut HashMap<String, Uuid>,
    game_world: &mut GameWorld,
) {
    let message = deserialize::<ClientToServerMessage>(&ser_msg).expect("Failed to deserialize!");
    match message {
        ClientToServerMessage::ActionEvent { event } => (),
        ClientToServerMessage::ConnectionEvent { event } => match event {
            Connection::Join => {
                let id: Uuid = Uuid::new_v4();
                EntityFactory::create_new_player_character(String::from("Joseph"), id, game_world);
                connection_table.insert(sender.to_string(), id);
            }
            Connection::KeepAlive => (),
            Connection::Drop => {
                connection_table.remove(&sender.to_string());
            },
        },
    }
}
