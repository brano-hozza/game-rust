use bevy::{prelude::*, window::PrimaryWindow};

use super::entities::{enemy, player, star};
use super::resources::{score, timers};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<score::Score>()
            .init_resource::<timers::StarSpawnTimer>()
            .init_resource::<timers::EnemySpawnTimer>()
            .add_systems(
                Startup,
                (
                    spawn_camera,
                    player::spawn_player,
                    enemy::spawn_enemies,
                    star::spawn_stars,
                ),
            )
            .add_systems(PreUpdate, timers::update_timers)
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
            .add_systems(
                Update,
                (
                    player::player_hit_enemy,
                    player::player_hit_star,
                    score::update_score,
                ),
            )
            .add_systems(
                Update,
                (star::spawn_start_over_time, enemy::spawn_enemy_over_time),
            );
    }
}

fn spawn_camera(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
        ..default()
    });
}
