
use macroquad::math::Vec2; 
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html



/*

        TABLE OF CONTENTS
+------------------+-----------+
|     Section      | Order No. |
+------------------+-----------+

| PlayerCharacter  |     1     |
+------------------+-----------+
| EnemyCharacter   |     2     |
+------------------+-----------+



--- ==== ==== ## TODO LIST:## ==== ==== ---

[ ] PlayerCharacter::from_saved() 12/29/2023
        * Make this function.

[X] PlayerCharacter.translate() 12/29/2023
        * Needs Delta Time included in computations.  RESOLVED

[X] EnemyCharacter.translate() 12/29/2023
        * Needs Delta Time included in computations.  RESOLVED

[ ] EnemyCharacter.translate() 12/29/2023
        * Use an angle for the direction instead of an enum

    [ ] EnemyCharacter.move_towards_player()
            * Use the NEW .translate() method to build smarter/faster
              pathfinding with better math.

[ ] PlayerCharacter::new() 12/29/2023
       * There has to be a more elegant way that is less error prone to get
         that 2.68 speed in three times. 

[ ] PlayerCharacter.try_cast() 12/29/2023
       * Make it a pure function. As long as we know whether or not
         we can cast it, the .drain() call can be handled elsewhere.


*/











//////////////////////////////////////////////////////////////////////////////
//                                                                          //
//                         SECTION 1 | PlayerCharacter                      //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

// We will get to cast these later. Here are the mana costs used for implementation.
pub enum Spell {
    KeneticPulse, // 10 mana
    Lightning,    // 30 mana
}

// Star of the show!
pub struct PlayerCharacter {

    position:         Vec2,  // euclidian coordinates in the game world
    speed:            f32,   // current speed in m/s
    normal_speed:     f32,   // jogging speed, just using WASD
    sprint_speed:     f32,   // 2x the normal speed, hold shift to access

    is_dead:          bool,
    current_health:   f32,
    max_health:       f32,

    is_oom:           bool, // (Out Of Mana)
    current_mana:     f32,
    max_mana:         f32,
    

}

impl PlayerCharacter {

    // TODO: A fix is needed. See TODO LIST at top of document. Remove this when resolved. 
    /// Default Constructor | Get a fresh player character.
    pub fn new() -> PlayerCharacter {
        PlayerCharacter {
            position: Vec2 {
                x: 0.0,
                y: 0.0,
            },
            speed: 2.68,
            normal_speed: 2.68,         // <--- fix repeating number (2.68)
            sprint_speed: 2.0 * 2.68,

            is_dead: false,
            current_health: 100.0,
            max_health: 100.0,

            is_oom: false,
            current_mana: 100.0,
            max_mana: 100.0,
    
        }
    }
    // TODO: A fix is needed. See TODO LIST at top of document. Remove this when resolved. 
    /// Construct a Player Character from saved data
    pub fn from_saved() {
    }

