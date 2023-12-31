use crate::player_character::*;
use crate::player_projectile::*;

use crate::enemy_character::*;


/// An Environment is just a data structure that stores our things in the world.
/// For something to "Exist" It needs to be stored in the Environment instance. The 
/// Environment contains vectors of the types of things that exist. With a few 
/// extra steps, this is what enables things in the world to interact with 
/// eachother -because they exist in the same environment!
/// 
/// Pretty cool Right!? Be nice to you environment. 
pub struct Environment {

    // allies
    pub player_characters: Vec<PlayerCharacter>,
    pub player_projectiles: Vec<PlayerProjectile>,

    // enemies
    pub enemy_characters: Vec<EnemyCharacter>,

}

impl Environment {
    /// default constructor
    pub fn new() -> Environment {
        Environment {
            player_characters: Vec::new(),
            player_projectiles: Vec::new(),

            enemy_characters: Vec::new(),
        }

    }
}

// TODO: MAKE
impl Environment {
    pub fn cull_expired_projectiles(&mut self) {
        ()
    }
}

