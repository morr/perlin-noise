use super::*;
use super::noise::NoiseControlled;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
    }
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
                Transform::from_xyz(grid_tile_center_to_world(x), grid_tile_center_to_world(y), 0.0),
                NoiseControlled { position: (x, y) },
            ));
        }
    }
}

pub fn grid_tile_center_to_world(value: i32) -> f32 {
    grid_tile_edge_to_world(value) + TILE_SIZE / 2.0
}

pub fn grid_tile_edge_to_world(value: i32) -> f32 {
    value as f32 * TILE_SIZE - GRID_SIZE as f32 / 2.0 * TILE_SIZE
}
