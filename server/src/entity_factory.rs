use crate::{
    enemy_character::*, game_world::GameWorld, player_character::PlayerCharacter,
    player_projectile::PlayerProjectile, spell::Spell,
};
use macroquad::math::Vec2;
use uuid::Uuid;

/// The `EntityFactory` provides an interface for the developer to initialiaze game entities
/// into the game's environment. These entities are outlined below. The environment that they are
/// instantiated into is the `EnvironmentSingleton`. Thus the `EntityFacoty` is striclty
/// dependent on the global mutable `ENVIRONMENT_INSTANCE` instance of `EnvironmentSingleton`
/// (It is baked in as the receiver to store the entity objects).
///
/// The EntityFactory is intended to be used as the primary/sole interface to use when
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
pub struct EntityFactory {}

impl EntityFactory {
    // Adds a ghouls to the game world at a position.
    pub fn create_ghoul(position: Vec2, game_world: &mut GameWorld) {
        let ghoul = EnemyCharacter::new(EnemyType::Ghoul, position);
        game_world.enemy_characters.push(ghoul);
    }

    pub fn create_new_player_character(name: String, id: Uuid, game_world: &mut GameWorld) {
        let player = PlayerCharacter::new(name, id);
        game_world.player_characters.push(player);
    }

    pub fn create_basic(postion: Vec2, direction: Vec2, game_world: &mut GameWorld) {
        game_world
            .player_projectiles
            .push(PlayerProjectile::new(postion, direction, Spell::Basic))
    }

    pub fn create_kenetic_pulse(postion: Vec2, direction: Vec2, game_world: &mut GameWorld) {
        game_world
            .player_projectiles
            .push(PlayerProjectile::new(postion, direction, Spell::KeneticPulse))
    }

    pub fn create_lightning(postion: Vec2, direction: Vec2, game_world: &mut GameWorld) {
        game_world
            .player_projectiles
            .push(PlayerProjectile::new(postion, direction, Spell::Lightning))
    }
}
