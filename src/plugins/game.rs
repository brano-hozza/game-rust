use bevy::{prelude::*, window::PrimaryWindow};

use super::entities::{enemy, player, star};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_camera,
                player::spawn_player,
                enemy::spawn_enemies,
                star::spawn_stars,
            ),
        )
        .add_systems(
            First,
            (player::move_player, player::confine_player_movement),
        )
        .add_systems(
            First,
            (
                enemy::move_enemy,
                enemy::update_enemy_direction,
                enemy::confine_enemy_movement,
            ),
        )
        .add_systems(Update, player_hit_enemy);
    }
}

fn spawn_camera(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
        ..default()
    });
}

fn player_hit_enemy(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<player::Player>>,
    enemy_query: Query<&Transform, With<enemy::Enemy>>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = enemy_transform
                .translation
                .distance(player_transform.translation);
            let player_radius = player::PLAYER_SIZE / 2.;
            let enemy_radius = enemy::ENEMY_SIZE / 2.;
            if distance < player_radius + enemy_radius {
                commands.entity(player_entity).despawn();
            }
        }
    }
}
