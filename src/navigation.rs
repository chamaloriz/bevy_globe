use bevy::input::mouse::MouseWheel;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseScrollUnit},
    prelude::*,
};

#[derive(Component)]
pub struct InteractiveViaCursor;

#[derive(Component)]
pub struct InteractivityReady;

#[derive(Resource, Default)]
pub struct CursorOverEntity(pub bool);

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorOverEntity>();
        app.add_systems(Update, (setup_interactivity, scroll_events, rotate_on_drag));
    }
}

fn setup_interactivity(
    mut commands: Commands,
    entities: Query<Entity, (With<InteractiveViaCursor>, Without<InteractivityReady>)>,
) {
    for entity in entities {
        commands
            .entity(entity)
            .insert(InteractivityReady)
            .observe(
                |_: On<Pointer<Over>>, mut state: ResMut<CursorOverEntity>| {
                    state.0 = true;
                },
            )
            .observe(|_: On<Pointer<Out>>, mut state: ResMut<CursorOverEntity>| {
                state.0 = false;
            });
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
    cursor_over_entity: Res<CursorOverEntity>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if !cursor_over_entity.0 {
        return;
    }

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
