use bevy::prelude::*;

// Release?
// pub const WINDOW_WIDTH: f32 = 1200.0;
// pub const WINDOW_HEIGHT: f32 = 800.0;
// pub const GROUND_SIZE_WIDTH: f32 = 1200.0;
// pub const GROUND_SIZE_HEIGHT: f32 = 750.0;
// pub const GROUND_OFFSET: Vec3 = Vec3::new(0., 25., 0.);

// Development
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 400.0;
pub const UI_HEIGHT: f32 = 50.0;
pub const GROUND_WIDTH: f32 = WINDOW_WIDTH;
pub const GROUND_HEIGHT: f32 = WINDOW_HEIGHT - UI_HEIGHT;
pub const GROUND_MIDDLE: f32 = (WINDOW_HEIGHT - GROUND_HEIGHT) / 2.;
pub const GROUND_OFFSET: Vec3 = Vec3::new(0., UI_HEIGHT / 2., 0.);
