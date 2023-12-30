
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



TODO LIST:
[ ] PlayerCharacter::from_saved() - Make this function.
[ ] PlayerCharacter.translate()   - Needs Delta Time included in computations.
[ ] EnemyCharacter.translate()    - Needs Delta Time included in computations. 


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

pub enum Spell {
    KeneticPulse, // 10 mana
    Lightning,    // 30 mana
}


pub struct PlayerCharacter {

    position:         Vec2,  // euclidian coordinates in the game world
    speed:            f32,   // m/s
    normal_speed:     f32,
    sprint_speed:     f32, 

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
            speed: 2.68,
            normal_speed: 2.68,         // fix repeating number
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

    // TODO: A fix is needed. See TODO LIST at top of document. Remove this when resolved. 
    /// Use this procedure to move the PlayerCharacter around in the world accoarding to arrow key input. 
    pub fn translate(&mut self, d: Direction, deltat: f32) {
        match d {
            Direction::Right => self.position.x = self.position.x + 1.0 * self.speed * deltat,
            Direction::Left  => self.position.x = self.position.x - 1.0 * self.speed * deltat,
            Direction::Up    => self.position.y = self.position.y + 1.0 * self.speed * deltat,
            Direction::Down  => self.position.y = self.position.y - 1.0 * self.speed * deltat,
            
        }
    }

    pub fn begin_sprint(&mut self) {
        self.speed = self.sprint_speed;
    }

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
pub struct EnemyCharacter { 

    enemy_type:       EnemyType,

    position:         Vec2,  // euclidian coordinates in the game world
    speed:         f32,   // m/s

    is_dead:          bool,
    current_health:   f32,
    max_health:       f32,

    power:            f32,   // a number that its attacks will scale off of

}

impl EnemyCharacter {

    pub fn new(t: EnemyType, p: Vec2) -> EnemyCharacter {
        match t {
            EnemyType::Ghoul => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 1.0,
                is_dead: false,
                current_health: 100.0,
                max_health: 100.0,
                power: 10.0,
            },
            EnemyType::Phantom => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 0.5,
                is_dead: false,
                current_health: 200.0,
                max_health: 200.0,
                power: 20.0,
            },
            EnemyType::Drinker => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 1.5,
                is_dead: false,
                current_health: 75.0,
                max_health: 150.0,
                power: 7.5,
            },
            EnemyType::Crawler => EnemyCharacter {
                enemy_type: t,
                position: p,
                speed: 3.0,
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
            Direction::Right => self.position.x = (self.position.x + 1.0 * self.speed) * deltat,
            Direction::Left  => self.position.x = (self.position.x - 1.0 * self.speed) * deltat,
            Direction::Up    => self.position.y = (self.position.y + 1.0 * self.speed) * deltat,
            Direction::Down  => self.position.y = (self.position.y - 1.0 * self.speed) * deltat,
            
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