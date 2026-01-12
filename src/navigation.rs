use bevy::input::mouse::MouseWheel;
use bevy::{input::mouse::MouseScrollUnit, prelude::*};
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, scroll_events);
    }
}

fn scroll_events(
    mut evr_scroll: MessageReader<MouseWheel>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    for ev in evr_scroll.read() {
        let scroll_amount = match ev.unit {
            MouseScrollUnit::Line => ev.y * 0.1,
            MouseScrollUnit::Pixel => ev.y / 300.0,
        };

        let current_distance = camera.translation.length();
        let new_distance = (current_distance - scroll_amount).clamp(0.90, 3.0);

        if current_distance > 0.0 {
            camera.translation = camera.translation.normalize() * new_distance;
        }
    }
}

pub fn rotate_on_drag(drag: On<Pointer<Drag>>, mut camera: Single<&mut Transform, With<Camera3d>>) {
    let rotation_sensitivity = 0.005;
    let camera_right = camera.right();

    let rotation_y = Quat::from_axis_angle(Vec3::Y, -drag.delta.x * rotation_sensitivity);
    let rotation_x =
        Quat::from_axis_angle(camera_right.into(), -drag.delta.y * rotation_sensitivity);

    let combined_rotation = rotation_y * rotation_x;
    camera.rotate_around(Vec3::ZERO, combined_rotation);
}
