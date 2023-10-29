use bevy::prelude::*;
use board::generate_main_board_mesh;

mod board;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, sample_level)
        .run();
}

fn sample_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(generate_main_board_mesh()),
        material: materials.add(Color::rgb(1.0, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 4.0, 10.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
