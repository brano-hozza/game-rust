pub mod events;
mod systems;

pub mod game;
pub mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .run();
}
