/////////////////////////////// IMPORTS ////////////////////////////////////

use glam::DVec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/////////////////////////////// TYPES ////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub enum Item {
    Stick,
    Rock,
    Yarn,
    Frog,
    HealthPotion,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TeamAffilition {
    Neutral,
    Player,
    Enemy,
}

/////////////////////////////// COMPONENTS ////////////////////////////////////
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub position: DVec2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Movement {
    pub velocity: DVec2,
    pub acceleration: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mass {
    pub mass: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Health {
    pub max: f64,
    pub current: f64,
    pub regen_rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mana {
    pub max: f64,
    pub current: f64,
    pub regen_rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Defense {
    pub damage_reduction: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub affiliation: TeamAffilition,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    pub power: f64,
    pub rate: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub inventory: Vec<(i32, Item)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Graphical {
    pub radius: f64,
}

//////////////////////////// ENTITY DATABASE /////////////////////////////////

/// Entity Database contains all elements in the game in tables
/// of components. Entities are abstract objects compose of components
/// linked together by a single unique id.
///
/// Think of the components as attributes that an entity might have.
pub struct EntityDatabase {
    pub location: HashMap<i64, Location>,
    pub movement: HashMap<i64, Movement>,
    pub mass: HashMap<i64, Mass>,
    pub health: HashMap<i64, Health>,
    pub mana: HashMap<i64, Mana>,
    pub defense: HashMap<i64, Defense>,
    pub team: HashMap<i64, Team>,
    pub attack: HashMap<i64, Attack>,
    pub container: HashMap<i64, Container>,
    pub graphical: HashMap<i64, Graphical>,
}
impl EntityDatabase {
    pub fn new() -> Self {
        EntityDatabase {
            location: HashMap::new(),
            movement: HashMap::new(),
            mass: HashMap::new(),
            health: HashMap::new(),
            mana: HashMap::new(),
            defense: HashMap::new(),
            team: HashMap::new(),
            attack: HashMap::new(),
            container: HashMap::new(),
            graphical: HashMap::new(),
        }
    }
}

///////////////////////////// ENTITY FACTORY //////////////////////////////////

/// used on the server side
pub fn create_new_player(id_counter: &mut i64, entity_db: &mut EntityDatabase) {
    *id_counter += 1;
    let id: i64 = *id_counter;

    /*
    [X] Location,
    [X] Movement,
    [X] Mass,
    [X] Health,
    [X] Mana,
    [X] Defense,
    [X] Team,
    [X] Attack,
    [ ] Container, TODO
    [X] Graphical,
    */

    entity_db.location.insert(
        id,
        Location {
            position: DVec2 { x: 0.0, y: 0.0 },
        },
    );
    entity_db.movement.insert(
        id,
        Movement {
            velocity: DVec2 { x: 0.0, y: 0.0 },
            acceleration: 0.0,
        },
    );
    entity_db.mass.insert(id, Mass { mass: 80.0 });
    entity_db.health.insert(
        id,
        Health {
            max: 100.0,
            current: 100.0,
            regen_rate: 1.0,
        },
    );
    entity_db.mana.insert(
        id,
        Mana {
            max: 100.0,
            current: 100.0,
            regen_rate: 1.0,
        },
    );
    entity_db.defense.insert(
        id,
        Defense {
            damage_reduction: 0.0,
        },
    );
    entity_db.team.insert(
        id,
        Team {
            affiliation: TeamAffilition::Player,
        },
    );
    entity_db.attack.insert(
        id,
        Attack {
            power: 10.0,
            rate: 1.0,
        },
    );
    entity_db.graphical.insert(
        id,
        Graphical {
            radius: 8.0,
        },
    );
}

/// used on the client side
pub fn create_drawable(id: i64, loc: Location, gfx: Graphical, entity_db: &mut EntityDatabase) {
    entity_db.location.insert(id, loc);
    entity_db.graphical.insert(id, gfx);
}

//////////////////////////////// SYSTEMS /////////////////////////////////////

/// __Depends On:__
/// + Database `location` components,
/// + Database `movement` components,
///   
/// Queries:
/// + `Movement`
/// + `Location`
///
/// Mutates:
/// + `Location`
/// 
/// __Usage:__ call on the game server once every fixed update. Simulates moving  
/// entities by advancing their locations as dictated by the properties of their  
/// movemnent e.g. velocity and acceleration.
pub fn movement_system(
    movement: &HashMap<i64, Movement>,
    location: &mut HashMap<i64, Location>,
    delta_time: f64,
) {
    for (id, movement_data) in movement.iter() {
        match location.get(id) {
            Some(location_data) => {
                location.insert(
                    *id,
                    Location {
                        position: location_data.position + movement_data.velocity * delta_time,
                    },
                );
            }
            None => (),
        }
    }
}
