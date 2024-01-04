////////////////////////////////// IMPORTS ////////////////////////////////////


use macroquad:: {
    math::Vec2,
    input:: {
        is_key_down,
        is_key_pressed,
        is_key_released,
        is_mouse_button_pressed,
        mouse_position
    },
    miniquad:: {
        KeyCode,
        MouseButton
    }
};

use crate:: {
    global_variables::{TILE_WIDTH, ENVIRONMENT_INSTANCE},
    player_projectile::PlayerProjectile,
};


//////////////////////////////////// CODE /////////////////////////////////////



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
    
    speed_stat:  f32, // jogging speed
    health_stat: f32, // max health
    mana_stat:   f32, // max mana
    power_stat:  f32, // attack power
    vitality_stat:  f32, // health regen in hp/s
    wisdom_stat:    f32, // mana regen   in mana/s
    dexterity_stat: f32, // attack speed  attacks/s
    defense_stat:   f32, // flat damage reduction

    current_speed:  f32, // in m/s
    current_health: f32,  
    current_mana:   f32,

    is_dead: bool,
    is_sprinting: bool,

    basic_cost: f32,          // mana cost of basic attack
    kenetic_pulse_cost: f32,  // ... kenetic_pulse 
    lightning_cost: f32,

    basic_power_multi: f32,          // a value that deterimes how much damage 
    kenetic_pulse_power_multi: f32,  // it will do. It's a multiplier.
    lightning_power_multi: f32,      // damage = power_stat * value

}



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

            self.cast_spell(Spell::Basic, d);
        }
    }

    /// Default Constructor | Get a fresh player character.
    pub fn new() -> PlayerCharacter {

        let initial_speed: f32 = 2.68;

        PlayerCharacter {
            position: Vec2{x:0.0, y:0.0},    
    
            speed_stat:  initial_speed,
            health_stat: 100.0,
            mana_stat:   100.0,
            power_stat:  10.0,
            vitality_stat:  1.0, 
            wisdom_stat:    2.5,
            dexterity_stat: 2.0, 
            defense_stat:   0.0, 
        
            current_speed:  initial_speed,
            current_health: 1.0,  
            current_mana:   1.0,
        
            is_dead: false,
            is_sprinting: false,
        
            basic_cost: 0.0,         
            kenetic_pulse_cost: 10.0, 
            lightning_cost: 30.0,
        
            basic_power_multi: 1.0,           
            kenetic_pulse_power_multi: 2.0,
            lightning_power_multi: 5.0,

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
            Direction::Right => self.position.x += 1.0 * self.current_speed * deltat,
            Direction::Left  => self.position.x -= 1.0 * self.current_speed * deltat,
            Direction::Up    => self.position.y += 1.0 * self.current_speed * deltat,
            Direction::Down  => self.position.y -= 1.0 * self.current_speed * deltat,
        }
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
            },
            false =>()
        }
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
        if self.current_health + amount < self.health_stat {
            self.current_health = self.current_health + amount;
        } else {
            self.current_health = self.health_stat;
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
        if self.current_mana + amount < self.mana_stat {
            self.mana_stat = self.mana_stat + amount;
        } else {
            self.current_mana = self.mana_stat;
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
            Spell::Basic        => self.basic_power_multi,
            Spell::KeneticPulse => self.kenetic_pulse_power_multi,
            Spell::Lightning    => self.lightning_power_multi,
        }
    }

    /// Call this when a spell is needed to be cast, like on a mouse event or key input.
    /// Let it know the spell you want to cast and the environment in which the projectile will be added to.
    pub fn cast_spell(&mut self, spell: Spell, direction: Vec2) {

        if self.can_cast(&spell) {

            self.drain( self.get_mana_cost( &spell ) );

            let result = ENVIRONMENT_INSTANCE.lock();
            match result {

                Ok(mut env_inst) => env_inst.player_projectiles.push(PlayerProjectile::new(self, direction, spell)),
                Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),

            }

             
        }
    }

    pub fn get_mana(&self) -> f32 {
        self.current_mana
    }


}
