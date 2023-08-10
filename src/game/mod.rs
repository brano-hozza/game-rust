use bevy::prelude::*;

use crate::{
    events::GameOver,
    systems::{exit_game, handle_game_over},
};

use self::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin};

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_event::<GameOver>()
            .add_systems(Update, exit_game)
            .add_systems(Update, handle_game_over);
    }
}
