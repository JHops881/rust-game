pub mod tiles;

use ::rand::thread_rng;
use ::rand::RngCore;
use macroquad::prelude::*;
use tiles::TileSet;
use tiles::get_tile_set;

#[macroquad::main("BasicShapes")]
async fn main() {

    let _tiles = get_tile_set();
    let map = create_example_map();

    set_fullscreen(true);
    loop {
        // Clear screen
        clear_background(BLACK);
        
        // sample drawing of tiles
        draw_map(&map, &_tiles);

        // Allow exiting
        if is_key_down(KeyCode::Escape) {
            std::process::exit(0);
        }

        next_frame().await
    }
}


fn create_example_map() -> [bool; 100] {
    let mut retval = [false; 100];
    let mut _rng = thread_rng();
    for i in 0..100 {
        retval[i] = (_rng.next_u32() & 0x11) > 0;
    }
    retval
}

fn draw_map(map: &[bool; 100], _tiles: &TileSet) {
    let params = DrawTextureParams {
        dest_size: Some(vec2(100., 100.)),
        ..Default::default()
    };

    for i in 0..100 {

        if !map[i] {
            continue;
        }
        let x: f32 = (i % 10) as f32 * 100. + 100.;
        let y: f32 = (i / 10) as f32 * 100. + 100.;

        let on_left_edge = i % 10 == 0;
        let on_right_edge = i % 10 == 9;
        let on_top_edge = i < 10;
        let on_bot_edge = 89 < i;

        let left_filled = on_left_edge || map[i-1];
        let right_filled = on_right_edge || map[i+1];
        let top_filled = on_top_edge || map[i-10];
        let bot_filled = on_bot_edge || map[i+10];

        match (left_filled, right_filled, top_filled, bot_filled) {
            (true, true, true, true) => draw_texture_ex(&_tiles.center, x, y, WHITE, params.clone()),
            (true, true, true, false) => draw_texture_ex(&_tiles.side_bottom, x, y, WHITE, params.clone()),
            (true, true, false, true) => draw_texture_ex(&_tiles.side_top, x, y, WHITE, params.clone()),
            (true, true, false, false) => draw_texture_ex(&_tiles.straight_left_right, x, y, WHITE, params.clone()),
            (true, false, true, true) => draw_texture_ex(&_tiles.side_right , x, y, WHITE, params.clone()),
            (true, false, true, false) => draw_texture_ex(&_tiles.corner_bottom_right, x, y, WHITE, params.clone()),
            (true, false, false, true) => draw_texture_ex(&_tiles.corner_top_right, x, y, WHITE, params.clone()),
            (true, false, false, false) => draw_texture_ex(&_tiles.peninsula_left, x, y, WHITE, params.clone()),
            (false, true, true, true) => draw_texture_ex(&_tiles.side_left, x, y, WHITE, params.clone()),
            (false, true, true, false) => draw_texture_ex(&_tiles.corner_bottom_left, x, y, WHITE, params.clone()),
            (false, true, false, true) => draw_texture_ex(&_tiles.corner_top_left, x, y, WHITE, params.clone()),
            (false, true, false, false) => draw_texture_ex(&_tiles.peninsula_right, x, y, WHITE, params.clone()),
            (false, false, true, true) => draw_texture_ex(&_tiles.straight_top_bottom, x, y, WHITE, params.clone()),
            (false, false, true, false) => draw_texture_ex(&_tiles.peninsula_top, x, y, WHITE, params.clone()),
            (false, false, false, true) => draw_texture_ex(&_tiles.peninsula_bottom, x, y, WHITE, params.clone()),
            (false, false, false, false) => (),
        }
    }
}

