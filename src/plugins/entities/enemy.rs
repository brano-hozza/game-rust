use bevy::{prelude::*, window::PrimaryWindow};

use crate::plugins::resources::timers::EnemySpawnTimer;

const NUMBER_OF_ENEMIES: usize = 10;
const ENEMY_SPEED: f32 = 100.;
pub const ENEMY_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub fn spawn_enemies(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            Enemy {
                direction: Vec2 {
                    x: rand::random::<f32>(),
                    y: rand::random::<f32>(),
                }
                .normalize(),
            },
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_small.png"),
                ..default()
            },
        ));
    }
}

pub fn move_enemy(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
pub fn update_enemy_direction(
    mut query: Query<(&Transform, &mut Enemy)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.;

    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in query.iter_mut() {
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
        }
    }
}
pub fn confine_enemy_movement(
    mut query: Query<&mut Transform, With<Enemy>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.;

    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min {
            transform.translation.x = x_min;
        } else if translation.x > x_max {
            transform.translation.x = x_max;
        }

        if translation.y < y_min {
            transform.translation.y = y_min;
        } else if translation.y > y_max {
            transform.translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = windows.get_single().unwrap();
        let enemy_width = ENEMY_SIZE / 2.;
        let enemy_height = ENEMY_SIZE / 2.;
        let random_x = rand::random::<f32>() * (window.width() - ENEMY_SIZE) + enemy_width;
        let random_y = rand::random::<f32>() * (window.height() - ENEMY_SIZE) + enemy_height;
        commands.spawn((
            Enemy {
                direction: Vec2 {
                    x: rand::random::<f32>(),
                    y: rand::random::<f32>(),
                }
                .normalize(),
            },
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.),
                texture: asset_server.load("sprites/ball_red_small.png"),
                ..default()
            },
        ));
    }
}
