use super::*;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_debug_ui_info);
    }
}

pub fn render_debug_ui_info(mut commands: Commands) {
    commands.spawn((
        Text::new(
            // \"r\" - rebuild map
            "\"space\" - generate perlin noise
\"g\" - toggle grid",
        ),
        TextFont {
            font_size: 12.,
            ..default()
        },
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.75)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            margin: UiRect {
                top: Val::Px(8.0),
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
            },
            ..default()
        },
    ));
}
