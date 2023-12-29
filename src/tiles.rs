use ::rand::rngs::ThreadRng;
use ::rand::RngCore;
use macroquad::prelude::*;

/// The native width and height of a tile
pub const TILE_DIM: u32 = 16;

const CENTER_TILE_BYTES: &'static [u8; 155] = include_bytes!("../assets/tiles/center.png");
const CORNER_BOTTOM_LEFT: &'static [u8; 196] =
    include_bytes!("../assets/tiles/corner-bottom-left.png");
const CORNER_BOTTOM_RIGHT: &'static [u8; 184] =
    include_bytes!("../assets/tiles/corner-bottom-right.png");
const CORNER_TOP_LEFT: &'static [u8; 186] = include_bytes!("../assets/tiles/corner-top-left.png");
const CORNER_TOP_RIGHT: &'static [u8; 207] = include_bytes!("../assets/tiles/corner-top-right.png");
const PENINSULA_BOTTOM: &'static [u8; 235] = include_bytes!("../assets/tiles/peninsula-bottom.png");
const PENINSULA_LEFT: &'static [u8; 231] = include_bytes!("../assets/tiles/peninsula-left.png");
const PENINSULA_RIGHT: &'static [u8; 220] = include_bytes!("../assets/tiles/peninsula-right.png");
const PENINSULA_TOP: &'static [u8; 223] = include_bytes!("../assets/tiles/peninsula-top.png");
const SIDE_BOTTOM: &'static [u8; 171] = include_bytes!("../assets/tiles/side-bottom.png");
const SIDE_LEFT: &'static [u8; 181] = include_bytes!("../assets/tiles/side-left.png");
const SIDE_RIGHT: &'static [u8; 129] = include_bytes!("../assets/tiles/side-right.png");
const SIDE_TOP: &'static [u8; 186] = include_bytes!("../assets/tiles/side-top.png");
const STRAIGHT_LEFT_RIGHT: &'static [u8; 200] =
    include_bytes!("../assets/tiles/straight-left-right.png");
const STRAIGHT_TOP_BOTTOM: &'static [u8; 223] =
    include_bytes!("../assets/tiles/straight-top-bottom.png");

pub struct TileSet {
    pub center: Texture2D,
    pub corner_bottom_left: Texture2D,
    pub corner_bottom_right: Texture2D,
    pub corner_top_left: Texture2D,
    pub corner_top_right: Texture2D,
    pub peninsula_bottom: Texture2D,
    pub peninsula_left: Texture2D,
    pub peninsula_right: Texture2D,
    pub peninsula_top: Texture2D,
    pub side_bottom: Texture2D,
    pub side_left: Texture2D,
    pub side_right: Texture2D,
    pub side_top: Texture2D,
    pub straight_left_right: Texture2D,
    pub straight_top_bottom: Texture2D,
}


pub fn get_tile_set() -> TileSet {
    let png = Some(ImageFormat::Png);
    TileSet {
        center: Texture2D::from_file_with_format(CENTER_TILE_BYTES, png),
        corner_bottom_left: Texture2D::from_file_with_format(CORNER_BOTTOM_LEFT, png),
        corner_bottom_right: Texture2D::from_file_with_format(CORNER_BOTTOM_RIGHT, png),
        corner_top_left: Texture2D::from_file_with_format(CORNER_TOP_LEFT, png),
        corner_top_right: Texture2D::from_file_with_format(CORNER_TOP_RIGHT, png),
        peninsula_bottom: Texture2D::from_file_with_format(PENINSULA_BOTTOM, png),
        peninsula_left: Texture2D::from_file_with_format(PENINSULA_LEFT, png),
        peninsula_right: Texture2D::from_file_with_format(PENINSULA_RIGHT, png),
        peninsula_top: Texture2D::from_file_with_format(PENINSULA_TOP, png),
        side_bottom: Texture2D::from_file_with_format(SIDE_BOTTOM, png),
        side_left: Texture2D::from_file_with_format(SIDE_LEFT, png),
        side_right: Texture2D::from_file_with_format(SIDE_RIGHT, png),
        side_top: Texture2D::from_file_with_format(SIDE_TOP, png),
        straight_left_right: Texture2D::from_file_with_format(STRAIGHT_LEFT_RIGHT, png),
        straight_top_bottom: Texture2D::from_file_with_format(STRAIGHT_TOP_BOTTOM, png),
    }
}

/// Total tiles in a map chunk
const CHUNK_SIZE: usize = 100;

