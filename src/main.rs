use bevy::prelude::*;

mod camera;
use crate::camera::CameraPlugin;

mod grid;
use crate::grid::GridPlugin;

mod debug_grid;
use crate::debug_grid::DebugGridPlugin;

const GRID_SIZE: i32 = 100;
const TILE_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, GridPlugin, DebugGridPlugin))
        // .init_resource::<NoiseSettings>()
        // .add_systems(Startup, setup)
        // .add_systems(Update, (update_noise, keyboard_input_system))
        .run();
}
