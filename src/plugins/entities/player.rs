use bevy::{prelude::*, window::PrimaryWindow};

const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_single().unwrap();

    commands.spawn((
        Player {},
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: asset_server.load("sprites/ball_blue_small.png"),
            ..default()
        },
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.;
        }
        if direction.length() > 0. {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = windows.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.;

        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}
