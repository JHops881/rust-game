use ::rand::{
    distributions::{Distribution, Standard},
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
pub enum TileType {
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

impl Tile {
    /// Get the texture coordinates for the top left corner of this tile in the
    /// texture atlas.
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
