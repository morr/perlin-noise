use super::*;
use super::grid::grid_tile_edge_to_world;

pub struct DebugGridPlugin;
impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_grid
        );
    }
}

pub fn render_grid(mut gizmos: Gizmos) {
    for i in 0..GRID_SIZE {
        gizmos.line_2d(
            Vec2::new(
                grid_tile_edge_to_world(0),
                grid_tile_edge_to_world(i),
            ),
            Vec2::new(
                grid_tile_edge_to_world(GRID_SIZE),
                grid_tile_edge_to_world(i),
            ),
            Color::srgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    for i in 0..GRID_SIZE {
        gizmos.line_2d(
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(0),
            ),
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(GRID_SIZE),
            ),
            Color::srgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    gizmos.arrow_2d(
        Vec2::new(0.0, 0.0),
        Vec2::new(GRID_SIZE as f32 * TILE_SIZE, 0.0),
        Color::srgb(1.0, 0.0, 0.0),
    );

    gizmos.arrow_2d(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, GRID_SIZE as f32 * TILE_SIZE),
        Color::srgb(0.0, 1.0, 0.0),
    );
}
