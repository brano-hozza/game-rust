use bevy::{prelude::*, window::PrimaryWindow};

const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 32.;
#[derive(Component)]
pub struct Star;

pub fn spawn_stars(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_single().unwrap();
    let star_width = STAR_SIZE / 2.;
    let star_height = STAR_SIZE / 2.;

    for _ in 0..NUMBER_OF_STARS {
        let random_x = rand::random::<f32>() * (window.width() - STAR_SIZE) + star_width;
        let random_y = rand::random::<f32>() * (window.height() - STAR_SIZE) + star_height;
        commands.spawn((
            Star {},
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
        ));
    }
}
