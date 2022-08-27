use bevy::prelude::*;

use crate::{color_mixer::mix_colors, AppState};

use crate::game::{
    ColorSelector, GameState, LevelState, PlayerColorsChanged, StartLevelEvent,
    PALETTE_DATA,
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
                .with_system(handle_exit_clicked)
                .with_system(update_player_color)
                .with_system(update_objective_color)
                .with_system(update_complexity_level_text)
                .with_system(update_selected_colors_text)
                .with_system(update_current_level_indicator)
                .with_system(update_remaining_lives_indicator)
                .with_system(handle_color_clicked),
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
struct ComplexityLevelText;

#[derive(Component)]
struct SelectedColorsText;

#[derive(Component)]
struct CurrentLevelIndicator;

#[derive(Component)]
struct RemainingLivesIndicator;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                .with_children(|top_section| {
                    top_section
                        .spawn_bundle(NodeBundle {
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|indicator_container| {
                            indicator_container
                                .spawn_bundle(TextBundle::from_sections([
                                    TextSection {
                                        value: "Current level: ".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 24.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                    TextSection {
                                        value: "1".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                ]))
                                .insert(CurrentLevelIndicator);
                        });

                    top_section
                        .spawn_bundle(NodeBundle {
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|indicator_container| {
                            indicator_container
                                .spawn_bundle(TextBundle::from_sections([
                                    TextSection {
                                        value: "Remaining lives: ".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 24.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                    TextSection {
                                        value: "XXXXX".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                ]))
                                .insert(RemainingLivesIndicator);
                        });
                });

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
                .with_children(|middle_container| {
                    middle_container
                        .spawn_bundle(NodeBundle {
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|container| {
                            container
                                .spawn_bundle(TextBundle::from_sections([
                                    TextSection {
                                        value: "Complexity level: ".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 24.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                    TextSection {
                                        value: "".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                ]))
                                .insert(ComplexityLevelText);
                        });
                    middle_container
                        .spawn_bundle(NodeBundle {
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|container| {
                            container
                                .spawn_bundle(TextBundle::from_sections([
                                    TextSection {
                                        value: "Selected colors: ".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 24.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                    TextSection {
                                        value: "".into(),
                                        style: TextStyle {
                                            font: asset_server
                                                .load("edosz.ttf"),
                                            font_size: 30.0,
                                            color: Color::BLACK,
                                        },
                                    },
                                ]))
                                .insert(SelectedColorsText);
                        });
                });

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
                    PALETTE_DATA.iter().for_each(|color| {
                        bottom_section
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    max_size: Size::new(
                                        Val::Px(200.0),
                                        Val::Undefined,
                                    ),
                                    size: Size::new(
                                        Val::Percent(20.0),
                                        Val::Percent(80.0),
                                    ),
                                    ..default()
                                },
                                color: (*color).into(),
                                ..default()
                            })
                            .insert(ColorSelector {
                                color: (*color).into(),
                            });
                    });
                });

            main_container
                .spawn_bundle(ButtonBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                })
                .insert(MenuButton)
                .with_children(|btn| {
                    btn.spawn_bundle(TextBundle::from_section(
                        "Menu",
                        TextStyle {
                            font: asset_server.load("edosz.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
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
) {
    if let Some(board) = board.as_mut() {
        for (interaction, color_selection) in &interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    board.selected_colors.push(color_selection.color);
                    evw.send(PlayerColorsChanged);
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

fn update_complexity_level_text(
    mut text_query: Query<&mut Text, With<ComplexityLevelText>>,
    level: Option<Res<LevelState>>,
) {
    if let Some(level) = level {
        for mut text in text_query.iter_mut() {
            let new_value = level.objective_colors.len().to_string();
            if text.sections[1].value != new_value {
                text.sections[1].value = new_value;
            }
        }
    }
}

fn update_selected_colors_text(
    mut text_query: Query<&mut Text, With<SelectedColorsText>>,
    level: Option<Res<LevelState>>,
) {
    if let Some(level) = level {
        for mut text in text_query.iter_mut() {
            let new_value = level.selected_colors.len().to_string();
            if text.sections[1].value != new_value {
                text.sections[1].value = new_value;
            }
        }
    }
}

fn update_current_level_indicator(
    mut text_query: Query<&mut Text, With<CurrentLevelIndicator>>,
    level: Option<Res<LevelState>>,
) {
    for mut text in text_query.iter_mut() {
        let level_text = level.as_ref().map_or("-".to_string(), |level| {
            (level.level_index + 1).to_string()
        });

        if text.sections[1].value != level_text {
            text.sections[1].value = level_text;
        }
    }
}

fn update_remaining_lives_indicator(
    game: Res<GameState>,
    mut query: Query<&mut Text, With<RemainingLivesIndicator>>,
) {
    if game.is_changed() {
        for mut text in query.iter_mut() {
            let lives_text =
                (0..game.lives_remaining).fold(String::new(), |s, _| s + "X");
            if text.sections[1].value != lives_text {
                text.sections[1].value = lives_text;
            }
        }
    }
}
