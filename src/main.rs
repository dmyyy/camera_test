use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

// pointy end of model faces -y axis (in blender)
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SceneBundle {
        scene: asset_server.load("test_shape.glb#Scene0"),
        transform: Transform::from_xyz(0., 0., 0.),
        // .looking_at(Vec3::new(0., 0., 100.), Vec3::Y),
        ..default()
    },));

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0., 5., 10.).looking_at(Vec3::new(0., 0., -5.), Vec3::Y),
        ..default()
    },));

    commands.insert_resource(AmbientLight {
        color: Color::BEIGE,
        brightness: 1.,
    });
}
