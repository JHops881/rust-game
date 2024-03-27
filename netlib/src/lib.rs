pub mod entity_component_system;
pub mod system_time;

use entity_component_system::{
    Attack, Container, Defense, Graphical, Health, Location, Mana, Mass, Movement, Team,
};

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

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerToClientMessage {
    LocationData { data: Vec<(i64, Location)> },
    MovementData { data: Vec<(i64, Movement)> },
    MassData { data: Vec<(i64, Mass)> },
    HealthData { data: Vec<(i64, Health)> },
    ManaData { data: Vec<(i64, Mana)> },
    DefenseData { data: Vec<(i64, Defense)> },
    TeamData { data: Vec<(i64, Team)> },
    AttackData { data: Vec<(i64, Attack)> },
    ContainerData { data: Vec<(i64, Container)> },
    GraphicalData { data: Vec<(i64, Graphical)> },
}