    /// Use this procedure to move the PlayerCharacter around in the world accoarding to arrow key input. 
    pub fn translate(&mut self, d: Direction, deltat: f32) {
        match d {
            Direction::Right => self.position.x = self.position.x + 1.0 * self.speed * deltat,
            Direction::Left  => self.position.x = self.position.x - 1.0 * self.speed * deltat,
            Direction::Up    => self.position.y = self.position.y + 1.0 * self.speed * deltat,
            Direction::Down  => self.position.y = self.position.y - 1.0 * self.speed * deltat,
            
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

    // THIS LOGIC IS NONESENSE. FIX IMMEDIEATELY! -JOSEPH
    // TODO: A fix is needed. See TODO LIST at top of document. Remove this when resolved. 
    /// Safely handles an attempt to cast a spell. Returns whether or not is possible
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

    pub fn get_mana(&self) -> f32 {
        self.current_mana
    }
}

/* END SECTION. LEAVE 10 LINES! */








//////////////////////////////////////////////////////////////////////////////
//                                                                          //
//                         SECTION 2 | EnemyCharacter                       //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

pub enum EnemyType {
    Ghoul,     // Zombie Type, Very standard stats
    Phantom,   // Slow, Beefy, Higher Power
    Drinker,   // a little faster than average, heals on attacking player
    Crawler,   // Fast, High Damage, Low HP pool
}
// Boggies! Watch out!
pub struct EnemyCharacter { 

    enemy_type:       EnemyType,

    position:         Vec2,  // euclidian coordinates in the game world
    speed:            f32,   // m/s

    is_dead:          bool,
    current_health:   f32,
    max_health:       f32,

    power:            f32,   // a number that its attacks will scale off of

}

impl EnemyCharacter {
    
    /// This will Construct and return an EnemyCharacter with corresponding stats
    /// based on what type you make it. For now, this is also where those stats are
    /// decided, meaning balance changes to enemy types can be tweaked here. 
    pub fn new(t: EnemyType, p: Vec2) -> EnemyCharacter {
        match t {
            EnemyType::Ghoul => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 2.5,
                is_dead: false,
                current_health: 100.0,
                max_health: 100.0,
                power: 10.0,
            },
            EnemyType::Phantom => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 1.75,
                is_dead: false,
                current_health: 200.0,
                max_health: 200.0,
                power: 20.0,
            },
            EnemyType::Drinker => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 3.5,
                is_dead: false,
                current_health: 75.0,
                max_health: 150.0,
                power: 7.5,
            },
            EnemyType::Crawler => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 8.0,
                is_dead: false,
                current_health: 50.0,
                max_health: 50.0,
                power: 33.5,
            },
        }

    }

    // TODO: A fix is needed. See TODO LIST at top of document. Remove this when resolved. 
    /// Use this procedure to move the EnemyCharacter around in the world. 
    pub fn translate(&mut self, d: Direction, deltat: f32) {
        match d {
            Direction::Right => self.position.x = self.position.x + 1.0 * self.speed * deltat,
            Direction::Left  => self.position.x = self.position.x - 1.0 * self.speed * deltat,
            Direction::Up    => self.position.y = self.position.y + 1.0 * self.speed * deltat,
            Direction::Down  => self.position.y = self.position.y - 1.0 * self.speed * deltat,
            
        }
    }

    // TODO: A fix is needed. See TODO LIST at top of document. Remove this when resolved. 
    /// Extremly Crude pathfinding for enemies. Will be reworked later.
    pub fn move_towards_player(&mut self, player_character: & PlayerCharacter, deltat: f32) {

        // the signed value of the distance between player and enemy on both axis?
        let x_difference: f32 = self.get_position().x - player_character.get_position().x;
        let y_difference: f32 = self.get_position().y - player_character.get_position().y;
        
        // basically, travel along the axis with the larger distance, also go in correct
        // direction (toward the player)
        match x_difference.abs().partial_cmp(&y_difference.abs()) {
            Some(std::cmp::Ordering::Less) => {
                if y_difference > 0.0 {
                    self.translate(Direction::Down, deltat)
                } else {
                    self.translate(Direction::Up, deltat)
                }
            }
            Some(std::cmp::Ordering::Equal) => {
                // this should like never happen, but if it does...
                self.translate(Direction::Up, deltat)
                // not a huge deal honeslty. We'll fix it later anyway lol
            }
            Some(std::cmp::Ordering::Greater) => {
                if x_difference > 0.0 {
                    self.translate(Direction::Left, deltat)
                } else {
                    self.translate(Direction::Right, deltat)
                }
            }
            None => {
                println!("Comparison failed; at least one value is NaN");
            }
            
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /* --- === ======== === ### Health Functions ### === ======== === --- */

    /// Reduce the health of an enemy character. This will kill
    /// the enemy if it is too much damage.
    pub fn hurt(&mut self, amount: f32) {
        if self.current_health - amount > 0.0 {
            self.current_health = self.current_health - amount;
        } else {
            self.current_health = 0.0;
            self.is_dead = true;
        }
    }
    /// Increase a enemy character health by amount. Cannot overheal (exceed max hp value)
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

    // Really is all that it sounds like. Pick a character to attack. Some sort of collision
    // is intended to precede this call. 
    pub fn basic_attack(&mut self, player_character: &mut PlayerCharacter) {

        let basic_attack_scale_factor: f32 = 0.75;

        let attack_amount: f32 = self.power * basic_attack_scale_factor;

        match self.enemy_type {
            EnemyType::Ghoul   => player_character.hurt(attack_amount),
            EnemyType::Phantom => player_character.hurt(attack_amount),
            EnemyType::Drinker => { 
                player_character.hurt(attack_amount);
                self.heal(attack_amount);
            },
            EnemyType::Crawler => player_character.hurt(attack_amount),
        }
    }
}