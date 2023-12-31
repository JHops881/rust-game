use ::rand::{
    distributions::{Distribution, Standard},
    rngs::ThreadRng,
    Rng,
};
use macroquad::prelude::*;
use std::{f32::consts::PI, sync::OnceLock};

pub const TILE_SIDE_LENGTH: f32 = 16.0;
pub const SCALE: f32 = 2.0;
pub const TILE_DIMENSIONS: macroquad::math::Vec2 = Vec2 {
    x: TILE_SIDE_LENGTH * SCALE,
    y: TILE_SIDE_LENGTH * SCALE,
};

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
}

/// Returns the tile atlas
fn get_tile_atlas() -> &'static Texture2D {
    match TILE_ATLAS.get() {
        Some(atlas) => atlas,
        None => panic!("Tile atlas has not been initialized."),
    }
}

/// A single game tile on the ground
pub struct Tile {
    pub tile_type: TileType,
    pub tile_shape: TileShape,
    pub orientation: TileOrientation,
    /// A number that may or may not give the tile a slight different
    /// appearance.
    pub variant: u8,
}

/// Which type of tile will be shown. E.g. ground, water, wood, etc.
#[derive(PartialEq)]
pub enum TileType {
    None,
    Ground,
}

/// The base shape of the tile given how it connects to the tiles around it.
pub enum TileShape {
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
    Island,
}

/// The rotation and mirroring of a tile.
pub enum TileOrientation {
    Default,
    Rotated90,
    Rotated180,
    Rotated270,
    MirroredDefault,
    MirroredRotated90,
    MirroredRotated180,
    MirroredRotated270,
}

/// Allow for random rotations of tiles.
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

pub struct SurroundingTiles {
    pub top_left: TileType,
    pub top: TileType,
    pub top_right: TileType,
    pub left: TileType,
    pub right: TileType,
    pub bot_left: TileType,
    pub bot: TileType,
    pub bot_right: TileType,
}

impl Tile {

    /// Create a new tile given the context
    pub fn new(tile_type: TileType, around: SurroundingTiles) -> Self {
        let mut tile_shape = TileShape::Center;
        let mut orientation = TileOrientation::Default;
        let mut variant = 0;

        let top_left: bool = around.top_left == tile_type;
        let top: bool = around.top == tile_type;
        let top_right: bool = around.top_right == tile_type;
        let left: bool = around.left == tile_type;
        let right: bool = around.right == tile_type;
        let bot_left: bool = around.bot_left == tile_type;
        let bot: bool = around.bot == tile_type;
        let bot_right: bool = around.bot_right == tile_type;

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
                tile_shape = TileShape::Island;
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

        Self {
            tile_type,
            tile_shape,
            orientation,
            variant,
        }
    }
    /// Get the texture coordinates for the top left corner of this tile in the
    /// texture atlas.
    fn get_texture_coords(&self) -> (f32, f32) {
        use TileShape::*;
        use TileType::*;
        match (&self.tile_type, &self.tile_shape) {
            (None, _) => (112.0, 112.0),
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
            (Ground, Island) => (80.0, 32.0),
        }
    }

    /// Get the numerical data of the tile's rotation given it's orientation.
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
        draw_rectangle_lines(
            position.x,
            position.y,
            TILE_SIDE_LENGTH * SCALE,
            TILE_SIDE_LENGTH * SCALE,
            1.0,
            Color::from_rgba(0x00, 0xff, 0x00, 0x80),
        )
    }
}
