use ::rand::{rngs::ThreadRng, Rng, RngCore};
use macroquad::math::{Vec2, IVec2};

use crate::tiles::*;

/// Side length of a chunk in tiles.
const CHUNK_DIM: usize = 16;

/// Total tiles in a chunk.
const CHUNK_SIZE: usize = CHUNK_DIM * CHUNK_DIM;

// TODO Get rid of this.
const P: f32 = 0.50; 

/// A section of the map.
pub struct Chunk {
    position: IVec2,
    pub data: [Option<Tile>; CHUNK_SIZE],
}

impl Chunk {

    /// Create a new chunk of the map
    pub fn new(position: IVec2) -> Self {

        const NONE_TILE: TileType = TileType::None;
        let mut type_map: [TileType; CHUNK_SIZE] = [NONE_TILE; CHUNK_SIZE];

        let mut random: ThreadRng = ThreadRng::default();

        // If below the threshold, tile is true
        // TODO: Rework chunk generation
        let threshold = ((P as f64) * (std::u32::MAX as f64)).trunc() as u32;

        // Boolean map
        for i in 0..CHUNK_SIZE {
            type_map[i] = if random.next_u32() < threshold {
                TileType::Ground
            } else {
                TileType::Wood
            };
        }

        const NO_TILE: Option<Tile> = None::<Tile>;
        let mut chunk_data: [Option<Tile>; CHUNK_SIZE] = [NO_TILE; CHUNK_SIZE];

        for i in 0..CHUNK_SIZE {
            if type_map[i] == TileType::None {
                continue;
            }

            // Detect chunk boundries
            let top_edge = i < CHUNK_DIM;
            let bot_edge = (CHUNK_SIZE - CHUNK_DIM - 1) < i;
            let left_edge = i % CHUNK_DIM == 0;
            let right_edge = i % CHUNK_DIM == (CHUNK_DIM - 1);

            // This is the logic that decides how to draw a tile depending on 
            // the 8 tiles that may or may not surround it.  
            let top_left = !top_edge && !left_edge && type_map[i - CHUNK_DIM - 1] != TileType::None;
            let top = !top_edge && type_map[i - CHUNK_DIM] != TileType::None;
            let top_right = !top_edge && !right_edge && type_map[i - CHUNK_DIM + 1] != TileType::None;
            let left = !left_edge && type_map[i - 1] != TileType::None;
            let right = !right_edge && type_map[i + 1] != TileType::None;
            let bot_left = !bot_edge && !left_edge && type_map[i + CHUNK_DIM - 1] != TileType::None;
            let bot = !bot_edge && type_map[i + CHUNK_DIM] != TileType::None;
            let bot_right = !bot_edge && !right_edge && type_map[i + CHUNK_DIM + 1] != TileType::None;

            // This builds a tile using the surrounding tiles data.
            chunk_data[i] = Some(Tile::new(
                type_map[i],
                SurroundingTiles {
                    top_left: if top_left {
                        type_map[i - CHUNK_DIM - 1]
                    } else {             
                        TileType::None
                    },
                    top: if top {
                        type_map[i - CHUNK_DIM]
                    } else {
                        TileType::None
                    },
                    top_right: if top_right {
                        type_map[i - CHUNK_DIM + 1]
                    } else {
                        TileType::None
                    },
                    left: if left {
                        type_map[i - 1]
                    } else {
                        TileType::None
                    },
                    right: if right {
                        type_map[i + 1]
                    } else {
                        TileType::None
                    },
                    bot_left: if bot_left {
                        type_map[i + CHUNK_DIM - 1]
                    } else {
                        TileType::None
                    },
                    bot: if bot {
                        type_map[i + CHUNK_DIM]
                    } else {
                        TileType::None
                    },
                    bot_right: if bot_right {
                        type_map[i + CHUNK_DIM + 1]
                    } else {
                        TileType::None
                    },
                },
            ));
        }

        return Self { 
            position,
            data: chunk_data,
        };
    }

    /// Display this chunk to the screen
    pub fn draw(&self) {
        for item in self.data.iter().enumerate() {
            let (i, tile) = item;
            let x = (i % CHUNK_DIM) as f32 * SCALE * TILE_SIDE_LENGTH;
            let y = (i / CHUNK_DIM) as f32 * SCALE * TILE_SIDE_LENGTH;
            let x_chunk_offset = self.position.x as f32 * TILE_SIDE_LENGTH * CHUNK_DIM as f32 * SCALE;
            let y_chunk_offset = self.position.y as f32 * TILE_SIDE_LENGTH * CHUNK_DIM as f32 * SCALE;
            
            match tile {
                Some(tile) => tile.draw(Vec2 { x: x + x_chunk_offset, y: y + y_chunk_offset }),
                None => (),
            }
        }
    }
}
