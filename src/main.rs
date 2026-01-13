use bevy::prelude::*;

mod months;
mod navigation;
mod ui;
mod wireframes;

use months::MonthsPlugin;
use navigation::{InteractiveViaCursor, NavigationPlugin};
use ui::UiPlugin;
use wireframes::CustomWireframePlugin;

#[derive(Component)]
pub struct Earth;

#[derive(Resource)]
pub struct GlobalState {
    month: i8,
    cycle_month: bool,
    wireframe: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            month: 1,
            cycle_month: true,
            wireframe: false,
        }
    }
}

fn main() {
    let mut app = App::new();
    app.init_resource::<GlobalState>();
    app.add_plugins((
        DefaultPlugins,
        MeshPickingPlugin,
        NavigationPlugin,
        UiPlugin,
        MonthsPlugin,
    ));
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(CustomWireframePlugin);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sky_texture_handle = asset_server.load("sky.png");

    let earth_material_handle = materials.add(StandardMaterial {
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..default()
    });

    let sky_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.9),
        base_color_texture: Some(sky_texture_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        Earth,
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().uv(64, 36))),
        MeshMaterial3d(earth_material_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
        InteractiveViaCursor,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(10.0).mesh().uv(32, 18))),
        MeshMaterial3d(sky_material_handle),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 2.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}
