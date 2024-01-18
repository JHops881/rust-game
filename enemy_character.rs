use crate::player_character::PlayerCharacter;

use macroquad::math::Vec2;
use netlib::EntityType;
use uuid::Uuid;

// Vec2 Docs: https://docs.rs/macroquad/latest/macroquad/math/struct.Vec2.html

pub enum EnemyType {
    Ghoul,   // Zombie Type, Very standard stats
    Phantom, // Slow, Beefy, Higher Power
    Drinker, // a little faster than average, heals on attacking player
    Crawler, // Fast, High Damage, Low HP pool
}
// Boggies! Watch out!
pub struct EnemyCharacter {
    id: Uuid,
    enemy_type: EnemyType,

    position: Vec2, // euclidian coordinates in the game world
    speed: f32,     // m/s

    is_dead: bool,
    current_health: f32,
    max_health: f32,

    power: f32, // a number that its attacks will scale off of
}

// Constant values of monster initial stats
// we should move these to the constants file. Maybe?
const NO: bool = false;

const FULL_PF32: f32 = 1.0;
const HALF_PF32: f32 = 0.5;

const GHOUL_MAX_HEALTH: f32 = 100.0;
const GHOUL_INIT_HEALTH: f32 = FULL_PF32 * GHOUL_MAX_HEALTH;
const GHOUL_SPEED: f32 = 2.5;
const GHOUL_POWER: f32 = 10.0;

const PHANTOM_MAX_HEALTH: f32 = 200.0;
const PHANTOM_INIT_HEALTH: f32 = FULL_PF32 * PHANTOM_MAX_HEALTH;
const PHANTOM_POWER: f32 = 20.0;
const PHANTOM_SPEED: f32 = 1.75;

const DRINKER_MAX_HEALTH: f32 = 150.0;
const DRINKER_INIT_HEALTH: f32 = HALF_PF32 * DRINKER_MAX_HEALTH;
const DRINKER_POWER: f32 = 7.5;
const DRINKER_SPEED: f32 = 3.5;

const CRAWLER_MAX_HEALTH: f32 = 50.0;
const CRAWLER_INIT_HEALTH: f32 = FULL_PF32 * CRAWLER_MAX_HEALTH;
const CRAWLER_POWER: f32 = 33.5;
const CRAWLER_SPEED: f32 = 8.0;

impl EnemyCharacter {
    /// This will Construct and return an EnemyCharacter with corresponding stats
    /// based on what type you make it.
    pub fn new(enemy_type: EnemyType, position: Vec2) -> EnemyCharacter {
        match enemy_type {
            EnemyType::Ghoul => EnemyCharacter {
                id: Uuid::new_v4(),
                enemy_type,
                position,
                speed: GHOUL_SPEED,
                is_dead: NO,
                current_health: GHOUL_INIT_HEALTH,
                max_health: GHOUL_MAX_HEALTH,
                power: GHOUL_POWER,
            },
            EnemyType::Phantom => EnemyCharacter {
                id: Uuid::new_v4(),
                enemy_type,
                position,
                speed: PHANTOM_SPEED,
                is_dead: NO,
                current_health: PHANTOM_INIT_HEALTH,
                max_health: PHANTOM_MAX_HEALTH,
                power: PHANTOM_POWER,
            },
            EnemyType::Drinker => EnemyCharacter {
                id: Uuid::new_v4(),
                enemy_type,
                position,
                speed: DRINKER_SPEED,
                is_dead: NO,
                current_health: DRINKER_INIT_HEALTH,
                max_health: DRINKER_MAX_HEALTH,
                power: DRINKER_POWER,
            },
            EnemyType::Crawler => EnemyCharacter {
                id: Uuid::new_v4(),
                enemy_type,
                position,
                speed: CRAWLER_SPEED,
                is_dead: NO,
                current_health: CRAWLER_INIT_HEALTH,
                max_health: CRAWLER_MAX_HEALTH,
                power: CRAWLER_POWER,
            },
        }
    }

    pub fn update() {}

    /// EnemyCharacter Position Value Getter -> Vec2<f32>
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// EnemyCharacter Health Stat Getter.
    pub fn get_health(&self) -> f32 {
        self.current_health
    }

    pub fn get_uuid(&self) -> Uuid {
        self.id
    }

    pub fn get_entity_type(&self) -> EntityType {
        match self.enemy_type {
            EnemyType::Crawler => EntityType::Crawler,
            EnemyType::Drinker => EntityType::Drinker,
            EnemyType::Phantom => EntityType::Phantom,
            EnemyType::Ghoul => EntityType::Ghoul,
        }
    }

    /// Use this procedure to move the EnemyCharacter around in the world.
    /// needs a unit vector as a direction to move in.
    pub fn translate(&mut self, unit_vector: Vec2, mut delta_time: f32) {
        delta_time /= 1000.0; // this is important because `delta_time` comes in in milliseconds

        self.position.x = self.position.x + unit_vector.x * self.speed * delta_time;
        self.position.y = self.position.y + unit_vector.y * self.speed * delta_time;
    }

    /// efficiently makes an enemy follow a player.
    pub fn move_towards_player(&mut self, player_character: &PlayerCharacter, delta_time: f32) {
        // Just a vector that is that is the distance and direction of the player
        // from the enemy.
        let vector_to_player: Vec2 = Vec2 {
            x: player_character.get_position().x - self.get_position().x,
            y: player_character.get_position().y - self.get_position().y,
        };

        // turning the vector into just a direction and got rid of the magnitude.
        // It was unneeded for our implementation of the .translate() procedure.
        let unit_vec = vector_to_player / vector_to_player.x.hypot(vector_to_player.y);

        self.translate(unit_vec, delta_time);
    }

    /// Reduce the health of an enemy character. This will kill
    /// the enemy if it is too much damage.
    pub fn decrease_health(&mut self, amount: f32) {
        if self.current_health - amount > 0.0 {
            self.current_health = self.current_health - amount;
        } else {
            self.current_health = 0.0;
            self.is_dead = true;
        }
    }
    /// Increase a enemy character health by amount. Cannot overheal (exceed max hp value)
    pub fn increase_health(&mut self, amount: f32) {
        if self.current_health + amount < self.max_health {
            self.current_health = self.current_health + amount;
        } else {
            self.current_health = self.max_health;
        }
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
            EnemyType::Ghoul => player_character.decrease_health(attack_amount),
            EnemyType::Phantom => player_character.decrease_health(attack_amount),
            EnemyType::Drinker => {
                player_character.decrease_health(attack_amount);
                self.increase_health(attack_amount);
            }
            EnemyType::Crawler => player_character.decrease_health(attack_amount),
        }
    }
}
