use macroquad::math::Vec2;

use crate::global_variables::{IS_MAIN_CHARACTER_MADE, ZERO_FLOAT};

/// A single struct that contains the data relevant to the playercharacter that the user of the client is playing as. 
pub struct MainCharacterSingleton {

    position: Vec2,    // euclidian coordinates in the game world
    
    speed_stat:  f32, // jogging speed
    health_stat: f32, // max health
    mana_stat:   f32, // max mana
    power_stat:  f32, // attack power
    vitality_stat:  f32, // health regen in hp/s
    wisdom_stat:    f32, // mana regen   in mana/s
    dexterity_stat: f32, // attack speed  attacks/s
    defense_stat:   f32, // flat damage reduction

    current_speed:  f32, // in m/s
    current_health: f32,  
    current_mana:   f32,

    is_dead: bool,
    is_sprinting: bool,

    basic_cost: f32,          // mana cost of basic attack
    kenetic_pulse_cost: f32,  // ... kenetic_pulse 
    lightning_cost: f32,

    basic_power_multi: f32,          // a value that deterimes how much damage 
    kenetic_pulse_power_multi: f32,  // it will do. It's a multiplier.
    lightning_power_multi: f32,      // damage = power_stat * value

}

impl MainCharacterSingleton {

    pub fn new() -> Self {

        unsafe {

            if IS_MAIN_CHARACTER_MADE == false {

                IS_MAIN_CHARACTER_MADE = true;

                MainCharacterSingleton {

                    position: Vec2 {x: ZERO_FLOAT, y: ZERO_FLOAT},    
                    
                    speed_stat:  ZERO_FLOAT, 
                    health_stat: ZERO_FLOAT,
                    mana_stat:   ZERO_FLOAT,
                    power_stat:  ZERO_FLOAT,
                    vitality_stat:  ZERO_FLOAT,
                    wisdom_stat:    ZERO_FLOAT,
                    dexterity_stat: ZERO_FLOAT,
                    defense_stat:   ZERO_FLOAT,
                
                    current_speed:  ZERO_FLOAT,
                    current_health: ZERO_FLOAT,  
                    current_mana:   ZERO_FLOAT,
                
                    is_dead: false,
                    is_sprinting: false,
                
                    basic_cost: ZERO_FLOAT,
                    kenetic_pulse_cost: ZERO_FLOAT,
                    lightning_cost: ZERO_FLOAT,
                
                    basic_power_multi: ZERO_FLOAT,
                    kenetic_pulse_power_multi: ZERO_FLOAT,
                    lightning_power_multi: ZERO_FLOAT,
                
                }
                

            } else {

                panic!("Attempted to create a second main character!")

            }
        }

    }



    pub fn get_x(&self) -> f32 {
        self.position.x
    }

    pub fn get_y(&self) -> f32 {
        self.position.y
    }

    pub fn get_position_vec2(&self) -> Vec2 {
        self.position
    }
}