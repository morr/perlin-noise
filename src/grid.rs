// grid.rs
use super::*;
// Remove unused imports - we don't need these anymore since setup_noise_texture moved

#[derive(Resource)]
pub struct NoiseTextureHandle(pub Handle<Image>);

#[derive(Component)]
pub struct NoiseTexture;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GridState {
    #[default]
    WaitingForTexture,
    Ready,
}

#[derive(Event)]
pub struct TextureReadyEvent;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GridState>()
            .add_event::<TextureReadyEvent>()
            .add_systems(Update, handle_texture_ready_event)
            .add_systems(Update, spawn_grid.run_if(in_state(GridState::Ready)));
    }
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    noise_texture: Res<NoiseTextureHandle>,
) {
    // Create a single mesh for the entire grid
    let grid_world_size = GRID_SIZE as f32 * TILE_SIZE;
    let mesh = meshes.add(Rectangle::new(grid_world_size, grid_world_size));

    // Position the mesh at the center (0,0)
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(ColorMaterial::from(noise_texture.0.clone()))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        NoiseTexture,
    ));
}

fn handle_texture_ready_event(
    mut events: EventReader<TextureReadyEvent>,
    mut next_grid_state: ResMut<NextState<GridState>>,
) {
    if events.read().next().is_some() {
        next_grid_state.set(GridState::Ready);
    }
}

#[allow(dead_code)]
pub fn grid_tile_center_to_world(value: i32) -> f32 {
    grid_tile_edge_to_world(value) + TILE_SIZE / 2.0
}

pub fn grid_tile_edge_to_world(value: i32) -> f32 {
    value as f32 * TILE_SIZE - GRID_SIZE as f32 / 2.0 * TILE_SIZE
}
