use bevy::input::mouse::MouseWheel;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseScrollUnit},
    prelude::*,
};
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (scroll_events, rotate_on_drag));
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
        let new_distance = (current_distance - scroll_amount).clamp(1.0, 4.0);

        if current_distance > 0.0 {
            camera.translation = camera.translation.normalize() * new_distance;
        }
    }
}

pub fn rotate_on_drag(
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }

    let rotation_sensitivity = 0.005;

    let rotation_y = Quat::from_axis_angle(
        camera.local_y().normalize().into(),
        -mouse_motion.delta.x * rotation_sensitivity,
    );
    let rotation_x = Quat::from_axis_angle(
        camera.local_x().normalize().into(),
        -mouse_motion.delta.y * rotation_sensitivity,
    );

    let combined_rotation = rotation_y * rotation_x;
    camera.rotate_around(Vec3::ZERO, combined_rotation);
}
