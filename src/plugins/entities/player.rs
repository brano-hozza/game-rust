use bevy::{prelude::*, window::PrimaryWindow};

use crate::plugins::{events::GameOver, resources::score::Score};

use super::{
    enemy::{Enemy, ENEMY_SIZE},
    star::{Star, STAR_SIZE},
};

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

pub fn player_hit_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over_writer: EventWriter<GameOver>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(player_transform.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let enemy_radius = ENEMY_SIZE / 2.;
            if distance < player_radius + enemy_radius {
                commands.entity(player_entity).despawn();
                game_over_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (start_entity, star_transform) in star_query.iter() {
            let distance = star_transform
                .translation
                .distance(player_transform.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let star_radius = STAR_SIZE / 2.;
            if distance < player_radius + star_radius {
                commands.entity(start_entity).despawn();
                score.value += 1;
            }
        }
    }
}
