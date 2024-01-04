
////////////////////////////////// IMPORTS ////////////////////////////////////


use macroquad::math::Vec2;

use crate:: {
    player_character:: {
        Spell,
        PlayerCharacter
    },
    system_time
};


//////////////////////////////////// CODE /////////////////////////////////////




/// These things are the result of a player casting a spell. They fly through
/// the air and blow things up fr fr fr
pub struct PlayerProjectile {
    
    expiriation_time:   f64,     // Epoch time in ms when it will be dropped
    is_expired:         bool,    // all things come to an end. 

    position:  Vec2,    // Euclidian
    direction: Vec2,    // unit vector that points in the direction its heading
    speed:     f32,     // m/s
    
    spell_type: Spell,  // we can get the power from this

}

impl PlayerProjectile {
    /// Call this when a player is casting a spell. Let it know the direction it is to go, the type
    /// of spell projectile it is. Pass the `direction` as a unit vector.
    pub fn new(player: & PlayerCharacter, direction: Vec2, spell: Spell) -> PlayerProjectile {
        PlayerProjectile { 
            expiriation_time: system_time() + 3100.0, // 3.1 seconds after creation
            is_expired: false,

            position: player.get_position(), // starts on the casting PlayerCharacter
            direction: direction,
            speed: 10.0,

            spell_type: spell,
        }
    }

    /// Call this every fixed game update.
    pub fn update(&mut self, delta_time: f32) {
        if self.expiriation_time > system_time() {
            self.translate(delta_time)
        } else {
            // Ideally, we would drop the damn thing right here. But it is
            // a tough cookie. Can't drop by reference at all. We need to 
            // instead mark it for death and have someone or something else
            // drop it externally. 
            // our Angel of Death (i mean `Environment`` instance) will take
            // it from here. 
            self.is_expired = true; // ew, I hope it's not milk!
        }
    }

    /// Nothing fancy, just a way to move the projectile. 
    pub fn translate(&mut self, mut delta_time: f32) {
        delta_time /= 1000.0;

        self.position.x += self.direction.x * self.speed * delta_time;
        self.position.y += self.direction.y * self.speed * delta_time;

    }

    pub fn get_position(&self) -> Vec2 { // google wants to know your location. 
        self.position
    }

    pub fn is_expired(&self) -> bool { 
        self.is_expired
    }
}