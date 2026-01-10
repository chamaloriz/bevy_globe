use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

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
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {}
            MouseScrollUnit::Pixel => {
                camera.translation.z = (camera.translation.z - ev.y / 300.0).clamp(0.90, 3.0);
            }
        }
    }
}

pub fn rotate_on_drag(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
