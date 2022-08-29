use bevy::prelude::*;

mod color_mixer;
mod main_menu;
mod game_ui;
mod game;
mod widgets;

pub const LAUNCHER_TITLE: &str = "Guess Hue?";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(main_menu::MainMenuPlugin)
    .add_plugin(game::GamePlugin)
    .add_plugin(game_ui::GameUiPlugin)
    .add_plugin(widgets::GameButtonPlugin)
    .add_plugin(widgets::GameIndicatorPlugin)
    .add_state(AppState::MainMenu)
    .add_startup_system(setup);
    app
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
