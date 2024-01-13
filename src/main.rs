use bevy::prelude::*;
use card_game_of_the_broken_king::GamePlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cards of the broken King".to_string(),
                canvas: Some("#bevy".to_owned()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin);
    app.run();
}
