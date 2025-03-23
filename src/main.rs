use bevy::prelude::*;

mod camera;
use crate::camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        // .init_resource::<NoiseSettings>()
        // .add_systems(Startup, setup)
        // .add_systems(Update, (update_noise, keyboard_input_system))
        .run();
}
