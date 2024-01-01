
use macroquad::math::Vec2;
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html
use macroquad::input::*;

use crate::{environment::Environment, player_projectile::PlayerProjectile, convert_to_world_coords, TILE_WIDTH};

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

// We will get to cast these later. 
pub enum Spell {
    Basic,
    KeneticPulse,
    Lightning,
}

// Star of the show!
pub struct PlayerCharacter {
    position: Vec2,    // euclidian coordinates in the game world
    speed: f32,        // current speed in m/s
    normal_speed: f32, // jogging speed, just using WASD
    sprint_speed: f32, // 2x the normal speed, hold shift to access

    is_dead: bool,
    current_health: f32,
    max_health: f32,

    current_mana: f32,
    max_mana: f32,

    basic_cost: f32,          // mana cost of basic attack
    kenetic_pulse_cost: f32,  // ... kenetic_pulse 
    lightning_cost: f32,

    basic_power: f32,           // how much damage each will do. 
    kenetic_pulse_power: f32,
    lightning_power: f32,

}



const PLAYER_JOG_SPEED:    f32 = 2.68;
const PLAYER_SPRINT_SPEED: f32 = 2.0 * PLAYER_JOG_SPEED;

impl PlayerCharacter {

    /// Update procedure obviously takes care of changing the internal state of a PlayerCharacter
    /// instance accoaring to both input from other variables ingame and input from the human
    /// player (keybaord input & mouse input)
    /// 
    /// Call this function every frame out whatever -you know.
    pub fn update(&mut self, delta_time: f32, environment: &mut Environment) {

        // update position in the world accoarding to movement input
        if is_key_down(KeyCode::D) {
            self.translate(Direction::Right, delta_time);
        }
        if is_key_down(KeyCode::A) {
            self.translate(Direction::Left, delta_time);
        }
        if is_key_down(KeyCode::W) {
            self.translate(Direction::Up, delta_time);
        }
        if is_key_down(KeyCode::S) {
            self.translate(Direction::Down, delta_time);
        }

        // Sprinting Contol.
        if is_key_pressed(KeyCode::LeftShift) {
            self.begin_sprint();
        }
        if is_key_released(KeyCode::LeftShift) {
            self.end_sprint();
        }

        // attacking
        if is_mouse_button_pressed(MouseButton::Left) {

            // vector containing mouse position on screen in pixels
            let (x, y) = mouse_position();

            // where the click was but now in the world coordinates
            let clicked_pos: Vec2 = convert_to_world_coords(Vec2{x,y}, &self, TILE_WIDTH);

            // where the click happened, but now relative to the player (world coords)
            let vec_from_player: Vec2 = clicked_pos - self.get_position();

            // divided by magnitude to get unit vector
            let d: Vec2 = vec_from_player / vec_from_player.x.hypot(vec_from_player.y);

            self.cast_spell(Spell::Basic, d, environment);
        }
    }

    /// Default Constructor | Get a fresh player character.
    pub fn new() -> PlayerCharacter {
        PlayerCharacter {
            position: Vec2 { x: 0.0, y: 0.0 },
            speed: PLAYER_JOG_SPEED,
            normal_speed: PLAYER_JOG_SPEED,
            sprint_speed: PLAYER_SPRINT_SPEED,

            is_dead: false,
            current_health: 100.0,
            max_health: 100.0,

            current_mana: 100.0,
            max_mana: 100.0,

            basic_cost: 0.0,
            kenetic_pulse_cost: 10.0,
            lightning_cost: 30.0,

            basic_power: 15.0,
            kenetic_pulse_power: 25.0,
            lightning_power: 100.0,

        }
    }
    // TODO: A fix is needed. See Trello. Remove this when resolved.
    /// Construct a Player Character from saved data
    pub fn from_saved() {}

