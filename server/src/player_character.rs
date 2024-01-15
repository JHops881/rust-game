////////////////////////////////// IMPORTS ////////////////////////////////////

use std::net::{IpAddr, Ipv4Addr};

use macroquad::{
    input::{is_key_down, is_key_pressed, is_key_released},
    math::Vec2,
    miniquad::KeyCode,
};
use netlib::EntityType;
use uuid::Uuid;

use crate::{
    entity_factory::EntityFactory,
    game_world::{self, GameWorld},
    player_projectile::PlayerProjectile,
    spell::Spell,
};

//////////////////////////////////// CODE /////////////////////////////////////

// Star of the show!
pub struct PlayerCharacter {
    id: Uuid,
    name: String,

    position: Vec2, // euclidian coordinates in the game world

    speed_stat: f32,     // jogging speed
    health_stat: f32,    // max health
    mana_stat: f32,      // max mana
    power_stat: f32,     // attack power
    vitality_stat: f32,  // health regen in hp/s
    wisdom_stat: f32,    // mana regen   in mana/s
    dexterity_stat: f32, // attack speed  attacks/s
    defense_stat: f32,   // flat damage reduction

    current_speed: f32, // in m/s
    current_health: f32,
    current_mana: f32,

    is_dead: bool,
    is_sprinting: bool,

    basic_cost: f32,         // mana cost of basic attack
    kenetic_pulse_cost: f32, // ... kenetic_pulse
    lightning_cost: f32,

    basic_power_multi: f32,         // a value that deterimes how much damage
    kenetic_pulse_power_multi: f32, // it will do. It's a multiplier.
    lightning_power_multi: f32,     // damage = power_stat * value

    movement: Option<Vec2>,  // unit vector of direction
    shootment: Option<Vec2>, // world coord
}

impl PlayerCharacter {
    /// call this every fixed_update to update the state of a player instance
    pub fn update(&mut self, delta_time: f32) {

        match self.get_movement() {
            Some(direction) => self.translate(direction, delta_time),
            None => (),
        }

        //// attacking
        //if is_mouse_button_pressed(MouseButton::Left) {

        //    // vector containing mouse position on screen in pixels
        //    let (x, y) = mouse_position();

        //    // where the click was but now in the world coordinates
        //    let clicked_pos: Vec2 = convert_to_world_coords(Vec2{x,y}, &self, TILE_WIDTH);

        //    // where the click happened, but now relative to the player (world coords)
        //    let vec_from_player: Vec2 = clicked_pos - self.get_position();

        //    // divided by magnitude to get unit vector
        //    let d: Vec2 = vec_from_player / vec_from_player.x.hypot(vec_from_player.y);

        //    self.cast_spell(Spell::Basic, d);
        //}
    }

    /// Default Constructor | Get a fresh player character.
    pub fn new(name: String, id: Uuid) -> PlayerCharacter {
        let initial_speed: f32 = 2.68;

        PlayerCharacter {
            id,
            name,

            position: Vec2 { x: 0.0, y: 0.0 },

            speed_stat: initial_speed,
            health_stat: 100.0,
            mana_stat: 100.0,
            power_stat: 10.0,
            vitality_stat: 1.0,
            wisdom_stat: 2.5,
            dexterity_stat: 2.0,
            defense_stat: 0.0,

            current_speed: initial_speed,
            current_health: 1.0,
            current_mana: 1.0,

            is_dead: false,
            is_sprinting: false,

            basic_cost: 0.0,
            kenetic_pulse_cost: 10.0,
            lightning_cost: 30.0,

            basic_power_multi: 1.0,
            kenetic_pulse_power_multi: 2.0,
            lightning_power_multi: 5.0,

            movement: None,
            shootment: None,
        }
    }

    // TODO: A fix is needed. See Trello. Remove this when resolved.
    /// Construct a Player Character from saved data
    pub fn new_from_saved() {}

