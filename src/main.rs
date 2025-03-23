use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};

mod camera;
use crate::camera::CameraPlugin;

mod grid;
use crate::grid::GridPlugin;

mod debug_ui;
use crate::debug_ui::DebugUiPlugin;

mod debug_grid;
use crate::debug_grid::DebugGridPlugin;

mod noise_texture;
use crate::noise_texture::NoiseTexturePlugin;

mod noise_settings_ui;
use crate::noise_settings_ui::NoiseSettingsUiPlugin;

const GRID_SIZE: i32 = 1000;
const TILE_SIZE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        mode: bevy::window::WindowMode::Windowed,
                        // present_mode: bevy::window::PresentMode::AutoNoVsync,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        resolution: (1400.0, 1000.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            CameraPlugin,
            GridPlugin,
            NoiseTexturePlugin,
            NoiseSettingsUiPlugin,
        ))
        .add_plugins((DebugGridPlugin, DebugUiPlugin))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 14.0,
                    font: default(),
                    font_smoothing: FontSmoothing::default(),
                },
                text_color: Color::srgba(1.0, 1.0, 1.0, 0.75),
                enabled: true,
            },
        })
        .run();
}
