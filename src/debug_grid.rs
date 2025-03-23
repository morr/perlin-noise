use super::grid::grid_tile_edge_to_world;
use super::*;

pub struct DebugGridPlugin;
impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugGridState>()
            .add_systems(
                Update,
                render_grid.run_if(in_state(DebugGridState::Visible)),
            )
            .add_systems(Update, handle_keys);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugGridState {
    #[default]
    Hidden,
    Visible,
}

pub fn render_grid(mut gizmos: Gizmos) {
    for i in 0..GRID_SIZE {
        gizmos.line_2d(
            Vec2::new(grid_tile_edge_to_world(0), grid_tile_edge_to_world(i)),
            Vec2::new(
                grid_tile_edge_to_world(GRID_SIZE),
                grid_tile_edge_to_world(i),
            ),
            Color::srgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    for i in 0..GRID_SIZE {
        gizmos.line_2d(
            Vec2::new(grid_tile_edge_to_world(i), grid_tile_edge_to_world(0)),
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(GRID_SIZE),
            ),
            Color::srgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    gizmos.line_2d(
        Vec2::new(grid_tile_edge_to_world(0), grid_tile_edge_to_world(0)),
        Vec2::new(
            grid_tile_edge_to_world(GRID_SIZE),
            grid_tile_edge_to_world(0),
        ),
        Color::srgb(1.0, 0.0, 0.0),
    );

    gizmos.line_2d(
        Vec2::new(grid_tile_edge_to_world(0), grid_tile_edge_to_world(0)),
        Vec2::new(
            grid_tile_edge_to_world(0),
            grid_tile_edge_to_world(GRID_SIZE),
        ),
        Color::srgb(0.0, 1.0, 0.0),
    );
}

#[allow(clippy::too_many_arguments)]
fn handle_keys(
    keys: Res<ButtonInput<KeyCode>>,
    debug_grid_state: Res<State<DebugGridState>>,
    mut next_debug_grid_state: ResMut<NextState<DebugGridState>>,
) {
    if keys.just_pressed(KeyCode::KeyG) {
        match debug_grid_state.get() {
            DebugGridState::Visible => next_debug_grid_state.set(DebugGridState::Hidden),
            DebugGridState::Hidden => next_debug_grid_state.set(DebugGridState::Visible),
        };
    }
}
