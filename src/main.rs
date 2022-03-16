use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod canvas;
mod config;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1000.,
            height: 300.,
            canvas: Some("1".to_owned()),
            ..Default::default()
        })
        .add_plugins(config::MyPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..Default::default()
    });
}
