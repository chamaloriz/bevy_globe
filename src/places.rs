use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use super::lat_lon_to_cartesian;

#[derive(Resource)]
pub struct MovementState {
    lat: f32,
    lon: f32,
    moving_requested: bool,
}

impl Default for MovementState {
    fn default() -> Self {
        Self {
            lat: 0.0,
            lon: 0.0,
            moving_requested: false,
        }
    }
}

#[derive(Event)]
pub struct NavigateTo {
    pub lat: f32,
    pub lon: f32,
}

pub struct PlacesPlugin;

impl Plugin for PlacesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementState>()
            .add_observer(move_camera)
            .add_systems(
                Update,
                camera_movement
                    .run_if(|state: Res<MovementState>| state.moving_requested)
                    .run_if(|mouse_button: Res<ButtonInput<MouseButton>>| {
                        !mouse_button.pressed(MouseButton::Left)
                    }),
            )
            .add_systems(EguiPrimaryContextPass, ui_system);
    }
}

fn move_camera(natigation: On<NavigateTo>, mut state: ResMut<MovementState>) {
    state.lat = natigation.lat;
    state.lon = natigation.lon;
    state.moving_requested = true;
}

fn camera_movement(
    mut state: ResMut<MovementState>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let current_distance = camera.translation.length();

    let target_position = lat_lon_to_cartesian(state.lat, state.lon, current_distance);

    let current_dir = camera.translation.normalize();
    let target_dir = target_position.normalize();

    let dot = current_dir.dot(target_dir);
    if dot > 0.9999 {
        state.moving_requested = false;
        return;
    }

    let rotation = Quat::from_rotation_arc(current_dir, target_dir);

    let speed = 3.0;
    let t = (speed * time.delta_secs()).min(1.0);
    let partial_rotation = Quat::IDENTITY.slerp(rotation, t);

    camera.rotate_around(Vec3::ZERO, partial_rotation);

    if camera.translation.distance(target_position) < 0.05 {
        state.moving_requested = false;
    }
}

fn ui_system(
    mut commands: Commands,
    mut state: ResMut<MovementState>,
    mut contexts: EguiContexts,
) -> Result {
    egui::Window::new("Places").show(contexts.ctx_mut()?, |ui| {
        ui.checkbox(&mut state.moving_requested, "moving");

        if ui.button("Belgium").clicked() {
            commands.trigger(NavigateTo {
                lat: 50.7024,
                lon: 4.7281,
            });
        }
        if ui.button("Madagascar").clicked() {
            commands.trigger(NavigateTo {
                lat: -19.6587,
                lon: 46.5245,
            });
        }
        if ui.button("Australia").clicked() {
            commands.trigger(NavigateTo {
                lat: -25.8226,
                lon: 134.1719,
            });
        }
        if ui.button("Iceland").clicked() {
            commands.trigger(NavigateTo {
                lat: 64.8881,
                lon: -18.4203,
            });
        }

        if ui.button("Chili").clicked() {
            commands.trigger(NavigateTo {
                lat: -26.2958,
                lon: -70.0307,
            });
        }

        if ui.button("Cansas").clicked() {
            commands.trigger(NavigateTo {
                lat: 38.4486,
                lon: -98.4658,
            });
        }
    });
    Ok(())
}
