use bevy::prelude::*;

use crate::widgets::{
    spawn_game_button, spawn_game_indicator, GameButton, GameButtonLabel,
    GameIndicator, GameIndicatorLabel,
};
use crate::{color_mixer::mix_colors, AppState};

use crate::game::{
    ColorSelector, GameState, LevelState, PlayerColorsChanged,
    StartLevelEvent, OBJECTIVES_DATA, PALETTE_DATA, AlertStartedEvent, AlertEndedEvent, GameOptions,
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(teardown),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .label("ui_update")
                .before(GameButtonLabel)
                .before(GameIndicatorLabel)
                .with_system(handle_exit_clicked)
                .with_system(update_player_color)
                .with_system(update_objective_color)
                .with_system(update_complexity_indicator)
                .with_system(update_selection_indicator)
                .with_system(update_level_indicator)
                .with_system(update_lives_indicator)
                .with_system(handle_color_clicked)
                .with_system(show_alert)
                .with_system(hide_alert),
        );
    }
}

#[derive(Component)]
struct GameUIRoot;

#[derive(Component)]
struct MenuButton;

#[derive(Component)]
struct ObjectiveColor;

#[derive(Component)]
struct PlayerColor;

#[derive(Component)]
struct ComplexityIndicator;

#[derive(Component)]
struct SelectionIndicator;

#[derive(Component)]
struct LevelIndicator;

#[derive(Component)]
struct LivesIndicator;

#[derive(Component)]
struct AlertVisibility(bool);

#[derive(Component)]
struct AlertTextNode;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_button = spawn_game_button(
        &mut commands,
        &asset_server,
        GameButton {
            text: "Back to menu".into(),
        },
    );
    commands.entity(menu_button).insert(MenuButton);

    let level_indicator = spawn_game_indicator(
        &mut commands,
        &asset_server,
        GameIndicator {
            label: "Level".into(),
            value: "1/25".into(),
        },
    );
    commands.entity(level_indicator).insert(LevelIndicator);

    let lives_indicator = spawn_game_indicator(
        &mut commands,
        &asset_server,
        GameIndicator {
            label: "Lives".into(),
            value: "1/3".into(),
        },
    );
    commands.entity(lives_indicator).insert(LivesIndicator);

    let complexity_indicator = spawn_game_indicator(
        &mut commands,
        &asset_server,
        GameIndicator {
            label: "Level complexity".into(),
            value: "2".into(),
        },
    );
    commands
        .entity(complexity_indicator)
        .insert(ComplexityIndicator);

    let selection_indicator = spawn_game_indicator(
        &mut commands,
        &asset_server,
        GameIndicator {
            label: "Selected colors".into(),
            value: "0".into(),
        },
    );
    commands
        .entity(selection_indicator)
        .insert(SelectionIndicator);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(GameUIRoot)
        .with_children(|main_container| {
            main_container
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        align_content: AlignContent::Center,
                        margin: UiRect {
                            bottom: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .add_child(level_indicator)
                .add_child(lives_indicator);

            main_container
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0),
                            Val::Percent(75.0),
                        ),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|board_section| {
                    board_section
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(45.0),
                                    Val::Percent(100.0),
                                ),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(ObjectiveColor);

                    board_section
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(45.0),
                                    Val::Percent(100.0),
                                ),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(PlayerColor);
                });

            main_container
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        justify_content: JustifyContent::SpaceAround,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .add_child(complexity_indicator)
                .add_child(selection_indicator);

            main_container
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0),
                            Val::Percent(25.0),
                        ),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|bottom_section| {
                    bottom_section
                        .spawn_bundle(TextBundle {
                            style: Style {
                                display: Display::None,
                                ..default()
                            },
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    color: Color::ORANGE_RED,
                                    font: asset_server.load("edosz.ttf"),
                                    font_size: 40.0,
                                },
                            ),
                            ..default()
                        })
                        .insert(AlertTextNode)
                        .insert(AlertVisibility(true));

                    PALETTE_DATA.iter().for_each(|color| {
                        bottom_section
                            .spawn_bundle(ButtonBundle {
                                image: asset_server.load("brush.png").into(),
                                style: Style {
                                    display: Display::Flex,
                                    size: Size::new(
                                        Val::Px(200.0),
                                        Val::Px(70.0),
                                    ),
                                    ..default()
                                },
                                color: (*color).into(),
                                ..default()
                            })
                            .insert(ColorSelector {
                                color: (*color).into(),
                            })
                            .insert(AlertVisibility(false));
                    });
                });
        })
        .add_child(menu_button);
}

