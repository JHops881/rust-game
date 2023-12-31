use macroquad::prelude::*;
use ::rand::{rngs::ThreadRng, RngCore, distributions::{Distribution, Standard}, Rng, random};
use std::{f32::consts::PI, sync::OnceLock};

static TILE_ATLAS: OnceLock<Texture2D> = OnceLock::new();

/// Load in the tile atlas for use in future calls
pub fn init_tile_atlas() {
    let atlas = Texture2D::from_file_with_format(
        include_bytes!("../assets/atlas/atlas2.png"),
        Some(ImageFormat::Png),
    );
    atlas.set_filter(FilterMode::Nearest);
    TILE_ATLAS
        .set(atlas)
        .expect("Initializing the tile atlas failed.");
    // build_textures_atlas(); // Unsure how this works or if it is even necessary.
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

enum TileOrientation {
    Default,
    Rotated90,
    Rotated180,
    Rotated270,
    MirroredDefault,
    MirroredRotated90,
    MirroredRotated180,
    MirroredRotated270,
}

impl Distribution<TileOrientation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileOrientation {
        match rng.gen_range(0..=8) { // rand 0.8
            0 => TileOrientation::Default,
            1 => TileOrientation::Rotated90,
            2 => TileOrientation::Rotated180,
            3 => TileOrientation::Rotated270,
            4 => TileOrientation::MirroredDefault,
            5 => TileOrientation::MirroredRotated90,
            6 => TileOrientation::MirroredRotated180,
            _ => TileOrientation::MirroredRotated270,
        }
    }
}

enum TileShape {
    Center,
    DentedCenter,
    DoubleDentedCenter,
    CrossDentedCenter,
    TripleDentedCenter,
    Junction,
    Edge,
    DentedEdge,
    DoubleDentedEdge,
    Straight,
    Peninsula,
    Corner,
    DentedCorner,
}

enum TileType {
    None,
    Ground,
}

pub struct Tile {
    _type: TileType,
    shape: TileShape,
    orientation: TileOrientation,
    variant: u8,
}

impl Tile {
    // Get the texture coordinates for the top left corner of this tile in the
    // texture atlas.
    fn get_texture_coords(&self) -> (f32, f32) {
        use TileType::*;
        use TileShape::*;
        match (&self._type, &self.shape) {
            (None, _) => panic!("None tile has no texture"),
            (Ground, Center) => (0.0 + (self.variant as f32 * 16.0), 0.0),
            (Ground, DentedCenter) => (0.0, 32.0),
            (Ground, DoubleDentedCenter) => (16.0, 32.0),
            (Ground, CrossDentedCenter) => (48.0, 32.0),
            (Ground, TripleDentedCenter) => (32.0, 32.0),
            (Ground, Junction) => (64.0, 32.0),
            (Ground, Edge) => (32.0, 16.0),
            (Ground, DentedEdge) => (48.0, 16.0),
            (Ground, DoubleDentedEdge) => (64.0, 16.0),
            (Ground, Straight) => (80.0, 16.0),
            (Ground, Peninsula) => (96.0, 16.0),
            (Ground, Corner) => (112.0, 16.0),
            (Ground, DentedCorner) => (112.0, 32.0),
        }
    }

    fn get_texture_rotation(&self) -> (f32, bool, bool) {
        match &self.orientation {
            TileOrientation::Default => (0.0, false, false),
            TileOrientation::Rotated90 => (PI / 2.0, false, false),
            TileOrientation::Rotated180 => (PI, false, false),
            TileOrientation::Rotated270 => (3.0 * PI / 2.0, false, false),
            TileOrientation::MirroredDefault => (0.0, true, false),
            TileOrientation::MirroredRotated90 => (PI / 2.0, true, false),
            TileOrientation::MirroredRotated180 => (PI, true, false),
            TileOrientation::MirroredRotated270 => (3.0 * PI / 2.0, true, false),
        }
    }
}

