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
    cycle_duration: u64,
    draw_wireframe: bool,
    draw_geographic_poles: bool,
    draw_magnetic_poles: bool,
    draw_equator: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            month: 1,
            cycle_duration: 1000,
            cycle_month: true,
            draw_wireframe: false,
            draw_geographic_poles: false,
            draw_magnetic_poles: false,
            draw_equator: false,
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

fn lat_lon_to_cartesian(lat: f32, lon: f32, radius: f32) -> Vec3 {
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();

    Vec3::new(
        -radius * lat_rad.cos() * lon_rad.cos(),
        -radius * lat_rad.cos() * lon_rad.sin(),
        radius * lat_rad.sin(),
    )
}
