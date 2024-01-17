use macroquad::math::Vec2;

// (0, 0) but in float
pub const ZERO_ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

pub const NO: bool = false;

pub const FULL_PF32: f32 = 1.0;
pub const HALF_PF32: f32 = 0.5;

pub const GHOUL_MAX_HEALTH: f32 = 100.0;
pub const GHOUL_INIT_HEALTH: f32 = FULL_PF32 * GHOUL_MAX_HEALTH;
pub const GHOUL_SPEED: f32 = 2.5;
pub const GHOUL_POWER: f32 = 10.0;

pub const PHANTOM_MAX_HEALTH: f32 = 200.0;
pub const PHANTOM_INIT_HEALTH: f32 = FULL_PF32 * PHANTOM_MAX_HEALTH;
pub const PHANTOM_POWER: f32 = 20.0;
pub const PHANTOM_SPEED: f32 = 1.75;

pub const DRINKER_MAX_HEALTH: f32 = 150.0;
pub const DRINKER_INIT_HEALTH: f32 = HALF_PF32 * DRINKER_MAX_HEALTH;
pub const DRINKER_POWER: f32 = 7.5;
pub const DRINKER_SPEED: f32 = 3.5;

pub const CRAWLER_MAX_HEALTH: f32 = 50.0;
pub const CRAWLER_INIT_HEALTH: f32 = FULL_PF32 * CRAWLER_MAX_HEALTH;
pub const CRAWLER_POWER: f32 = 33.5;
pub const CRAWLER_SPEED: f32 = 8.0;