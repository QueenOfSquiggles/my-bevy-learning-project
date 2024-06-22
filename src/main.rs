use bevy::prelude::*;

const RADIANS_PER_SECOND: f32 = 0.1;

mod game_plugin;
mod hud;
mod ui_builder;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(game_plugin::GamePlugin)
        .add_plugins(hud::HUDPlugin)
        // .add_systems(Startup, setup)
        // .add_systems(Update, rotate_camera)
        // .add_systems(Update, animate_coins)
        .run();
}

#[derive(Component)]
struct CoinLabel;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.4, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5., 5.)),
        material: materials.add(Color::rgb(0., 0.2, 0.)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::rgb(0., 1., 1.)),
        transform: Transform::from_xyz(2., 0.5, 0.),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });
    commands
        .spawn(SceneBundle {
            scene: assets.load("models/coin.glb#Scene0"),
            ..default()
        })
        .insert(CoinLabel {});
}

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();
    transform.rotate_around(
        Vec3::ZERO,
        Quat::from_rotation_y(time.delta_seconds() * RADIANS_PER_SECOND),
    );
}

fn animate_coins(mut query: Query<&mut Transform, With<CoinLabel>>, time: Res<Time>) {
    for mut coin_trans in &mut query {
        coin_trans.rotate_y(time.delta_seconds() * 4.);
        coin_trans.translation.y = (time.elapsed_seconds() * 2.).sin() + 1.;
    }
}
