use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, my_startup);
    }
}

fn my_startup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1., 2., -3.),
        point_light: PointLight {
            color: Color::rgb(1., 0., 1.),
            ..default()
        },
        ..default()
    });
}
