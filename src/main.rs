use bevy::{app::AppExit, prelude::*};

mod plugins;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::game::GamePlugin))
        .add_systems(Update, exit_game)
        .run();
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
