
use macroquad::math::Vec2; 
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub enum Spell {
    KeneticPulse, // 10 mana
    Lightning,    // 30 mana
}


pub struct PlayerCharacter {

    position:         Vec2,  // euclidian coordinates in the game world
    velocity:         f32,   // m/s

    is_dead:          bool,
    current_health:   f32,
    max_health:       f32,

    is_oom:           bool, // (Out Of Mana)
    current_mana:     f32,
    max_mana:         f32,
    

}

impl PlayerCharacter {

    /// Default Constructor | Get a fresh player character.
    pub fn new() -> PlayerCharacter {
        PlayerCharacter {
            position: Vec2 {
                x: 0.0,
                y: 0.0,
            },
            velocity: 1.0,

            is_dead: false,
            current_health: 100.0,
            max_health: 100.0,

            is_oom: false,
            current_mana: 100.0,
            max_mana: 100.0,
    
        }
    }
    // TODO: MAKE
    /// Construct a Player Character from saved data
    pub fn from_saved() {
    }

    // TODO: NEEDS DELTATIME IN COMPUTATIONS
    /// Use this procedure to move the PlayerCharacter around in the world accoarding to arrow key input. 
    pub fn translate(&mut self, d: Direction) {
        match d {
            Direction::Right => self.position.x = self.position.x + 1.0 * self.velocity,
            Direction::Left  => self.position.x = self.position.x - 1.0 * self.velocity,
            Direction::Up    => self.position.y = self.position.y + 1.0 * self.velocity,
            Direction::Down  => self.position.y = self.position.y - 1.0 * self.velocity,
            
        }
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
    // TODO: FIX
    /// Safely handles an attempt to cast a spell.
    pub fn try_cast(&mut self, spell: Spell) -> bool {
        if self.is_oom {
            false
        } else {
            match spell {
                Spell::KeneticPulse => if self.current_mana >= 10.0 {
                    self.drain(10.0);
                    true 
                } else {
                    false
                }
                Spell::Lightning    => if self.current_mana >= 30.0 {
                    self.drain(30.0);
                    true 
                } else {
                    false
                }
            }
        }
    }

    pub fn get_mana(pc: &PlayerCharacter) -> f32 {
        pc.current_mana
    }
}



