
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

mod camera;
use crate::camera::CameraPlugin;

mod grid;
use crate::grid::GridPlugin;

mod debug_grid;
use crate::debug_grid::DebugGridPlugin;

mod noise;
use crate::noise::NoisePlugin;

const GRID_SIZE: i32 = 250;
const TILE_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, GridPlugin, DebugGridPlugin, NoisePlugin))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 14.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                },
                // We can also change color of the overlay
                text_color: Color::srgba(1.0, 1.0, 1.0, 0.6),
                enabled: true,
            },
        })
        .run();
}
