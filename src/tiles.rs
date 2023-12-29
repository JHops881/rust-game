use macroquad::prelude::*;

const CENTER_TILE_BYTES: &'static [u8; 155] = include_bytes!("../assets/tiles/center.png");
const CORNER_BOTTOM_LEFT: &'static [u8; 196] = include_bytes!("../assets/tiles/corner-bottom-left.png");
const CORNER_BOTTOM_RIGHT: &'static [u8; 184] = include_bytes!("../assets/tiles/corner-bottom-right.png");
const CORNER_TOP_LEFT: &'static [u8; 184] = include_bytes!("../assets/tiles/corner-top-left.png");
const CORNER_TOP_RIGHT: &'static [u8; 196] = include_bytes!("../assets/tiles/corner-top-right.png");
const PENINSULA_BOTTOM: &'static [u8; 223] = include_bytes!("../assets/tiles/peninsula-bottom.png");
const PENINSULA_LEFT: &'static [u8; 220] = include_bytes!("../assets/tiles/peninsula-left.png");
const PENINSULA_RIGHT: &'static [u8; 235] = include_bytes!("../assets/tiles/peninsula-right.png");
const PENINSULA_TOP: &'static [u8; 235] = include_bytes!("../assets/tiles/peninsula-top.png");
const SIDE_BOTTOM: &'static [u8; 171] = include_bytes!("../assets/tiles/side-bottom.png");
const SIDE_LEFT: &'static [u8; 181] = include_bytes!("../assets/tiles/side-left.png");
const SIDE_RIGHT: &'static [u8; 129] = include_bytes!("../assets/tiles/side-right.png");
const SIDE_TOP: &'static [u8; 181] = include_bytes!("../assets/tiles/side-top.png");
const STRAIGHT_LEFT_RIGHT: &'static [u8; 223] = include_bytes!("../assets/tiles/straight-left-right.png");
const STRAIGHT_TOP_BOTTOM: &'static [u8; 223] = include_bytes!("../assets/tiles/straight-top-bottom.png");

struct TileSet {
    center: Image,
    corner_bottom_left: Image,
    corner_bottom_right: Image,
    corner_top_left: Image,
    corner_top_right: Image,
    peninsula_bottom: Image,
    peninsula_left: Image,
    peninsula_right: Image,
    peninsula_top: Image,
    side_bottom: Image,
    side_left: Image,
    side_right : Image,
    side_top: Image,
    straight_left_right: Image,
    straight_top_bottom: Image,
}

fn get_tile_set() -> TileSet {
    let png = Some(ImageFormat::Png);
    TileSet {
        center: Image::from_file_with_format(CENTER_TILE_BYTES, png).unwrap(),
        corner_bottom_left: Image::from_file_with_format(CORNER_BOTTOM_LEFT, png).unwrap(),
        corner_bottom_right: Image::from_file_with_format(CORNER_BOTTOM_RIGHT, png).unwrap(),
        corner_top_left: Image::from_file_with_format(CORNER_TOP_LEFT, png).unwrap(),
        corner_top_right: Image::from_file_with_format(CORNER_TOP_RIGHT, png).unwrap(),
        peninsula_bottom: Image::from_file_with_format(PENINSULA_BOTTOM, png).unwrap(),
        peninsula_left: Image::from_file_with_format(PENINSULA_LEFT, png).unwrap(),
        peninsula_right: Image::from_file_with_format(PENINSULA_RIGHT, png).unwrap(),
        peninsula_top: Image::from_file_with_format(PENINSULA_TOP, png).unwrap(),
        side_bottom: Image::from_file_with_format(SIDE_BOTTOM, png).unwrap(),
        side_left: Image::from_file_with_format(SIDE_LEFT, png).unwrap(),
        side_right: Image::from_file_with_format(SIDE_RIGHT, png).unwrap(),
        side_top: Image::from_file_with_format(SIDE_TOP, png).unwrap(),
        straight_left_right: Image::from_file_with_format(STRAIGHT_LEFT_RIGHT, png).unwrap(),
        straight_top_bottom: Image::from_file_with_format(STRAIGHT_TOP_BOTTOM, png).unwrap(),
    }
}
