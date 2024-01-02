use std::sync::Mutex;

use crate::environment_singleton::*;


use lazy_static::lazy_static;




/* GLOBAL MUTABLE STATE */
pub static mut IS_ENVIRONNMENT_MADE: bool = false;

// Use lazy_static to create a global mutable singleton
lazy_static! {
    pub static ref ENVIRONMENT_INSTANCE: Mutex<EnvironmentSingleton> =
        Mutex::new(EnvironmentSingleton::new());
}


/* GLOBAL CONSTATNS */
pub const TILE_WIDTH: f32 = 16.0; // pixels