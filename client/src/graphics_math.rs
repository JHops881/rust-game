use macroquad:: {
    math::Vec2,
    window:: {
        screen_width,
        screen_height
    }
};

use crate::global_variables::{TILE_WIDTH, MAIN_CHARACTER_INSTANCE};



/// Depends on Glabal Variables: `MAIN_CHARACTER_INSTANCE` and `TILE_WIDTH`
pub fn convert_to_world_coords(screen_coords: Vec2) -> Vec2 {

    let result = MAIN_CHARACTER_INSTANCE.lock();
    match result {
        Ok(mci) => Vec2 {

            x:   (screen_coords.x - screen_width() / 2.0)
               / TILE_WIDTH
               + mci.get_x(),
    
            y: - (screen_coords.y - screen_height() / 2.0)
               / TILE_WIDTH
               + mci.get_y(),
            },
        Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
    }

    

}


/// Depends on Glabal Variables: `MAIN_CHARACTER_INSTANCE` and `TILE_WIDTH`
pub fn convert_to_screen_coords(world_coords: Vec2) -> Vec2 {

    let result = MAIN_CHARACTER_INSTANCE.lock();
    match result {
        Ok(mci) => Vec2 {

        x:   (world_coords.x - mci.get_x())
           * TILE_WIDTH
           + screen_width() / 2.0,

        y:   (world_coords.y - mci.get_y())
           * -1.0
           * TILE_WIDTH
           + screen_height() / 2.0,
        },
        Err(poisoned) => panic!("Mutex is poisoned: {:?}", poisoned),
    }
}
