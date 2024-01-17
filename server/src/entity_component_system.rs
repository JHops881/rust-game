/////////////////////////////// IMPORTS ////////////////////////////////////

use macroquad::math::DVec2;
use std::{collections::HashMap, fs::copy};

/////////////////////////////// TYPES ////////////////////////////////////

pub enum Item {
    Stick,
    Rock,
    Yarn,
    Frog,
    HealthPotion,
}

pub enum TeamAffilition {
    Neutral,
    Player,
    Enemy,
}

/////////////////////////////// COMPONENTS ////////////////////////////////////

pub struct Location {
    pub position: DVec2,
}

pub struct Movement {
    pub velocity: DVec2,
    pub acceleration: f64,
}

pub struct Mass {
    pub mass: f64,
}

pub struct Health {
    pub max: f64,
    pub current: f64,
    pub regen_rate: f64,
}

pub struct Mana {
    pub max: f64,
    pub current: f64,
    pub regen_rate: f64,
}

pub struct Defense {
    pub damage_reduction: f64,
}

pub struct Team {
    pub affiliation: TeamAffilition,
}

pub struct Attack {
    pub power: f64,
    pub rate: f64,
}

pub struct Container {
    pub inventory: Vec<(i32, Item)>,
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
}

///////////////////////////// ENTITY FACTORY //////////////////////////////////

pub fn create_new_player(id_counter: &mut i64, entity_db: &mut EntityDatabase) {
    *id_counter += 1;
    let id: i64 = *id_counter;

    /* A player will have a location, movement, mass, health,
    mana, defense, team, attack, and a container component. */

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
}

//////////////////////////////// SYSTEMS /////////////////////////////////////

/// __Depends On:__
/// + `location` component table,
/// + `movement` component table,
/// + `mass` component table,
///   
/// Queries:
/// + `Movement`
/// + `Mass`
/// 
/// Mutates:
/// + `Location`
pub fn player_movement_system() {}
