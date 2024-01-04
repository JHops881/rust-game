use global_variables::{MAIN_CHARACTER_INSTANCE, ZERO_FLOAT};
use graphics_entity::{GraphicsEntity, GameEntity};
use macroquad:: {miniquad::window::set_fullscreen,
    window:: {
        clear_background,
        next_frame
    }, 
    color::BLACK, math::Vec2
};


pub mod global_variables;
pub mod graphics_math;
pub mod graphics_procedure;
pub mod graphics_entity;
pub mod Graphics_Entity;
pub mod main_character_singleton;





/*

Okay, so lets think about this.. The server is going to send us a block of data that contains the
the locations and types of graphics entities that are relevant to us.

it is our job to the decide which ones to take, add them to a collection, (like a list)
draw them


The client will always be one game tick behind the server so that it can use the "next" game
tick to lineraly interpolate the clients visuals. 


*/


#[macroquad::main("BasicShapes")]

async fn main() {


    let mut connection_ipv4: String = String::new();
    let mut session_id:      String = String::new();

    set_fullscreen(true);

    //let main_player: GraphicsEntity = GraphicsEntity {world_pos: MAIN_CHARACTER_INSTANCE.get_position_vec2(), entity_type: GameEntity::PlayerCharacter};


    let ghoul: GraphicsEntity = GraphicsEntity {world_pos: Vec2 {x: ZERO_FLOAT, y: ZERO_FLOAT}, entity_type: GameEntity::Ghoul};

    let coin: GraphicsEntity = GraphicsEntity {world_pos: Vec2 {x: 20.0, y: 5.0}, entity_type: GameEntity::Coin};

    let lootbag: GraphicsEntity = GraphicsEntity {world_pos: Vec2 {x: -6.0, y: -9.6}, entity_type: GameEntity::LootBag};

    // While game is running...
    loop {

        // grab local input (macroquad will do this)
        // push local input to server

        // recieve server state

        
        

        /* DRAW */

        // Clear screen
        clear_background(BLACK);
        ghoul.draw();
        coin.draw();
        lootbag.draw();


        // Basic Play UI and Debug Information

        next_frame().await
    }
}

/// pocedure to send client information to server
fn send_client_info() {

}