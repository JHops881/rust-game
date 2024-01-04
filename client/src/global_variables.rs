use std::sync::Mutex;

use lazy_static::*;

use crate::main_character_singleton::MainCharacterSingleton;





/* GLOBAL CONSTATNS */
pub const TILE_WIDTH: f32 = 16.0; // pixels

const LOOPBACK_IPV4:    &str = "127.0.0.1";
const APPLICATION_PORT: &str = "42110";

pub const ZERO_FLOAT: f32 = 0.0;



/* GLOBAL MUTABLE STATE */

// `MAIN_CHARACTER_INSTANCE` depends on this bool to determine if the singleton has already
// been instantiated. 
pub static mut IS_MAIN_CHARACTER_MADE: bool = false;

// Use lazy_static to create a global mutable singleton `MAIN_CHARACTER_INSTANCE`: `EnvironmentSingleton`
lazy_static! {
    pub static ref MAIN_CHARACTER_INSTANCE: Mutex<MainCharacterSingleton> =
        Mutex::new(MainCharacterSingleton::new());
}