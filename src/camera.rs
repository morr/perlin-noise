use super::*;
use bevy_pancam::PanCam;
use bevy_pancam::PanCamPlugin;

#[derive(Component)]
struct WorldCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera2d,
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scale: 1.25,
                ..OrthographicProjection::default_2d()
            },
            Name::new("camera"),
            WorldCamera,
            Msaa::Off,
        ))
        .insert(PanCam {
            enabled: true,
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            move_keys: bevy_pancam::DirectionKeys {
                up: vec![KeyCode::KeyW],
                down: vec![KeyCode::KeyS],
                left: vec![KeyCode::KeyA],
                right: vec![KeyCode::KeyD],
            },
            speed: 600., // the speed for the keyboard movement
            max_scale: 20.0,
            max_x: f32::INFINITY,
            max_y: f32::INFINITY,
            min_scale: 0.1, // 0.5,
            min_x: f32::NEG_INFINITY,
            min_y: f32::NEG_INFINITY,
            zoom_to_cursor: true,
        });
}
