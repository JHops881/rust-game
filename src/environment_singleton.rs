use crate:: {
    player_projectile::PlayerProjectile,
    player_character::PlayerCharacter,
    enemy_character::EnemyCharacter,
    global_variables::IS_ENVIRONNMENT_MADE
};





/// An Environment is just a data structure that stores our things in the world.
/// For something to "Exist" It needs to be stored in the Environment instance. The
/// Environment contains vectors of the types of things that exist. With a few
/// extra steps, this is what enables things in the world to interact with
/// eachother -because they exist in the same environment!
///
/// Pretty cool Right!? Be nice to you environment.
pub struct EnvironmentSingleton {
    // allies
    pub player_characters: Vec<PlayerCharacter>,
    pub player_projectiles: Vec<PlayerProjectile>,

    // enemies
    pub enemy_characters: Vec<EnemyCharacter>,
}

impl EnvironmentSingleton {

    /// Construct the environmnet. This enforces singleton rules.
    /// It depends on a global counter `ENVIRONMENT_COUNT`
    pub fn new() -> Self {

        unsafe {

            if IS_ENVIRONNMENT_MADE == false {

                IS_ENVIRONNMENT_MADE = true;

                return EnvironmentSingleton {
                    player_characters: Vec::new(),
                    player_projectiles: Vec::new(),
                    enemy_characters: Vec::new(),
                };

            } else {

                panic!("Attempted to create a second environment!")

            }
        }
    }
}

// TODO: MAKE
impl EnvironmentSingleton {
    pub fn cull_expired_projectiles(&mut self) {
        ()
    }
}