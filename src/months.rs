use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use super::Earth;

#[derive(Resource)]
pub struct MonthState {
    month: i8,
    cycle_month: bool,
    cycle_duration: u64,
}

impl Default for MonthState {
    fn default() -> Self {
        Self {
            month: 1,
            cycle_duration: 1000,
            cycle_month: true
        }
    }
}

#[derive(Resource, Default)]
pub struct EarthTextures {
    pub textures: Vec<Handle<Image>>,
}

pub struct MonthsPlugin;

impl Plugin for MonthsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EarthTextures>()
            .init_resource::<MonthState>()
            .add_systems(Startup, preload_earth_textures)
            .add_systems(EguiPrimaryContextPass, ui_system)
            .add_systems(
                Update,
                (
                    change_earth_texture.run_if(resource_changed::<MonthState>),
                    cycle_through_months.run_if(
                        |time: Res<Time>, state: Res<MonthState>, mut timer: Local<Timer>| {
                            timer.set_mode(TimerMode::Repeating);
                            timer.set_duration(Duration::from_millis(state.cycle_duration));
                            state.cycle_month && timer.tick(time.delta()).just_finished()
                        },
                    ),
                ),
            );
    }
}

fn preload_earth_textures(
    mut earth_textures: ResMut<EarthTextures>,
    asset_server: Res<AssetServer>,
) {
    earth_textures.textures = (1..=12)
        .map(|month| {
            let texture_path = format!("globe/earth_{:02}.png", month);
            asset_server.load(&texture_path)
        })
        .collect();
}

fn change_earth_texture(
    state: Res<MonthState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_material: Single<&MeshMaterial3d<StandardMaterial>, With<Earth>>,
    earth_textures: Res<EarthTextures>,
) {
    if let Some(material) = materials.get_mut(mesh_material.0.id()) {
        let texture_index = (state.month - 1) as usize;
        if let Some(texture_handle) = earth_textures.textures.get(texture_index) {
            material.base_color_texture = Some(texture_handle.clone());
        }
    }
}

fn cycle_through_months(mut state: ResMut<MonthState>) {
    state.month = (state.month % 12) + 1;
}

fn ui_system(
    mut contexts: EguiContexts, 
    mut state: ResMut<MonthState>
) -> Result {
    egui::Window::new("Months").show(contexts.ctx_mut()?, |ui| {
        ui.add(egui::Slider::new(&mut state.month, 1..=12).text("month"));
        ui.add(
            egui::Slider::new(&mut state.cycle_duration, 1..=1000)
                .text("cycle duration")
                .suffix("ms"),
        );
        ui.checkbox(&mut state.cycle_month, "cycle months");
    });
    Ok(())
}