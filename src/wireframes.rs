use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
    color::palettes::basic::{BLUE, RED, WHITE, YELLOW},
    prelude::*,
};

use super::{GlobalState, lat_lon_to_cartesian};

pub struct CustomWireframePlugin;

impl Plugin for CustomWireframePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WireframePlugin::default());
        app.add_systems(
            Update,
            (
                draw_equator.run_if(|state: Res<GlobalState>| state.draw_equator),
                draw_earth_geographic_poles
                    .run_if(|state: Res<GlobalState>| state.draw_geographic_poles),
                draw_earth_magnetic_poles
                    .run_if(|state: Res<GlobalState>| state.draw_magnetic_poles),
                toggle_wireframe.run_if(resource_changed::<GlobalState>),
            ),
        );
    }
}

fn toggle_wireframe(mut wireframe_config: ResMut<WireframeConfig>, global_state: Res<GlobalState>) {
    wireframe_config.global = global_state.draw_wireframe;
}

fn draw_earth_geographic_poles(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), WHITE);
    gizmos.arrow(Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.), WHITE);
}

fn draw_earth_magnetic_poles(mut gizmos: Gizmos) {
    let tilt_angle = 11.5_f32.to_radians();

    let tilt_rotation = Quat::from_rotation_x(tilt_angle);

    let mag_north = tilt_rotation * Vec3::Z;
    let mag_south = tilt_rotation * Vec3::NEG_Z;

    gizmos.arrow(Vec3::ZERO, mag_north, RED);
    gizmos.arrow(Vec3::ZERO, mag_south, BLUE);
}

fn draw_equator(mut gizmos: Gizmos) {
    let segments = 64;
    for i in 0..segments {
        let lon1 = (i as f32 / segments as f32) * 360.0;
        let lon2 = ((i + 1) as f32 / segments as f32) * 360.0;

        gizmos.line(
            lat_lon_to_cartesian(0.0, lon1, 0.5),
            lat_lon_to_cartesian(0.0, lon2, 0.5),
            YELLOW,
        );
    }
}
