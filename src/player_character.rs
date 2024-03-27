use macroquad::math::Vec2;
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html
use macroquad::input::*;

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

// We will get to cast these later. Here are the mana costs used for implementation.
pub enum Spell {
    KeneticPulse,
    Lightning,
}
const KENETIC_PULSE_COST: f32 = 10.0;
const LIGHTNING_COST: f32 = 30.0;

// Star of the show!
pub struct PlayerCharacter {
    position: Vec2,    // euclidian coordinates in the game world
    speed: f32,        // current speed in m/s
    normal_speed: f32, // jogging speed, just using WASD
    sprint_speed: f32, // 2x the normal speed, hold shift to access

    is_dead: bool,
    current_health: f32,
    max_health: f32,

    is_oom: bool, // (Out Of Mana)
    current_mana: f32,
    max_mana: f32,
}



const PLAYER_JOG_SPEED:    f32 = 2.68;
const PLAYER_SPRINT_SPEED: f32 = 2.0 * PLAYER_JOG_SPEED;

impl PlayerCharacter {

    /// Update procedure obviously takes care of changing the internal state of a PlayerCharacter
    /// instance accoaring to both input from other variables ingame and input from the human
    /// player (keybaord input & mouse input)
    /// 
    /// Call this function every frame out whatever -you know.
    pub fn update(&mut self, delta_time: f32) {

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

            is_oom: false,
            current_mana: 100.0,
            max_mana: 100.0,
        }
    }
    // TODO: A fix is needed. See Trello. Remove this when resolved.
    /// Construct a Player Character from saved data
    pub fn from_saved() {}

    /// Use this procedure to move the PlayerCharacter around in the world accoarding to arrow key input.
    pub fn translate(&mut self, d: Direction, mut deltat: f32) {
        deltat /= 1000.0;
        match d {
            Direction::Right => self.position.x = self.position.x + 1.0 * self.speed * deltat,
            Direction::Left => self.position.x = self.position.x - 1.0 * self.speed * deltat,
            Direction::Up => self.position.y = self.position.y + 1.0 * self.speed * deltat,
            Direction::Down => self.position.y = self.position.y - 1.0 * self.speed * deltat,
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
            self.is_oom = true;
        }
    }
    /// Safely increases mana of a player character by amount.
    pub fn energize(&mut self, amount: f32) {
        if self.current_mana + amount < self.max_mana {
            self.max_mana = self.max_mana + amount;
        } else {
            self.current_mana = self.max_mana;
        }
        self.is_oom = false;
    }

    /// Safely handles an attempt to cast a spell. Returns whether or not is possible
    pub fn can_cast(&mut self, spell: Spell) -> bool {
        match spell {
            Spell::KeneticPulse => {
                if self.current_mana >= KENETIC_PULSE_COST {
                    return true;
                } else {
                    return false;
                }
            }
            Spell::Lightning => {
                if self.current_mana >= LIGHTNING_COST {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }

    pub fn get_mana(&self) -> f32 {
        self.current_mana
    }
}
