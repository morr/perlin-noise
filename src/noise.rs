use super::*;
use ::noise::{NoiseFn, Perlin};
use rand::random;

#[derive(Resource)]
pub struct NoiseSettings {
    pub seed: u32,
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            seed: random(),
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
        }
    }
}

#[derive(Component)]
pub struct NoiseControlled {
    pub position: (i32, i32),
}

#[derive(Event)]
pub struct GenerateNoiseEvent;

pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseSettings>()
            .add_event::<GenerateNoiseEvent>()
            .add_systems(Update, keyboard_input_system)
            .add_systems(Update, generate_noise);
    }
}

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut noise_settings: ResMut<NoiseSettings>,
    mut generate_noise_events: EventWriter<GenerateNoiseEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        noise_settings.seed = random();
        generate_noise_events.send(GenerateNoiseEvent);
    }
}

fn generate_noise(
    mut events: EventReader<GenerateNoiseEvent>,
    noise_settings: Res<NoiseSettings>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tiles: Query<(&NoiseControlled, &MeshMaterial2d<ColorMaterial>)>,
) {
    if events.read().next().is_none() {
        return;
    }

    let perlin = Perlin::new(noise_settings.seed);

    for (controlled, mesh_material) in tiles.iter_mut() {
        let (x, y) = controlled.position;
        let nx = x as f64 * noise_settings.frequency;
        let ny = y as f64 * noise_settings.frequency;

        let mut noise_value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 1.0;

        for _ in 0..noise_settings.octaves {
            noise_value += perlin.get([nx * frequency, ny * frequency, 0.0]) * amplitude;
            amplitude *= noise_settings.persistence;
            frequency *= noise_settings.lacunarity;
        }

        // Normalize to 0.0 - 1.0
        noise_value = (noise_value + 1.0) / 2.0;

        // Update material color based on noise value
        let color = Color::srgb(noise_value as f32, noise_value as f32, noise_value as f32);
        if let Some(material) = materials.get_mut(&mesh_material.0) {
            *material = ColorMaterial::from(color);
        }
    }
}