    pub fn get_uuid(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> String {
        // String does not impl Copy or Clone
        let string: String = String::from(self.name.as_str());
        return string;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_speed_stat(&self) -> f32 {
        self.speed_stat
    }

    pub fn get_health_stat(&self) -> f32 {
        self.health_stat
    }

    pub fn get_mana_stat(&self) -> f32 {
        self.mana_stat
    }

    pub fn get_power_stat(&self) -> f32 {
        self.power_stat
    }

    pub fn get_vitality_stat(&self) -> f32 {
        self.vitality_stat
    }

    pub fn get_wisdom_stat(&self) -> f32 {
        self.wisdom_stat
    }

    pub fn get_dexterity_stat(&self) -> f32 {
        self.defense_stat
    }

    pub fn get_defense_stat(&self) -> f32 {
        self.defense_stat
    }

    pub fn get_current_speed(&self) -> f32 {
        self.current_speed
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn is_sprinting(&self) -> bool {
        self.is_sprinting
    }

    pub fn get_basic_cost(&self) -> f32 {
        self.basic_cost
    }

    pub fn get_kenetic_pulse_cost(&self) -> f32 {
        self.kenetic_pulse_cost
    }

    pub fn get_lightning_cost(&self) -> f32 {
        self.lightning_cost
    }

    pub fn get_basic_multi(&self) -> f32 {
        self.basic_power_multi
    }

    pub fn get_kenetic_pulse_multi(&self) -> f32 {
        self.kenetic_pulse_power_multi
    }

    pub fn get_lightning_multi(&self) -> f32 {
        self.lightning_power_multi
    }

    pub fn get_entity_type(&self) -> EntityType {
        EntityType::PlayerCharacter
    }

    /// PlayerCharacter health getter
    pub fn get_health(&self) -> f32 {
        self.current_health
    }

    /// PlayerCharacter mana getter
    pub fn get_mana(&self) -> f32 {
        self.current_mana
    }

    pub fn get_movement(&self) -> Option<Vec2> {
        self.movement
    }
    
    pub fn get_shootment(&self) -> Option<Vec2> {
        self.shootment
    }

    pub fn set_movement(&mut self, direction__: Option<Vec2>) {
        self.movement = direction__;
    }

    pub fn set_shootment(&mut self, location__: Option<Vec2>) {
        self.shootment = location__;
    }

    /// Reutrns the mana cost of a spell for a player.
    pub fn get_mana_cost(&self, spell: &Spell) -> f32 {
        match spell {
            Spell::Basic => self.basic_cost,
            Spell::KeneticPulse => self.kenetic_pulse_cost,
            Spell::Lightning => self.lightning_cost,
        }
    }

    /// Returns the power of a spell for a player
    pub fn get_spell_power(&self, spell: &Spell) -> f32 {
        match spell {
            Spell::Basic => self.basic_power_multi,
            Spell::KeneticPulse => self.kenetic_pulse_power_multi,
            Spell::Lightning => self.lightning_power_multi,
        }
    }

    /// Use this procedure to move the PlayerCharacter around in the world.
    /// needs a unit vector as a direction to move in.
    pub fn translate(&mut self, unit_vector: Vec2, mut delta_time: f32) {
        delta_time /= 1000.0; // this is important because `delta_time` comes in in milliseconds

        self.position.x = self.position.x + unit_vector.x * self.current_speed * delta_time;
        self.position.y = self.position.y + unit_vector.y * self.current_speed * delta_time;
    }

    /// toggles sprinting of player on
    pub fn begin_sprint(&mut self) {
        match self.is_sprinting {
            true => (),
            false => {
                self.is_sprinting = true;
                self.current_speed = 2.0 * self.speed_stat;
            }
        }
    }

    /// toggles sprinting of player off
    pub fn end_sprint(&mut self) {
        match self.is_sprinting {
            true => {
                self.is_sprinting = false;
                self.current_speed = self.speed_stat;
            }
            false => (),
        }
    }

    /// This will kill the player if it is too much damage.
    pub fn decrease_health(&mut self, amount: f32) {
        if self.current_health - amount > 0.0 {
            self.current_health = self.current_health - amount;
        } else {
            self.current_health = 0.0;
            self.is_dead = true;
        }
    }

    /// Function will safely handle amounts exceeding max health.
    pub fn increase_health(&mut self, amount: f32) {
        if self.current_health + amount < self.health_stat {
            self.current_health = self.current_health + amount;
        } else {
            self.current_health = self.health_stat;
        }
    }

    /// Safely decreases (will stop at 0.0)
    pub fn decrease_mana(&mut self, amount: f32) {
        if self.current_mana - amount > 0.0 {
            self.current_mana = self.current_mana - amount;
        } else {
            self.current_mana = 0.0;
        }
    }
    /// Function will safely handle amounts exceeding max mana
    pub fn increase_mana(&mut self, amount: f32) {
        if self.current_mana + amount < self.mana_stat {
            self.mana_stat = self.mana_stat + amount;
        } else {
            self.current_mana = self.mana_stat;
        }
    }

    /// Safely handles an attempt to cast a spell. Returns whether or
    /// not the player character has sufficient mana to cast the spell.
    pub fn can_cast(&self, spell: &Spell) -> bool {
        if self.current_mana >= self.get_mana_cost(spell) {
            return true;
        } else {
            return false;
        }
    }

    /// Call this when a spell is needed to be cast, like on a mouse event or key input.
    /// Let it know the spell you want to cast and the environment in which the projectile will be added to.
    pub fn cast_spell(&mut self, spell: Spell, direction: Vec2, game_world: &mut GameWorld) {
        match self.can_cast(&spell) {
            true => {
                self.decrease_mana(self.get_mana_cost(&spell));
                match spell {
                    Spell::Basic => {
                        EntityFactory::create_basic(self.get_position(), direction, game_world)
                    }
                    Spell::KeneticPulse => EntityFactory::create_kenetic_pulse(
                        self.get_position(),
                        direction,
                        game_world,
                    ),
                    Spell::Lightning => {
                        EntityFactory::create_lightning(self.get_position(), direction, game_world)
                    }
                }
            }
            false => (),
        }
    }
}