    /// Use this procedure to move the PlayerCharacter around in the world accoarding to arrow key input.
    pub fn translate(&mut self, d: Direction, mut deltat: f32) {
        deltat /= 1000.0; // this is importatnt because `deltat` comes in in milliseconds
                          // we need it in seconds. 
        match d {
            Direction::Right => self.position.x += 1.0 * self.speed * deltat,
            Direction::Left  => self.position.x -= 1.0 * self.speed * deltat,
            Direction::Up    => self.position.y += 1.0 * self.speed * deltat,
            Direction::Down  => self.position.y -= 1.0 * self.speed * deltat,
        }
    }

    /// toggles sprinting of player on
    pub fn begin_sprint(&mut self) {
        self.speed = self.sprint_speed;
    }
    /// toggles sprinting of player off
    pub fn end_sprint(&mut self) {
        self.speed = self.normal_speed;
    }

    /// PlayerCharacter position getter
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /* --- === ======== === ### Health Functions ### === ======== === --- */

    /// Reduce the health of a player character. This will kill
    /// the player if it is too much damage.
    pub fn hurt(&mut self, amount: f32) {
        if self.current_health - amount > 0.0 {
            self.current_health = self.current_health - amount;
        } else {
            self.current_health = 0.0;
            self.is_dead = true;
        }
    }
    /// Increase a player character health by amount. Cannot overheal (exceed max hp value)
    pub fn heal(&mut self, amount: f32) {
        if self.current_health + amount < self.max_health {
            self.current_health = self.current_health + amount;
        } else {
            self.current_health = self.max_health;
        }
    }

    pub fn get_health(&self) -> f32 {
        self.current_health
    }

    /* --- === ======== === ### Mana Functions ### === ======== === --- */

    /// Safely reduces the mana of a player character by amount.
    pub fn drain(&mut self, amount: f32) {
        if self.current_mana - amount > 0.0 {
            self.current_mana = self.current_mana - amount;
        } else {
            self.current_mana = 0.0;
        }
    }
    /// Safely increases mana of a player character by amount.
    pub fn energize(&mut self, amount: f32) {
        if self.current_mana + amount < self.max_mana {
            self.max_mana = self.max_mana + amount;
        } else {
            self.current_mana = self.max_mana;
        }
    }

    /// Safely handles an attempt to cast a spell. Returns whether or
    /// not the player character has sufficient mana to cast the spell. 
    pub fn can_cast(& self, spell: &Spell) -> bool {
        match spell {
            Spell::Basic => {
                if self.current_mana >= self.basic_cost {
                    return true;
                } else {
                    return false;      // I don't have to use the getter here .get_mana_cost()
                }                      // because we already know which cost it is in each case.
            }                          // Also, that would be slower.
            Spell::KeneticPulse => {    
                if self.current_mana >= self.kenetic_pulse_cost {
                    return true;
                } else {
                    return false;
                }
            }
            Spell::Lightning => {
                if self.current_mana >= self.lightning_cost {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    /// Reutrns the mana cost of a spell for a player. 
    pub fn get_mana_cost(&self, spell: &Spell) -> f32 {
        match spell {
            Spell::Basic        => self.basic_cost,
            Spell::KeneticPulse => self.kenetic_pulse_cost,
            Spell::Lightning    => self.lightning_cost,
        }
    }

    /// Returns the power of a spell for a player
    pub fn get_spell_power(&self, spell: &Spell) -> f32 {
        match spell {
            Spell::Basic        => self.basic_power,
            Spell::KeneticPulse => self.kenetic_pulse_power,
            Spell::Lightning    => self.lightning_power,
        }
    }

    /// Call this when a spell is needed to be cast, like on a mouse event or key input.
    /// Let it know the spell you want to cast and the environment in which the projectile will be added to.
    pub fn cast_spell(&mut self, spell: Spell, direction: Vec2, environment:  &mut Environment) {

        if self.can_cast(&spell) {

            self.drain( self.get_mana_cost( &spell ) );

            environment.player_projectiles.push(PlayerProjectile::new(self, direction, spell)) 
        }
    }

    pub fn get_mana(&self) -> f32 {
        self.current_mana
    }


}
