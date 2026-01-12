use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{color::palettes::basic::WHITE, prelude::*};

pub struct CustomWireframePlugin;

impl Plugin for CustomWireframePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WireframePlugin::default());
        app.add_systems(Update, toggle_wireframe);
        app.add_systems(Update, draw_earth_geographic_poles);
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn draw_earth_geographic_poles(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), WHITE);
    gizmos.arrow(Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.), WHITE);
}
