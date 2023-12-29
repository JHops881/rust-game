
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
    pub fn translate(pc: &mut PlayerCharacter, d: Direction) {
        match d {
            Direction::Right => pc.position.x = pc.position.x + 1.0 * pc.velocity,
            Direction::Left  => pc.position.x = pc.position.x - 1.0 * pc.velocity,
            Direction::Up    => pc.position.y = pc.position.y + 1.0 * pc.velocity,
            Direction::Down  => pc.position.y = pc.position.y - 1.0 * pc.velocity,
            
        }
    }
    
    pub fn get_position(pc: &PlayerCharacter) -> Vec2 {
        pc.position
    }

    /* --- === ======== === ### Health Functions ### === ======== === --- */

    /// Reduce the health of a player character. This will kill
    /// the player if it is too much damage.
    pub fn damage(pc: &mut PlayerCharacter, amount: f32) {
        if pc.current_health - amount > 0.0 {
            pc.current_health = pc.current_health - amount;
        } else {
            pc.current_health = 0.0;
            pc.is_dead = true;
        }
    }
    /// Increase a player character health by amount. Cannot overheal (exceed max hp value)
    pub fn heal(pc: &mut PlayerCharacter, amount: f32) {
        if pc.current_health + amount < pc.max_health {
            pc.current_health = pc.current_health + amount;
        } else {
            pc.current_health = pc.max_health;
        }
    }

    pub fn get_health(pc: &PlayerCharacter) -> f32 {
        pc.current_health
    }

    /* --- === ======== === ### Mana Functions ### === ======== === --- */

    /// Safely reduces the mana of a player character by amount.
    pub fn drain(pc: &mut PlayerCharacter, amount: f32) {
        if pc.current_mana - amount > 0.0 {
            pc.current_mana = pc.current_mana - amount;
        } else { 
            pc.current_mana = 0.0;
            pc.is_oom = true;
        }
    } 
    /// Safely increases mana of a player character by amount.
    pub fn energize(pc: &mut PlayerCharacter, amount: f32) {
        if pc.current_mana + amount < pc.max_mana {
            pc.max_mana = pc.max_mana + amount;
        } else {
            pc.current_mana = pc. max_mana;
        }
    }
    // TODO: FIX
    /// Safely handles an attempt to cast a spell.
    pub fn cast(pc: &mut PlayerCharacter, spell: Spell) {
        if pc.is_oom {
            () // do nothing, out of mana / cant cast
        } else {
            match spell {
                Spell::KeneticPulse => if pc.current_mana >= 10.0 {
                    PlayerCharacter::drain(pc, 10.0);
                    () // TODO: Actually be able to cast a spell meaningfully
                } else {
                    () // do nothing, insufficient mana
                }
                Spell::Lightning    => if pc.current_mana >= 30.0 {
                    PlayerCharacter::drain(pc, 30.0);
                    () // TODO: Actually be able to cast a spell meaningfully
                } else {
                    () // do nothing, insufficient mana
                }
            }
        }
    }

    pub fn get_mana(pc: &PlayerCharacter) -> f32 {
        pc.current_mana
    }
}



