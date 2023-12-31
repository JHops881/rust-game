use ::rand::{rngs::ThreadRng, Rng, RngCore};
use macroquad::math::Vec2;

use crate::tiles::*;

/// Total tiles in a map chunk
const CHUNK_DIM: usize = 20;
const CHUNK_SIZE: usize = CHUNK_DIM * CHUNK_DIM;
const P: f32 = 0.80; // TODO Get rid of this.

pub struct Chunk {
    pub data: [Option<Tile>; CHUNK_SIZE],
}

impl Chunk {
    /// Create a new chunk of the map
    pub fn new() -> Self {
        let mut chunk: [bool; CHUNK_SIZE] = [false; CHUNK_SIZE];
        let mut random: ThreadRng = ThreadRng::default();

        // If below the threshold, tile is true
        let threshold = ((P as f64) * (std::u32::MAX as f64)).trunc() as u32;

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

            arr[i] = Some(Tile::new(
                TileType::Ground,
                SurroundingTiles {
                    top_left: if top_left {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    top: if top {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    top_right: if top_right {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    left: if left {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    right: if right {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    bot_left: if bot_left {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    bot: if bot {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                    bot_right: if bot_right {
                        TileType::Ground
                    } else {
                        TileType::None
                    },
                },
            ));
        }

        return Self { data: arr };
    }

    /// Display this chunk to the screen
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
