use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::{Earth, GlobalState};

#[derive(Resource, Default)]
pub struct EarthTextures {
    pub textures: Vec<Handle<Image>>,
}

pub struct MonthsPlugin;

impl Plugin for MonthsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EarthTextures>()
            .add_systems(Startup, preload_earth_textures)
            .add_systems(
                Update,
                (
                    update_earth_texture,
                    cycle_through_months.run_if(on_timer(Duration::from_millis(100))),
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

fn update_earth_texture(
    global_state: Res<GlobalState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_material: Single<&MeshMaterial3d<StandardMaterial>, With<Earth>>,
    earth_textures: Res<EarthTextures>,
) {
    if !global_state.is_changed() {
        return;
    }

    if let Some(material) = materials.get_mut(mesh_material.0.id()) {
        let texture_index = (global_state.month - 1) as usize;
        if let Some(texture_handle) = earth_textures.textures.get(texture_index) {
            material.base_color_texture = Some(texture_handle.clone());
        }
    }
}

fn cycle_through_months(mut global_state: ResMut<GlobalState>) {
    global_state.month = (global_state.month % 12) + 1;
}
