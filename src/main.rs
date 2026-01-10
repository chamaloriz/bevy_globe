use bevy::prelude::*;

#[derive(Component)]
struct Globe;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin));
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

fn rotate_on_drag(drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    let mut transform = transforms.get_mut(drag.entity).unwrap();
    transform.rotate_y(drag.delta.x * 0.02);
    transform.rotate_x(drag.delta.y * 0.02);
}
