// noise_settings_ui.rs
use super::*;
use crate::noise_texture::{GenerateNoiseEvent, NoiseSettings};
use bevy_egui::{egui, EguiContext};
use rand::random;

pub struct NoiseSettingsUiPlugin;

impl Plugin for NoiseSettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, noise_settings_ui_system);
    }
}

fn noise_settings_ui_system(
    mut contexts: Query<&mut EguiContext>,
    mut noise_settings: ResMut<NoiseSettings>,
    mut generate_noise_events: EventWriter<GenerateNoiseEvent>,
) {
    if let Ok(mut context) = contexts.get_single_mut() {
        egui::Window::new("Noise Settings").show(context.get_mut(), |ui| {
            let mut settings_changed = false;

            ui.horizontal(|ui| {
                ui.label("Seed:");
                settings_changed |= ui
                    .add(egui::DragValue::new(&mut noise_settings.seed).speed(1.0))
                    .changed();
                if ui.button("Randomize").clicked() {
                    noise_settings.seed = random();
                    settings_changed = true;
                }
            });

            settings_changed |= ui
                .add(
                    egui::Slider::new(&mut noise_settings.frequency, 0.001..=0.1)
                        .text("Frequency")
                        .logarithmic(true),
                )
                .changed();

            let mut octaves = noise_settings.octaves as i32;
            if ui
                .add(egui::Slider::new(&mut octaves, 1..=8).text("Octaves"))
                .changed()
            {
                noise_settings.octaves = octaves as usize;
                settings_changed = true;
            }

            settings_changed |= ui
                .add(
                    egui::Slider::new(&mut noise_settings.lacunarity, 1.0..=4.0).text("Lacunarity"),
                )
                .changed();

            settings_changed |= ui
                .add(
                    egui::Slider::new(&mut noise_settings.persistence, 0.0..=1.0)
                        .text("Persistence"),
                )
                .changed();

            if settings_changed {
                generate_noise_events.send(GenerateNoiseEvent);
            }
        });
    }
}
