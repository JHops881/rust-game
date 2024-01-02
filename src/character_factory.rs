
use crate::{enemy_character::*, global_variables::ENVIRONMENT_INSTANCE};
use macroquad::math::Vec2;

pub struct CharacterFactory {

}

impl CharacterFactory {


    // Adds a ghould to the game world at a position. 
    pub fn create_ghoul(position: Vec2) {

        let ghoul: EnemyCharacter = EnemyCharacter::new(EnemyType::Ghoul, position);

        let result = ENVIRONMENT_INSTANCE.lock();

        match result {

            Ok(mut env_inst) => env_inst.enemy_characters.push(ghoul),

            Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
        }
        
    }
}