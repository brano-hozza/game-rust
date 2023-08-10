use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod star;
mod systems;

use crate::{
    events::GameOver,
    systems::{exit_game, handle_game_over},
    AppState,
};

use self::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin};
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_event::<GameOver>()
            .add_state::<SimulationState>()
            .add_systems(Update, exit_game)
            .add_systems(Update, handle_game_over)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
