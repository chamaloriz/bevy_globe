use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{color::palettes::basic::WHITE, prelude::*};

use super::GlobalState;

pub struct CustomWireframePlugin;

impl Plugin for CustomWireframePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WireframePlugin::default());
        app.add_systems(
            Update,
            (
                draw_earth_geographic_poles,
                toggle_wireframe.run_if(resource_changed::<GlobalState>),
            ),
        );
    }
}

fn toggle_wireframe(mut wireframe_config: ResMut<WireframeConfig>, global_state: Res<GlobalState>) {
    wireframe_config.global = global_state.wireframe;
}

fn draw_earth_geographic_poles(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), WHITE);
    gizmos.arrow(Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.), WHITE);
}
