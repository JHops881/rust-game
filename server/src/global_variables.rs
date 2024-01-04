use std::sync::Mutex;

use crate::environment_singleton::*;


use lazy_static::lazy_static;




/* GLOBAL MUTABLE STATE */

// `ENVIRONMENT_INSTANCE` depends on this bool to determine if the singleton has already
// been instantiated. 
pub static mut IS_ENVIRONNMENT_MADE: bool = false;

// Use lazy_static to create a global mutable singleton `ENVIRONMENT_INSTANCE`: `EnvironmentSingleton`
lazy_static! {
    pub static ref ENVIRONMENT_INSTANCE: Mutex<EnvironmentSingleton> =
        Mutex::new(EnvironmentSingleton::new());
}

