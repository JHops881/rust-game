use macroquad::prelude::*;
use std::sync::OnceLock;

static TILE_ATLAS: OnceLock<Texture2D> = OnceLock::new();

/// Load in the tile atlas for use in future calls
pub fn init_tile_atlas() {
    TILE_ATLAS.set(Texture2D::from_file_with_format(
        include_bytes!("../assets/atlas/atlas1.png"),
        Some(ImageFormat::Png),
    )).expect("Initializing the tile atlas failed.");
    // build_textures_atlas();
}

/// # Returns
/// 
/// The tile atlas
fn get_tile_atlas() -> &'static Texture2D {
    match TILE_ATLAS.get() {
        Some(atlas) => atlas,
        None => panic!("Tile atlas has not been initialized."),
    }
}

pub enum Tile {
    GroundCenter,
}

impl Tile {
    // Get the texture coordinates for the top left corner of this tile in the
    // texture atlas.
    fn get_texture_coords(&self) -> (f32, f32) {
        match self {
            Tile::GroundCenter => (0.0, 0.0),
        }
    } 
}

// Draw a tile to the screen 
pub fn draw_tile(tile: Tile, position: Vec2) {
    
    const TILE_SIDE_LENGTH: f32 = 16.0;
    const SCALE: f32 = 10.0;

    let (x, y) = tile.get_texture_coords();

    let draw_params = DrawTextureParams {
        dest_size: Some(Vec2{ x: TILE_SIDE_LENGTH * SCALE, y: TILE_SIDE_LENGTH * SCALE }),
        source: Some(Rect{x, y, w: TILE_SIDE_LENGTH, h: TILE_SIDE_LENGTH}),
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None
    };

    let color = Color::from_rgba(0x0b, 0x66, 0x23, 0xff);

    draw_texture_ex(get_tile_atlas(), position.x, position.y, color, draw_params);
}