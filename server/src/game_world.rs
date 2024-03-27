use crate::{
    enemy_character::EnemyCharacter, player_character::PlayerCharacter,
    player_projectile::PlayerProjectile,
};

/// An game world is just a data structure that stores our things in the world.
/// For something to "Exist" It needs to be stored in the game world instance. The
/// game world contains vectors of the types of things that exist. With a few
/// extra steps, this is what enables things in the world to interact with
/// eachother -because they exist in the same game world!
pub struct GameWorld {
    pub player_characters: Vec<PlayerCharacter>,
    pub player_projectiles: Vec<PlayerProjectile>,
    pub enemy_characters: Vec<EnemyCharacter>,
}

impl GameWorld {
    pub fn new() -> Self {
        return GameWorld {
            player_characters: Vec::new(),
            player_projectiles: Vec::new(),
            enemy_characters: Vec::new(),
        };
    }

    /// games primary update function that is called at the tickrate of the server. 
    /// encompasses the whole state of the game. 
    pub fn fixed_update(&mut self, delta_time: f32) {
        // update players
        for player_character in self.player_characters.iter_mut() {
            player_character.update(delta_time);
        }

        // update plyaer projectiles
        for proj in self.player_projectiles.iter_mut() {
            proj.update(delta_time);
        }
    }

    pub fn cull_expired_projectiles(&mut self) {
        self.player_projectiles.retain(|proj| !proj.is_expired());
    }
}
