use bevy::prelude::*;

mod navigation;
mod wireframes;

use navigation::{NavigationPlugin, rotate_on_drag};
use wireframes::CustomWireframePlugin;

#[derive(Component)]
struct Globe;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin, NavigationPlugin));
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
    let texture_handle = asset_server.load("globe_default.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands
        .spawn((
            Mesh3d(meshes.add(Sphere::default().mesh().uv(32, 18))),
            MeshMaterial3d(material_handle),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Globe,
        ))
        .observe(rotate_on_drag);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 2.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}
