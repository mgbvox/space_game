use bevy::math::Vec3;
use bevy::color::Color;

pub const DELTA: f32 = 1.0;
pub const SPEED: f32 = 500.0;
pub const DOT_SPAWN_DELTA: f32 = 0.03;
pub const DOT_DECAY_DELTA: f32 = 0.01;
pub const DOT_DECAY_RATE: f32 = 0.93;
pub const DOT_DECAY_DESPAWN_THRESHOLD: f32 = 0.05;
pub const PLAYER_COLOR: Color = Color::srgba(0.8, 0.2, 0.1, 1.0);
pub const PLAYER_INITIAL_POSITION: Vec3 = Vec3::splat(0.);
pub const PLAYER_INITIAL_VELOCITY: Vec3 = Vec3::splat(0.);

pub const DESIRED_DOT_DISTANCE: f32 = 0.1;