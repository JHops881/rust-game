use macroquad::prelude::*;

const CENTER_TILE_BYTES: &'static [u8; 155] = include_bytes!("../assets/tiles/center.png");
const CORNER_BOTTOM_LEFT: &'static [u8; 196] = include_bytes!("../assets/tiles/corner-bottom-left.png");
const CORNER_BOTTOM_RIGHT: &'static [u8; 184] = include_bytes!("../assets/tiles/corner-bottom-right.png");
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
const STRAIGHT_LEFT_RIGHT: &'static [u8; 200] = include_bytes!("../assets/tiles/straight-left-right.png");
const STRAIGHT_TOP_BOTTOM: &'static [u8; 223] = include_bytes!("../assets/tiles/straight-top-bottom.png");

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
    pub side_right : Texture2D,
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