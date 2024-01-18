pub mod entity_component_system;

use std::time::{UNIX_EPOCH, SystemTime};

use macroquad::prelude::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// This represents each unique tangible animate or inanimate body that exists in the game.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum EntityType {
    PlayerCharacter,

    BasicAttack,
    KeneticPulse,
    Lightning,

    ArmsDealer,

    // Enemy Character Types
    Ghoul,
    Phantom,
    Drinker,
    Crawler,

    // Dropped Item Types
    Coin,
    LootBag,
}

/// anything that exists in the game
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: Uuid,
    pub x: f32,
    pub y: f32,
    pub entity_type: EntityType,
}

/// types of actions that a client can produce
#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Move { x: f32, y: f32 },
    Shoot { x: f32, y: f32 },
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
    pub fn package_action_shoot(mouse_world_pos: Vec2) -> Self {
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
pub struct PlayerData {
    pub x: f32,
    pub y: f32,

    pub id: Uuid,
    pub name: String,

    pub speed_stat: f32,
    pub health_stat: f32,
    pub mana_stat: f32,
    pub power_stat: f32,
    pub vitality_stat: f32,
    pub wisdom_stat: f32,
    pub dexterity_stat: f32,
    pub defense_stat: f32,

    pub current_speed: f32,
    pub current_health: f32,
    pub current_mana: f32,

    pub is_dead: bool,
    pub is_sprinting: bool,

    pub basic_cost: f32,
    pub kenetic_pulse_cost: f32,
    pub lightning_cost: f32,

    pub basic_power_multi: f32,
    pub kenetic_pulse_power_multi: f32,
    pub lightning_power_multi: f32,
}

impl PlayerData {
    pub fn new_default() -> Self {
        let zero: f32 = 0.0;
        PlayerData {
            x: zero,
            y: zero,
            id: Uuid::nil(),
            name: String::new(),
            speed_stat: zero,
            health_stat: zero,
            mana_stat: zero,
            power_stat: zero,
            vitality_stat: zero,
            wisdom_stat: zero,
            dexterity_stat: zero,
            defense_stat: zero,
            current_speed: zero,
            current_health: zero,
            current_mana: zero,
            is_dead: false,
            is_sprinting: false,
            basic_cost: zero,
            kenetic_pulse_cost: zero,
            lightning_cost: zero,
            basic_power_multi: zero,
            kenetic_pulse_power_multi: zero,
            lightning_power_multi: zero
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerToClientMessage {
    PlayerData { data: PlayerData },
    EntityData { data: Vec<Entity> },
}

impl ServerToClientMessage {
    pub fn package_player_data(data: PlayerData) -> Self {
        Self::PlayerData { data }
    }

    pub fn package_entity_data(data: Vec<Entity>) -> Self {
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