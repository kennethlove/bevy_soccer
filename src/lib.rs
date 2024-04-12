pub mod animation;
pub mod arena;
pub mod ball;
pub mod camera;
pub mod constants;
pub mod obstacle;
pub mod player;
pub mod ui;
mod utils;

pub mod prelude {
    pub use crate::animation::AnimationPlugin;
    pub use crate::arena::ArenaPlugin;
    pub use crate::ball::BallPlugin;
    pub use crate::camera::CameraPlugin;
    pub use crate::constants::*;
    pub use crate::obstacle::ObstaclePlugin;
    pub use crate::player::PlayerPlugin;
    pub use crate::ui::UIPlugin;
}
