pub mod entity_component_system;

use std::{time::{UNIX_EPOCH, SystemTime}, collections::HashMap};

use entity_component_system::Location;
use glam::DVec2;
use serde::{Deserialize, Serialize};

/// types of actions that a client can produce
#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Move { x: f64, y: f64 },
    Shoot { x: f64, y: f64 },
}

/// types of events that are needed in a connection
#[derive(Debug, Serialize, Deserialize)]
pub enum Connection {
    Join,
    KeepAlive,
    Drop,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientToServerMessage {
    ActionEvent { event: Action },
    ConnectionEvent { event: Connection },
}

impl ClientToServerMessage {
    pub fn package_action_shoot(mouse_world_pos: DVec2) -> Self {
        Self::ActionEvent {
            event: Action::Shoot {
                x: mouse_world_pos.x,
                y: mouse_world_pos.y,
            },
        }
    }

    pub fn package_action_move(direction: Vec2) -> Self {
        Self::ActionEvent {
            event: Action::Move {
                x: direction.x,
                y: direction.y,
            },
        }
    }

    pub fn package_connection_join() -> Self {
        Self::ConnectionEvent { event: Connection::Join }
    }

    pub fn package_connection_drop() -> Self {
        Self::ConnectionEvent { event: Connection::Drop }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ServerToClientMessage {
    LocationData { data: HashMap<i64, Location> },
}

impl ServerToClientMessage {
    pub fn package__data(data: Vec<Entity>) -> Self {
        Self::EntityData { data }
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