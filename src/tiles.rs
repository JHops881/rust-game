use ::rand::{
    distributions::{Distribution, Standard},
    rngs::ThreadRng,
    Rng, RngCore,
};
use macroquad::prelude::*;
use std::{f32::consts::PI, sync::OnceLock};

static TILE_ATLAS: OnceLock<Texture2D> = OnceLock::new();

const TILE_SIDE_LENGTH: f32 = 16.0;
const SCALE: f32 = 4.0;
const TILE_DIMENSIONS: macroquad::math::Vec2 = Vec2 {
    x: TILE_SIDE_LENGTH * SCALE,
    y: TILE_SIDE_LENGTH * SCALE,
};

/// Total tiles in a map chunk
const CHUNK_SIZE: usize = 100;

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
        match rng.gen_range(0..=8) {
            // rand 0.8
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
    Ground,
}

pub struct Tile {
    tile_type: TileType,
    tile_shape: TileShape,
    orientation: TileOrientation,
    variant: u8,
}

impl Tile {
    // Get the texture coordinates for the top left corner of this tile in the
    // texture atlas.
    fn get_texture_coords(&self) -> (f32, f32) {
        use TileShape::*;
        use TileType::*;
        match (&self.tile_type, &self.tile_shape) {
            (Ground, Center) => (0.0 + (self.variant as f32 * TILE_SIDE_LENGTH), 0.0),
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

    /// Draw a tile to the screen
    pub fn draw(&self, position: Vec2) {
        let (x, y) = self.get_texture_coords();
        let (rotation, flip_x, flip_y) = self.get_texture_rotation();

        let draw_params = DrawTextureParams {
            dest_size: Some(TILE_DIMENSIONS),
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
}

pub fn draw_map_test(chunk: &[Option<Tile>; CHUNK_SIZE]) {
    for item in chunk.iter().enumerate() {
        let (i, tile) = item;

        let x = (i % 10) as f32 * SCALE * TILE_SIDE_LENGTH;
        let y = (i / 10) as f32 * SCALE * TILE_SIDE_LENGTH;
        match tile {
            Some(tile) => tile.draw(Vec2 { x, y }),
            None => (),
        }
    }
}

/// # Arguments
///
/// * `p` - The probability that a given tile will be `true`
///
/// # Returns
///
/// A section of the map of size `CHUNK_SIZE`. For now, just a bunch of `true`s
/// and `false`s.
pub fn gen_map_chunk(p: f32) -> [Option<Tile>; CHUNK_SIZE] {
    let mut chunk: [bool; CHUNK_SIZE] = [false; CHUNK_SIZE];
    let mut random: ThreadRng = ThreadRng::default();

    // If below the threshold, tile is true
    let threshold = ((p as f64) * (std::u32::MAX as f64)).trunc() as u32;

    // Boolean map
    for i in 0..CHUNK_SIZE {
        chunk[i] = random.next_u32() < threshold;
    }

    const NO_TILE: Option<Tile> = None::<Tile>;
    let mut arr: [Option<Tile>; CHUNK_SIZE] = [NO_TILE; CHUNK_SIZE];

    for i in 0..CHUNK_SIZE {
        if !chunk[i] {
            continue;
        }

        let top_edge = i < 10;
        let bot_edge = 89 < i;
        let left_edge = i % 10 == 0;
        let right_edge = i % 10 == 9;

        let top_left = !top_edge && !left_edge && chunk[i - 11];
        let top = !top_edge && chunk[i - 10];
        let top_right = !top_edge && !right_edge && chunk[i - 9];
        let left = !left_edge && chunk[i - 1];
        let right = !right_edge && chunk[i + 1];
        let bot_left = !bot_edge && !left_edge && chunk[i + 9];
        let bot = !bot_edge && chunk[i + 10];
        let bot_right = !bot_edge && !right_edge && chunk[i + 11];

        let mut tile_type = TileType::Ground;
        let mut tile_shape = TileShape::Center;
        let mut orientation = TileOrientation::Default;
        let mut variant = 0;

        // Get the basic shape and orientation
        match (top, bot, left, right) {
            (true, true, true, true) => {
                // Center tile
                orientation = ThreadRng::default().gen();
                variant = ThreadRng::default().gen::<u8>() % 8;
            }
            (false, true, true, true) => {
                // Edge tile
                tile_shape = TileShape::Edge;
                orientation = TileOrientation::Default;
            }
            (true, false, true, true) => {
                // Edge tile
                tile_shape = TileShape::Edge;
                orientation = TileOrientation::Rotated180;
            }
            (true, true, false, true) => {
                // Edge tile
                tile_shape = TileShape::Edge;
                orientation = TileOrientation::Rotated270;
            }
            (true, true, true, false) => {
                // Edge tile
                tile_shape = TileShape::Edge;
                orientation = TileOrientation::Rotated90;
            }
            (false, false, true, true) => {
                // Straight tile
                tile_shape = TileShape::Straight;
                orientation = TileOrientation::Default;
            }
            (true, true, false, false) => {
                // Straight tile
                tile_shape = TileShape::Straight;
                orientation = TileOrientation::Rotated90;
            }
            (false, true, true, false) => {
                // Corner tile
                tile_shape = TileShape::Corner;
                orientation = TileOrientation::Default;
            }
            (true, false, true, false) => {
                // Corner tile
                tile_shape = TileShape::Corner;
                orientation = TileOrientation::Rotated90;
            }
            (true, false, false, true) => {
                // Corner tile
                tile_shape = TileShape::Corner;
                orientation = TileOrientation::Rotated180;
            }
            (false, true, false, true) => {
                // Corner tile
                tile_shape = TileShape::Corner;
                orientation = TileOrientation::Rotated270;
            }
            (false, false, true, false) => {
                // Peninsula tile
                tile_shape = TileShape::Peninsula;
                orientation = TileOrientation::Default;
            }
            (false, false, false, true) => {
                // Peninsula tile
                tile_shape = TileShape::Peninsula;
                orientation = TileOrientation::Rotated180;
            }
            (true, false, false, false) => {
                // Peninsula tile
                tile_shape = TileShape::Peninsula;
                orientation = TileOrientation::Rotated90;
            }
            (false, true, false, false) => {
                // Peninsula tile
                tile_shape = TileShape::Peninsula;
                orientation = TileOrientation::Rotated270;
            }
            (false, false, false, false) => {
                // Island tile
                tile_shape = TileShape::Junction;
                orientation = TileOrientation::Default;
            }
        };

        match tile_shape {
            TileShape::Center => match (top_left, top_right, bot_left, bot_right) {
                (true, true, true, true) => (),
                (true, true, true, false) => {
                    tile_shape = TileShape::DentedCenter;
                    orientation = TileOrientation::Rotated90;
                }
                (true, true, false, true) => {
                    tile_shape = TileShape::DentedCenter;
                    orientation = TileOrientation::Rotated180;
                }
                (true, true, false, false) => {
                    tile_shape = TileShape::DoubleDentedCenter;
                    orientation = TileOrientation::Rotated180;
                }
                (true, false, true, true) => {
                    tile_shape = TileShape::DentedCenter;
                    orientation = TileOrientation::Default;
                }
                (true, false, true, false) => {
                    tile_shape = TileShape::DoubleDentedCenter;
                    orientation = TileOrientation::Rotated90;
                }
                (true, false, false, true) => {
                    tile_shape = TileShape::CrossDentedCenter;
                    orientation = TileOrientation::Default;
                }
                (true, false, false, false) => {
                    tile_shape = TileShape::TripleDentedCenter;
                    orientation = TileOrientation::Rotated90;
                }
                (false, true, true, true) => {
                    tile_shape = TileShape::DentedCenter;
                    orientation = TileOrientation::Rotated270;
                }
                (false, true, true, false) => {
                    tile_shape = TileShape::CrossDentedCenter;
                    orientation = TileOrientation::Rotated90;
                }
                (false, true, false, true) => {
                    tile_shape = TileShape::DoubleDentedCenter;
                    orientation = TileOrientation::Rotated270;
                }
                (false, true, false, false) => {
                    tile_shape = TileShape::TripleDentedCenter;
                    orientation = TileOrientation::Rotated180;
                }
                (false, false, true, true) => {
                    tile_shape = TileShape::DoubleDentedCenter;
                    orientation = TileOrientation::Default;
                }
                (false, false, true, false) => {
                    tile_shape = TileShape::TripleDentedCenter;
                    orientation = TileOrientation::Default;
                }
                (false, false, false, true) => {
                    tile_shape = TileShape::TripleDentedCenter;
                    orientation = TileOrientation::Rotated270;
                }
                (false, false, false, false) => {
                    tile_shape = TileShape::Junction;
                }
            },
            TileShape::Edge => match orientation {
                TileOrientation::Default => match (bot_left, bot_right) {
                    (true, true) => (),
                    (true, false) => tile_shape = TileShape::DentedEdge,
                    (false, true) => {
                        tile_shape = TileShape::DentedEdge;
                        orientation = TileOrientation::MirroredDefault;
                    }
                    (false, false) => tile_shape = TileShape::DoubleDentedEdge,
                },
                TileOrientation::Rotated90 => match (top_left, bot_left) {
                    (true, true) => (),
                    (true, false) => tile_shape = TileShape::DentedEdge,
                    (false, true) => {
                        tile_shape = TileShape::DentedEdge;
                        orientation = TileOrientation::MirroredRotated90;
                    }
                    (false, false) => tile_shape = TileShape::DoubleDentedEdge,
                },
                TileOrientation::Rotated180 => match (top_right, top_left) {
                    (true, true) => (),
                    (true, false) => tile_shape = TileShape::DentedEdge,
                    (false, true) => {
                        tile_shape = TileShape::DentedEdge;
                        orientation = TileOrientation::MirroredRotated180;
                    }
                    (false, false) => tile_shape = TileShape::DoubleDentedEdge,
                },
                TileOrientation::Rotated270 => match (bot_right, top_right) {
                    (true, true) => (),
                    (true, false) => tile_shape = TileShape::DentedEdge,
                    (false, true) => {
                        tile_shape = TileShape::DentedEdge;
                        orientation = TileOrientation::MirroredRotated270;
                    }
                    (false, false) => tile_shape = TileShape::DoubleDentedEdge,
                },
                _ => (),
            },

            TileShape::Corner => match orientation {
                TileOrientation::Default => {
                    if !bot_left {
                        tile_shape = TileShape::DentedCorner;
                    }
                }
                TileOrientation::Rotated90 => {
                    if !top_left {
                        tile_shape = TileShape::DentedCorner;
                    }
                }
                TileOrientation::Rotated180 => {
                    if !top_right {
                        tile_shape = TileShape::DentedCorner;
                    }
                }
                TileOrientation::Rotated270 => {
                    if !bot_right {
                        tile_shape = TileShape::DentedCorner;
                    }
                }
                _ => (),
            },
            _ => (),
        }

        arr[i] = Some(Tile {
            tile_type,
            tile_shape,
            orientation,
            variant,
        });
    }

    return arr;
}
