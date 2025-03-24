use super::*;
use crate::grid::NoiseTexture;
use crate::grid::NoiseTextureHandle;
use crate::grid::TextureReadyEvent;
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use noise::{NoiseFn, Perlin};
use rand::random;

#[derive(Resource)]
pub struct NoiseSettings {
    pub seed: u32,
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            // seed: random(),
            seed: 1655470700,
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

#[derive(Resource)]
pub struct NoiseTextureBuffers {
    pub active: Handle<Image>,
    pub inactive: Handle<Image>,
    pub is_first_active: bool,
}

#[derive(Event)]
pub struct GenerateNoiseEvent;

pub struct NoiseTexturePlugin;

impl Plugin for NoiseTexturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseSettings>()
            .add_event::<GenerateNoiseEvent>()
            .add_systems(Update, keyboard_input_system)
            .add_systems(Startup, setup_noise_texture)
            .add_systems(Startup, initial_noise_generation.after(setup_noise_texture))
            .add_systems(Update, generate_noise_and_update);
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
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            noise_settings.offset_x += 1;
            generate_noise_events.send(GenerateNoiseEvent);
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            noise_settings.offset_x -= 1;
            generate_noise_events.send(GenerateNoiseEvent);
        }
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            noise_settings.offset_y += 1;
            generate_noise_events.send(GenerateNoiseEvent);
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            noise_settings.offset_y -= 1;
            generate_noise_events.send(GenerateNoiseEvent);
        }
    }
}

fn setup_noise_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create texture dimensions
    let texture_size = UVec2::new(
        (GRID_SIZE as f32 * TILE_SIZE) as u32,
        (GRID_SIZE as f32 * TILE_SIZE) as u32,
    );

    // Create two identical textures with proper settings
    let texture1 = create_empty_texture(texture_size);
    let texture2 = create_empty_texture(texture_size);

    let handle1 = images.add(texture1);
    let handle2 = images.add(texture2);

    // Store both handles and track which is active
    commands.insert_resource(NoiseTextureBuffers {
        active: handle1.clone(),
        inactive: handle2,
        is_first_active: true,
    });

    // For compatibility with existing code
    commands.insert_resource(NoiseTextureHandle(handle1));
}

// Helper function to create a properly configured texture
fn create_empty_texture(size: UVec2) -> Image {
    // Calculate total pixels for RGBA format (4 bytes per pixel)
    let pixel_count = size.x * size.y;
    let texture_data = vec![0u8; (pixel_count * 4) as usize];

    // Create the image with proper sampling settings
    let mut texture = Image::new(
        Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture_data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    // Set nearest-neighbor filtering with proper address modes
    texture.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Nearest,
        mipmap_filter: ImageFilterMode::Nearest,
        address_mode_u: ImageAddressMode::ClampToEdge,
        address_mode_v: ImageAddressMode::ClampToEdge,
        address_mode_w: ImageAddressMode::ClampToEdge,
        ..default()
    });

    texture
}

pub fn initial_noise_generation(
    noise_settings: Res<NoiseSettings>,
    mut images: ResMut<Assets<Image>>,
    buffers: Res<NoiseTextureBuffers>,
    mut texture_ready_events: EventWriter<TextureReadyEvent>,
) {
    // Generate the initial noise texture in both buffers for consistency
    generate_noise_texture(&noise_settings, &mut images, &buffers.active);
    generate_noise_texture(&noise_settings, &mut images, &buffers.inactive);

    // Signal that the texture is ready
    texture_ready_events.send(TextureReadyEvent);
}

// Combined system that handles both generating noise and updating materials
fn generate_noise_and_update(
    mut events: EventReader<GenerateNoiseEvent>,
    noise_settings: Res<NoiseSettings>,
    mut images: ResMut<Assets<Image>>,
    mut buffers: ResMut<NoiseTextureBuffers>,
    mut noise_texture: ResMut<NoiseTextureHandle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut MeshMaterial2d<ColorMaterial>, With<NoiseTexture>>,
) {
    if events.read().next().is_some() {
        generate_noise_texture(&noise_settings, &mut images, &buffers.inactive);

        // Manual swap using a temporary variable to avoid multiple mutable borrows
        let temp = buffers.active.clone();
        buffers.active = buffers.inactive.clone();
        buffers.inactive = temp;

        // Toggle the active buffer indicator
        buffers.is_first_active = !buffers.is_first_active;

        // Update the main handle
        noise_texture.0 = buffers.active.clone();

        // Create a new material with the updated texture
        let new_material = materials.add(ColorMaterial::from(buffers.active.clone()));

        // Update all NoiseTexture entities to use the new material
        for mut mesh_material in query.iter_mut() {
            *mesh_material = MeshMaterial2d(new_material.clone());
        }
    }
}

// Function for generating the noise texture (remains unchanged)
fn generate_noise_texture(
    noise_settings: &NoiseSettings,
    images: &mut Assets<Image>,
    texture_handle: &Handle<Image>,
) {
    // Get the Perlin noise generator with the current seed
    let perlin = Perlin::new(noise_settings.seed);

    // Get the image from the handle
    let image = if let Some(image) = images.get_mut(texture_handle) {
        image
    } else {
        return;
    };

    // println!("{:?}x{:?}", image.width(), image.height());
    let width = image.width() as usize;
    let height = image.height() as usize;

    // Generate new image data directly into the texture
    let mut texture_data = vec![0u8; width * height * 4];

    // Generate noise values for each pixel in the texture
    for y in 0..height {
        for x in 0..width {
            let offseted_x = x as f64 + noise_settings.offset_x as f64 * TILE_SIZE as f64 * 30.0;
            let offseted_y = y as f64 + noise_settings.offset_y as f64 * TILE_SIZE as f64 * 30.0;

            // Convert pixel coordinates to normalized grid coordinates (0.0 to 1.0)
            let grid_x = offseted_x / width as f64;
            let grid_y = offseted_y / height as f64;

            // Scale to the grid range and apply frequency
            let nx = grid_x * GRID_SIZE as f64 * noise_settings.frequency * TILE_SIZE as f64;
            let ny = grid_y * GRID_SIZE as f64 * noise_settings.frequency * TILE_SIZE as f64;

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

    // Update the image data
    image.data = texture_data;
}