/// # Arguments
///
/// * `p` - The probability that a given tile will be `true`
///
/// # Returns
///
/// A section of the map of size `CHUNK_SIZE`. For now, just a bunch of `true`s
/// and `false`s.
pub fn gen_map_chunk(p: f32) -> [bool; CHUNK_SIZE] {
    let mut map_chunk: [bool; CHUNK_SIZE] = [false; CHUNK_SIZE];
    let mut random: ThreadRng = ThreadRng::default();

    // If below the threshold, tile is true
    let threshold = ((p as f64) * (std::u32::MAX as f64)).trunc() as u32;

    for i in 0..CHUNK_SIZE {
        map_chunk[i] = random.next_u32() < threshold;
    }
    return map_chunk;
}

/// Draw a section of the map to the screen
///
/// # Arguments
///
/// * `chunk` - The chunk to draw
/// * `_tiles` - The tileset to draw with
/// * `looking_at` - Where the camera is looking
/// * `looking_zoom` - How zoomed the camera is. Smaller zoom value means
///     more zoomed in.
pub fn draw_map_chunk(
    chunk: &[bool; CHUNK_SIZE],
    _tiles: &TileSet,
    looking_at: &Vec2,
    looking_zoom: &f32,
) {
    // Scale tile dimensions by zoom level
    let zoomed_tile_dim: f32 = TILE_DIM as f32 * looking_zoom;
    let draw_params: DrawTextureParams = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: zoomed_tile_dim,
            y: zoomed_tile_dim,
        }),
        ..Default::default()
    };

    for i in 0..CHUNK_SIZE {
        // If there is no tile, do not draw
        if !chunk[i] {
            continue;
        }

        // Position tile
        let x: f32 = (i % 10) as f32 * zoomed_tile_dim - looking_at.x;
        let y: f32 = (i / 10) as f32 * zoomed_tile_dim - looking_at.y;

        // Detect if on an edge or adjacent to other tiles
        let left_adjacent: bool = (i % 10 == 0) || chunk[i - 1];
        let right_adjacent: bool = (i % 10 == 9) || chunk[i + 1];
        let top_adjacent: bool = (i < 10) || chunk[i - 10];
        let bot_adjacent: bool = (89 < i) || chunk[i + 10];

        // Draw the right tile sprite for the given adjacency
        match (left_adjacent, right_adjacent, top_adjacent, bot_adjacent) {
            (true, true, true, true) => {
                draw_texture_ex(&_tiles.center, x, y, WHITE, draw_params.clone())
            }
            (true, true, true, false) => {
                draw_texture_ex(&_tiles.side_bottom, x, y, WHITE, draw_params.clone())
            }
            (true, true, false, true) => {
                draw_texture_ex(&_tiles.side_top, x, y, WHITE, draw_params.clone())
            }
            (true, true, false, false) => draw_texture_ex(
                &_tiles.straight_left_right,
                x,
                y,
                WHITE,
                draw_params.clone(),
            ),
            (true, false, true, true) => {
                draw_texture_ex(&_tiles.side_right, x, y, WHITE, draw_params.clone())
            }
            (true, false, true, false) => draw_texture_ex(
                &_tiles.corner_bottom_right,
                x,
                y,
                WHITE,
                draw_params.clone(),
            ),
            (true, false, false, true) => {
                draw_texture_ex(&_tiles.corner_top_right, x, y, WHITE, draw_params.clone())
            }
            (true, false, false, false) => {
                draw_texture_ex(&_tiles.peninsula_left, x, y, WHITE, draw_params.clone())
            }
            (false, true, true, true) => {
                draw_texture_ex(&_tiles.side_left, x, y, WHITE, draw_params.clone())
            }
            (false, true, true, false) => {
                draw_texture_ex(&_tiles.corner_bottom_left, x, y, WHITE, draw_params.clone())
            }
            (false, true, false, true) => {
                draw_texture_ex(&_tiles.corner_top_left, x, y, WHITE, draw_params.clone())
            }
            (false, true, false, false) => {
                draw_texture_ex(&_tiles.peninsula_right, x, y, WHITE, draw_params.clone())
            }
            (false, false, true, true) => draw_texture_ex(
                &_tiles.straight_top_bottom,
                x,
                y,
                WHITE,
                draw_params.clone(),
            ),
            (false, false, true, false) => {
                draw_texture_ex(&_tiles.peninsula_top, x, y, WHITE, draw_params.clone())
            }
            (false, false, false, true) => {
                draw_texture_ex(&_tiles.peninsula_bottom, x, y, WHITE, draw_params.clone())
            }
            (false, false, false, false) => (),
        }
    }
}
