
use crate::{enemy_character::*, global_variables::ENVIRONMENT_INSTANCE, player_character::PlayerCharacter};
use macroquad::math::Vec2;

/// The `GameEntityFactory` provides an interface for the developer to initialiaze game entities
/// into the game's environment. These entities are outlined below. The environment that they are
/// instantiated into is the `EnvironmentSingleton`. Thus the `GameEntityFacoty` is striclty
/// dependent on the global mutable `ENVIRONMENT_INSTANCE` instance of `EnvironmentSingleton`
/// (It is baked in as the receiver to store the entity objects).
/// 
/// The GameEntityFactory is intended to be used as the primary/sole interface to use when
/// creating a new instance of any game entity. It provides some of the highest level of 
/// abstraction possible and hides pretty much every detail. As long as you give the methods
/// the handful of critical arguments, it will take care of the rest and make sure it is
/// inserted into the environment and thus the game world. 
/// 
/// 
/// Game Entities:
/// 
///     Characters:
///         Enemy Characters:
///             1. Ghoul
///             2. Phantom
///             3. Drinker
///             4. Crawler
///         Player Characters:
///             5. Player Character
///         Shop Character:
///             6. NOT IMPLIMENTED
///     Projectiles:
///         Enemy Projectiles:
///             NOT IMPLIMENTED
///         Character Projectles:
///             7. Kenetic Pulse
///             8. Lightning
///             9. Basic
///   
pub struct GameEntityFactory {

}

impl GameEntityFactory {


    // Adds a ghouls to the game world at a position. 
    pub fn create_ghoul(position: Vec2) {

        let ghoul: EnemyCharacter = EnemyCharacter::new(EnemyType::Ghoul, position);

        let result = ENVIRONMENT_INSTANCE.lock();
        match result {

            Ok(mut env_inst) => env_inst.enemy_characters.push(ghoul),
            Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
        }
        
    }

    pub fn create_new_player_character() {
        
        let mut player = PlayerCharacter::new();

        let result = ENVIRONMENT_INSTANCE.lock();
        match result {

            Ok(mut env_inst) => env_inst.player_characters.push(player),
            Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
        }
        
    }

    pub fn create_kenetic_pulse() {

    }
}