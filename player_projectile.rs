////////////////////////////////// IMPORTS ////////////////////////////////////

use macroquad::math::Vec2;
use netlib::EntityType;
use uuid::Uuid;

use crate::{
    spell::Spell, system_time,
};

//////////////////////////////////// CODE /////////////////////////////////////

/// These things are the result of a player casting a spell. They fly through
/// the air and blow things up fr fr fr
pub struct PlayerProjectile {
    id: Uuid,

    expiriation_time: f64, // Epoch time in ms when it will be dropped
    is_expired: bool,      // all things come to an end.

    position: Vec2,  // Euclidian
    direction: Vec2, // unit vector that points in the direction its heading
    speed: f32,      // m/s

    spell_type: Spell, // we can get the power from this
}

impl PlayerProjectile {
    pub fn get_position(&self) -> Vec2 {
        // google wants to know your location.
        self.position
    }

    pub fn is_expired(&self) -> bool {
        self.is_expired
    }

    pub fn get_entity_type(&self) -> EntityType {
        match self.spell_type {
            Spell::Basic => EntityType::BasicAttack,
            Spell::KeneticPulse => EntityType::KeneticPulse,
            Spell::Lightning => EntityType::Lightning,
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.id
    }

    /// Call this when a player is casting a spell. Let it know the direction it is to go, the type
    /// of spell projectile it is. Pass the `direction` as a unit vector.
    pub fn new(position: Vec2, direction: Vec2, spell: Spell) -> PlayerProjectile {
        PlayerProjectile {
            id: Uuid::new_v4(),

            expiriation_time: system_time() + 3100.0, // 3.1 seconds after creation
            is_expired: false,

            position,
            direction,
            speed: 10.0,

            spell_type: spell,
        }
    }

    /// Call this every fixed game update.
    pub fn update(&mut self, delta_time: f32) {
        if self.expiriation_time > system_time() {
            self.translate(delta_time)
        } else {
            // Ideally, we would drop the damn thing right here. But it is
            // a tough cookie. Can't drop by reference at all. We need to
            // instead mark it for death and have someone or something else
            // drop it externally.
            // our Angel of Death (i mean `Environment`` instance) will take
            // it from here.
            self.is_expired = true; // ew, I hope it's not milk!
        }
    }

    /// Nothing fancy, just a way to move the projectile.
    pub fn translate(&mut self, mut delta_time: f32) {
        delta_time /= 1000.0;

        self.position.x += self.direction.x * self.speed * delta_time;
        self.position.y += self.direction.y * self.speed * delta_time;
    }
}
