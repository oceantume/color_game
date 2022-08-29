use bevy::prelude::*;

use crate::{
    game::GameOptions,
    widgets::{spawn_game_button, GameButton},
    AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu).with_system(teardown),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(play)
                .with_system(toggle_mute)
                .with_system(update_mute_button),
        );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MenuItem;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct MuteButton;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let play_button = spawn_game_button(
        &mut commands,
        &asset_server,
        GameButton {
            text: "Play".into(),
        },
    );
    commands.entity(play_button).insert(PlayButton);

    let mute_button = spawn_game_button(
        &mut commands,
        &asset_server,
        GameButton {
            text: "Mute sound".into(),
        },
    );
    commands.entity(mute_button).insert(MuteButton);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Percent(15.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MainMenu)
        .with_children(|main_container| {
            main_container
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|title_container| {
                    title_container.spawn_bundle(
                        TextBundle::from_section(
                            "Guess Hue?",
                            TextStyle {
                                font: asset_server.load("edosz.ttf"),
                                font_size: 45.0,
                                color: Color::GREEN,
                            },
                        )
                        .with_style(Style { ..default() }),
                    );
                    title_container.spawn_bundle(
                        TextBundle::from_section(
                            "Made for Bevy Jam 2",
                            TextStyle {
                                font: asset_server.load("edosz.ttf"),
                                font_size: 22.0,
                                color: Color::YELLOW_GREEN,
                            },
                        )
                        .with_style(Style { ..default() }),
                    );
                });

            main_container
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        margin: UiRect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        size: Size::new(Val::Auto, Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        //justify_content: JustifyContent::SpaceAround,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .add_child(play_button)
                .add_child(mute_button);
        });
}

fn teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn play(
    mut app_state: ResMut<State<AppState>>,
    query: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
) {
    let clicked = query.iter().any(|interaction| match interaction {
        Interaction::Clicked => true,
        _ => false,
    });

    if clicked {
        app_state.set(AppState::InGame).unwrap();
    }
}

fn toggle_mute(
    mut options: ResMut<GameOptions>,
    query: Query<&Interaction, (With<MuteButton>, Changed<Interaction>)>,
) {
    let clicked = query.iter().any(|interaction| match interaction {
        Interaction::Clicked => true,
        _ => false,
    });

    if clicked {
        options.mute = !options.mute;
    }
}

fn update_mute_button(
    options: Res<GameOptions>,
    mut query: Query<&mut GameButton, With<MuteButton>>,
) {
    query.iter_mut().for_each(|mut btn| {
        let text = match options.mute {
            true => "Unmute sound",
            false => "Mute sound",
        };
        
        if btn.text != text {
            btn.text = text.into();
        }
    })
}
