use bevy::prelude::*;

mod plugins;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::game::GamePlugin))
        .run();
}
