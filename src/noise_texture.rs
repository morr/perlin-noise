// noise.rs
use super::*;
use crate::grid::NoiseTextureHandle;
use crate::grid::TextureReadyEvent;
use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use noise::{NoiseFn, Perlin};
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

#[derive(Event)]
pub struct GenerateNoiseEvent;

pub struct NoiseTexturePlugin;

impl Plugin for NoiseTexturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseSettings>()
            .add_event::<GenerateNoiseEvent>()
            .add_systems(Startup, setup_noise_texture)
            .add_systems(Startup, initial_noise_generation.after(setup_noise_texture))
            .add_systems(Update, keyboard_input_system)
            .add_systems(Update, generate_noise);
    }
}

fn setup_noise_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let texture_size = UVec2::new(
        (GRID_SIZE as f32 * TILE_SIZE) as u32,
        (GRID_SIZE as f32 * TILE_SIZE) as u32,
    );

    let pixel_count = texture_size.x * texture_size.y;
    let texture_data = vec![0u8; (pixel_count * 4) as usize];

    let texture = Image::new(
        Extent3d {
            width: texture_size.x,
            height: texture_size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture_data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let texture_handle = images.add(texture);
    commands.insert_resource(NoiseTextureHandle(texture_handle));
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

fn initial_noise_generation(
    noise_settings: Res<NoiseSettings>,
    mut images: ResMut<Assets<Image>>,
    noise_texture: Res<NoiseTextureHandle>,
    mut texture_ready_events: EventWriter<TextureReadyEvent>,
) {
    // Generate the initial noise texture
    generate_noise_texture(&noise_settings, &mut images, &noise_texture);

    // Signal that the texture is ready
    texture_ready_events.send(TextureReadyEvent);
}

fn generate_noise(
    mut events: EventReader<GenerateNoiseEvent>,
    noise_settings: Res<NoiseSettings>,
    mut images: ResMut<Assets<Image>>,
    noise_texture: Res<NoiseTextureHandle>,
) {
    // Only process one event per frame to avoid multiple rapid updates
    if events.read().next().is_some() {
        generate_noise_texture(&noise_settings, &mut images, &noise_texture);
    }
}

// Fixed function that properly updates the texture with higher resolution
fn generate_noise_texture(
    noise_settings: &NoiseSettings,
    images: &mut Assets<Image>,
    noise_texture: &NoiseTextureHandle,
) {
    // Get the Perlin noise generator with the current seed
    let perlin = Perlin::new(noise_settings.seed);

    // Get the image from the handle
    let image = if let Some(image) = images.get_mut(&noise_texture.0) {
        image
    } else {
        return;
    };

    let width = image.width() as usize;
    let height = image.height() as usize;

    // Generate new image data directly into the texture
    let mut texture_data = vec![0u8; width * height * 4];

    // Generate noise values for each pixel in the texture
    for y in 0..height {
        for x in 0..width {
            // Scale coordinates to noise space - adjust frequency to match the original appearance
            // We need to adjust the frequency since we're sampling more points now
            let scaling_factor = 1.0 / TILE_SIZE as f64;
            let nx = x as f64 * noise_settings.frequency * scaling_factor;
            let ny = y as f64 * noise_settings.frequency * scaling_factor;

            let mut noise_value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;

            // Generate octaves of noise
            for _ in 0..noise_settings.octaves {
                noise_value += perlin.get([nx * frequency, ny * frequency, 0.0]) * amplitude;
                amplitude *= noise_settings.persistence;
                frequency *= noise_settings.lacunarity;
            }

            // Normalize to 0.0 - 1.0
            noise_value = (noise_value + 1.0) / 2.0;

            // Convert to 0-255 for RGBA
            let value = (noise_value * 255.0) as u8;

            // Calculate pixel index (y * width + x) * 4 for RGBA format
            let idx = (y * width + x) * 4;

            // Set RGBA values (grayscale with full opacity)
            texture_data[idx] = value; // R
            texture_data[idx + 1] = value; // G
            texture_data[idx + 2] = value; // B
            texture_data[idx + 3] = 255; // A (full opacity)
        }
    }

    image.data = texture_data;
}