fn teardown(mut commands: Commands, query: Query<Entity, With<GameUIRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_exit_clicked(
    mut app_state: ResMut<State<AppState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
) {
    let clicked = query.iter().any(|interaction| match interaction {
        Interaction::Clicked => true,
        _ => false,
    });

    if clicked {
        app_state.set(AppState::MainMenu).unwrap();
    }
}

fn handle_color_clicked(
    interaction_query: Query<
        (&Interaction, &ColorSelector),
        (Changed<Interaction>, With<Button>),
    >,
    mut board: Option<ResMut<LevelState>>,
    mut evw: EventWriter<PlayerColorsChanged>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    options: Res<GameOptions>,
) {
    if let Some(board) = board.as_mut() {
        for (interaction, color_selection) in &interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    board.selected_colors.push(color_selection.color);
                    evw.send(PlayerColorsChanged);

                    if !options.mute {
                        audio.play(asset_server.load("audio/click.ogg"));
                    }
                }
                _ => (),
            }
        }
    }
}

fn update_player_color(
    mut player_color_query: Query<&mut UiColor, With<PlayerColor>>,
    level: Option<Res<LevelState>>,
) {
    if let Some(level) = level {
        if !level.is_changed() {
            return;
        }

        for mut ui_color in player_color_query.iter_mut() {
            let new_color = mix_colors(&level.selected_colors);
            if ui_color.0 != new_color {
                ui_color.0 = new_color
            }
        }
    }
}

fn update_objective_color(
    level: Option<Res<LevelState>>,
    mut start_evr: EventReader<StartLevelEvent>,
    mut query: Query<&mut UiColor, With<ObjectiveColor>>,
) {
    for _ in start_evr.iter() {
        if let Some(ref level) = level {
            let new_color = mix_colors(&level.objective_colors);
            for mut color in query.iter_mut() {
                color.0 = new_color;
            }
        }
    }
}

fn update_complexity_indicator(
    mut query: Query<&mut GameIndicator, With<ComplexityIndicator>>,
    level: Option<Res<LevelState>>,
) {
    if let Some(level) = level {
        for mut indicator in query.iter_mut() {
            let new_value = level.objective_colors.len().to_string();
            if indicator.value != new_value {
                indicator.value = new_value;
            }
        }
    }
}

fn update_selection_indicator(
    mut query: Query<&mut GameIndicator, With<SelectionIndicator>>,
    level: Option<Res<LevelState>>,
) {
    if let Some(level) = level {
        for mut indicator in query.iter_mut() {
            let new_value = level.selected_colors.len().to_string();
            if indicator.value != new_value {
                indicator.value = new_value;
            }
        }
    }
}

fn update_level_indicator(
    mut query: Query<&mut GameIndicator, With<LevelIndicator>>,
    level: Option<Res<LevelState>>,
) {
    for mut indicator in query.iter_mut() {
        let level_text = level.as_ref().map_or("-".to_string(), |level| {
            format!("{}/{}", level.level_index + 1, OBJECTIVES_DATA.len())
        });

        if indicator.value != level_text {
            indicator.value = level_text;
        }
    }
}

fn update_lives_indicator(
    game: Res<GameState>,
    mut query: Query<&mut GameIndicator, With<LivesIndicator>>,
) {
    if game.is_changed() {
        for mut indicator in query.iter_mut() {
            //let lives_text =
            //    (0..game.lives_remaining).fold(String::new(), |s, _| s + "X");
            let lives_text = format!("{}/{}", game.lives_remaining, 5);
            if indicator.value != lives_text {
                indicator.value = lives_text;
            }
        }
    }
}

fn show_alert(
    mut alert_evr: EventReader<AlertStartedEvent>,
    mut visiblity_query: Query<(&mut Style, &AlertVisibility)>,
    mut text_query: Query<&mut Text, With<AlertTextNode>>,
) {
    if let Some(event) = alert_evr.iter().last() {
        for (mut style, alert_visibility) in visiblity_query.iter_mut() {
            style.display = match alert_visibility.0 {
                true => Display::Flex,
                false => Display::None,
            }
        }

        for mut text in text_query.iter_mut() {
            text.sections[0].value = event.1.clone();
            text.sections[0].style.color = match event.0 {
                crate::game::AlertType::LevelFailed => Color::ORANGE_RED,
                crate::game::AlertType::LevelSucceeded => Color::GREEN,
                crate::game::AlertType::GameLost => Color::RED,
                crate::game::AlertType::GameWon => Color::GREEN,
            };
        }
    }
}

fn hide_alert(
    mut alert_evr: EventReader<AlertEndedEvent>,
    mut visiblity_query: Query<(&mut Style, &AlertVisibility)>,
) {
    if alert_evr.iter().last().is_some() {
        for (mut style, alert_visibility) in visiblity_query.iter_mut() {
            style.display = match alert_visibility.0 {
                true => Display::None,
                false => Display::Flex,
            }
        }
    }
}
