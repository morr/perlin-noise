use bevy::prelude::*;

mod camera;
use crate::camera::CameraPlugin;

mod grid;
use crate::grid::GridPlugin;

mod debug_grid;
use crate::debug_grid::DebugGridPlugin;

mod noise;
use crate::noise::NoisePlugin;

const GRID_SIZE: i32 = 100;
const TILE_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, GridPlugin, DebugGridPlugin, NoisePlugin))
        .run();
}
