use super::grid::grid_tile_edge_to_world;
use super::*;

pub struct DebugGridPlugin;
impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_grid);
    }
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

