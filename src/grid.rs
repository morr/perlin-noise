use super::*;
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_grid);
    }
}

const GRID_SIZE: i32 = 100;
const TILE_SIZE: f32 = 10.0;

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(20.0, 20.0));
    let material = materials.add(Color::srgb(1.0, 1.0, 1.0));

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_xyz(grid_tile_edge_to_world(x), grid_tile_edge_to_world(y), 0.0),
            ));
        }
    }
}

pub fn grid_tile_edge_to_world(value: i32) -> f32 {
    value as f32 * TILE_SIZE - GRID_SIZE as f32 / 2.0 * TILE_SIZE
}
