use macroquad::math::Vec2;
use ::rand::{
    rngs::ThreadRng,
    Rng, RngCore,
};

use crate::tiles::*;

/// Total tiles in a map chunk
const CHUNK_DIM: usize = 20;
const CHUNK_SIZE: usize = CHUNK_DIM * CHUNK_DIM;

pub struct Chunk {
    pub data: [Option<Tile>; CHUNK_SIZE]
}

impl Chunk {
    pub fn draw(&self) {
        for item in self.data.iter().enumerate() {
            let (i, tile) = item;
    
            let x = (i % CHUNK_DIM) as f32 * SCALE * TILE_SIDE_LENGTH;
            let y = (i / CHUNK_DIM) as f32 * SCALE * TILE_SIDE_LENGTH;
            match tile {
                Some(tile) => tile.draw(Vec2 { x, y }),
                None => (),
            }
        }
    }
}


/// Create a new chunk of the map
pub fn gen_map_chunk(p: f32) -> Chunk {
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

        let top_edge = i < CHUNK_DIM;
        let bot_edge = (CHUNK_SIZE - CHUNK_DIM - 1) < i;
        let left_edge = i % CHUNK_DIM == 0;
        let right_edge = i % CHUNK_DIM == (CHUNK_DIM - 1);

        let top_left = !top_edge && !left_edge && chunk[i - CHUNK_DIM - 1];
        let top = !top_edge && chunk[i - CHUNK_DIM];
        let top_right = !top_edge && !right_edge && chunk[i - CHUNK_DIM + 1];
        let left = !left_edge && chunk[i - 1];
        let right = !right_edge && chunk[i + 1];
        let bot_left = !bot_edge && !left_edge && chunk[i + CHUNK_DIM - 1];
        let bot = !bot_edge && chunk[i + CHUNK_DIM];
        let bot_right = !bot_edge && !right_edge && chunk[i + CHUNK_DIM + 1];

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

        arr[i] = Some(Tile {
            tile_type,
            tile_shape,
            orientation,
            variant,
        });
    }

    return Chunk {data: arr};
}
