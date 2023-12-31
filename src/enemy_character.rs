
use crate::player_character::*;

use macroquad::math::Vec2;
// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html

pub enum EnemyType {
    Ghoul,   // Zombie Type, Very standard stats
    Phantom, // Slow, Beefy, Higher Power
    Drinker, // a little faster than average, heals on attacking player
    Crawler, // Fast, High Damage, Low HP pool
}
// Boggies! Watch out!
pub struct EnemyCharacter {
    enemy_type: EnemyType,

    position: Vec2, // euclidian coordinates in the game world
    speed: f32,     // m/s

    is_dead: bool,
    current_health: f32,
    max_health: f32,

    power: f32, // a number that its attacks will scale off of
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

    /// Use this procedure to move the EnemyCharacter around in the world.
    /// needs a unit vector as a direction to move in.
    pub fn translate(&mut self, unit_vector: Vec2, mut delta_time: f32) {
        
        delta_time /= 1000.0; // this is importatnt because `delta_time` comes in in milliseconds

        self.position.x = self.position.x + unit_vector.x * self.speed * delta_time;
        self.position.y = self.position.y + unit_vector.y * self.speed * delta_time;
    
    }

    /// efficiently makes an enemy follow a player.
    pub fn move_towards_player(&mut self, player_character: &PlayerCharacter, delta_time: f32) {

        // Just a vector that is that is the distance and direction of the player
        // from the enemy. 
        let vector_to_player: Vec2  = Vec2 {
            x: player_character.get_position().x - self.get_position().x,
            y: player_character.get_position().y - self.get_position().y,
        };

        // turning the vector into just a direction and got rid of the magnitude. 
        // It was unneeded for our implementation of the .translate() procedure. 
        let unit_vec = vector_to_player / vector_to_player.x.hypot(vector_to_player.y);

        self.translate(unit_vec, delta_time);

    }

    /// EnemyCharacter Position Value Getter -> Vec2<f32>
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

    /// EnemyCharacter Health Stat Getter. 
    pub fn get_health(&self) -> f32 {
        self.current_health
    }

    /// Really is all that it sounds like. Pick a character to attack. Some sort of collision
    /// is intended to precede this call.
    pub fn basic_attack(&mut self, player_character: &mut PlayerCharacter) {

        // Might as well change this to 1.0 at some point. For now the basic attacks will
        // onlly for 75% of the power that an EnemyCharacter Instance possesses. 
        let basic_attack_scale_factor: f32 = 0.75;

        let attack_amount: f32 = self.power * basic_attack_scale_factor;

        // The enemy type matters here. Normally, it would just be a basic attack that scales
        // off of the power of the EnemyCharacter instance that is attacking... But we are 
        // going to do it a bit different! This way, if a special interaction is needed when 
        // The enemy attacks, it can be added here. Look! The `Drinker` Type already has one:
        // It heals for the amount that it attacks for. 
        match self.enemy_type {
            EnemyType::Ghoul => player_character.hurt(attack_amount),
            EnemyType::Phantom => player_character.hurt(attack_amount),
            EnemyType::Drinker => {
                player_character.hurt(attack_amount);
                self.heal(attack_amount);
            }
            EnemyType::Crawler => player_character.hurt(attack_amount),
        }
    }
}