pub fn draw_map_test(chunk: [bool; CHUNK_SIZE]) {
    for i in 0..CHUNK_SIZE {
        
        if !chunk[i] {
            continue;
        }

        // Position tile
        let x: f32 = (i % 10) as f32 * 64.0;
        let y: f32 = (i / 10) as f32 * 64.0;

        let west_edge = i % 10 == 0; 
        let east_edge = i % 10 == 9; 
        let north_edge = i < 10; 
        let south_edge = 89 < i; 

        let west: bool = !west_edge && chunk[i - 1];
        let east: bool = !east_edge && chunk[i + 1];
        let north: bool = !north_edge && chunk[i - 10];
        let south: bool = !south_edge && chunk[i + 10];

        let nw: bool = !north_edge && !west_edge && chunk[i - 11];
        let ne: bool = !north_edge && !east_edge && chunk[i - 9];
        let sw: bool = !south_edge && !west_edge && chunk[i + 9];
        let se: bool = !south_edge && !east_edge && chunk[i + 11];

        // Tile adjacency
        // NW   North   NE
        // West         East
        // SW   South   SE
        const T: bool = true;
        const F: bool = false;
        let tile = match (nw, north, ne, west, east, sw, south, se) {

            // Center

            // nw, north, ne, west, east, sw, south, se            
            (T, T, T, T, T, T, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::Center,
                orientation: random(),
                variant: random::<u8>() % 8,
            },

            // DentedCenter

            // nw, north, ne, west, east, sw, south, se            
            (T, T, F, T, T, T, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DentedCenter,
                orientation: TileOrientation::Default,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (F, T, T, T, T, T, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DentedCenter,
                orientation: TileOrientation::Rotated270,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, T, T, T, F, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DentedCenter,
                orientation: TileOrientation::Rotated180,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, T, T, T, T, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DentedCenter,
                orientation: TileOrientation::Rotated90,
                variant: 0,
            },

            // DoubleDentedCenter

            // nw, north, ne, west, east, sw, south, se            
            (F, T, F, T, T, T, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DoubleDentedCenter,
                orientation: TileOrientation::Default,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, F, T, T, T, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DoubleDentedCenter,
                orientation: TileOrientation::Rotated90,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, T, T, T, F, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DoubleDentedCenter,
                orientation: TileOrientation::Rotated180,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (F, T, T, T, T, F, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::DoubleDentedCenter,
                orientation: TileOrientation::Rotated270,
                variant: 0,
            },

            // CrossDentedCenter

            // nw, north, ne, west, east, sw, south, se            
            (T, T, F, T, T, F, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::CrossDentedCenter,
                orientation: TileOrientation::Default,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (F, T, T, T, T, T, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::CrossDentedCenter,
                orientation: TileOrientation::Rotated90,
                variant: 0,
            },

            // TripleDentedCenter

            // nw, north, ne, west, east, sw, south, se            
            (F, T, F, T, T, T, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::TripleDentedCenter,
                orientation: TileOrientation::Default,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, F, T, T, F, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::TripleDentedCenter,
                orientation: TileOrientation::Rotated90,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (F, T, T, T, T, F, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::TripleDentedCenter,
                orientation: TileOrientation::Rotated180,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (F, T, F, T, T, F, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::TripleDentedCenter,
                orientation: TileOrientation::Rotated270,
                variant: 0,
            },

            // Junction

            // nw, north, ne, west, east, sw, south, se            
            (F, T, F, T, T, F, T, F) => Tile {
                _type: TileType::Ground,
                shape: TileShape::Junction,
                orientation: random(),
                variant: 0,
            },

            // Edge 

            // nw, north, ne, west, east, sw, south, se            
            (_, F, _, T, T, T, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::Edge,
                orientation: TileOrientation::Default,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, _, T, F, T, T, _) => Tile {
                _type: TileType::Ground,
                shape: TileShape::Edge,
                orientation: TileOrientation::Rotated90,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (T, T, T, T, T, _, F, _) => Tile {
                _type: TileType::Ground,
                shape: TileShape::Edge,
                orientation: TileOrientation::Rotated180,
                variant: 0,
            },

            // nw, north, ne, west, east, sw, south, se            
            (_, T, T, F, T, _, T, T) => Tile {
                _type: TileType::Ground,
                shape: TileShape::Edge,
                orientation: TileOrientation::Rotated270,
                variant: 0,
            },


            _ => Tile {
                _type: TileType::Ground,
                shape: TileShape::Center,
                orientation: TileOrientation::Default,
                variant: 0,
            }
            
            

            // Edge,
            // DentedEdge,
            // DoubleDentedEdge,
            
            
            // Straight,
            

            // Corner,
            // DentedCorner,
            

            // Peninsula,
            

            // Island
            
        };
        draw_tile(tile, Vec2 { x,  y })
    }

}

/// Draw a tile to the screen
pub fn draw_tile(tile: Tile, position: Vec2) {
    const TILE_SIDE_LENGTH: f32 = 16.0;
    const SCALE: f32 = 4.0;

    let (x, y) = tile.get_texture_coords();
    let (rotation, flip_x, flip_y) = tile.get_texture_rotation();

    let draw_params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: TILE_SIDE_LENGTH * SCALE,
            y: TILE_SIDE_LENGTH * SCALE,
        }),
        source: Some(Rect {
            x,
            y,
            w: TILE_SIDE_LENGTH,
            h: TILE_SIDE_LENGTH,
        }),
        rotation,
        flip_x,
        flip_y,
        pivot: None,
    };

    // let color = Color::from_rgba(0x0b, 0x46, 0x23, 0xff);
    let color = WHITE;
    draw_texture_ex(get_tile_atlas(), position.x, position.y, color, draw_params);
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