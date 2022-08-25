use bevy::prelude::*;

use crate::{color_mixer::mix_colors, AppState};

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetLevelEvent>()
            .add_event::<PrepareLevelEvent>()
            .add_event::<StartLevelEvent>()
            .add_event::<PlayerColorsChanged>()
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                    .with_system(setup)
                    .with_system(prepare_first_level),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame).with_system(teardown),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("ui_update")
                    .with_system(player_color_update)
                    .with_system(update_objective_color)
                    .with_system(update_complexity_level_text)
                    .with_system(update_selected_colors_text),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    // NOTE: UI update needs to run before prepare_level because the resource
                    // is inserted via command on the previous frame, and the event can be received
                    // in the same frame.
                    .after("ui_update")
                    .with_system(exit_clicked)
                    .with_system(color_button_clicked)
                    .with_system(prepare_level)
                    .with_system(reset_level)
                    .with_system(check_level_finished),
            );
    }
}
const PALETTE_WHITE: Color = Color::rgb(0.0, 0.0, 0.0);
const PALETTE_RED: Color = Color::rgb(1.0, 0.0, 0.0);
const PALETTE_YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
const PALETTE_BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
const PALETTE_BLACK: Color = Color::rgb(1.0, 1.0, 1.0);

const PALETTE_DATA: [Color; 5] = [
    PALETTE_WHITE,
    PALETTE_RED,
    PALETTE_YELLOW,
    PALETTE_BLUE,
    PALETTE_BLACK,
];

// TODO: add more objectives.
// TODO: generate randomized objectives with increasing difficulty.
const OBJECTIVES_DATA: [&'static [Color]; 7] = [
    &[PALETTE_BLUE, PALETTE_BLACK],
    &[PALETTE_RED, PALETTE_YELLOW],
    &[PALETTE_BLUE, PALETTE_BLACK],
    &[PALETTE_RED, PALETTE_WHITE],
    &[PALETTE_YELLOW, PALETTE_BLUE, PALETTE_WHITE],
    &[PALETTE_RED, PALETTE_BLUE, PALETTE_WHITE],
    &[PALETTE_YELLOW, PALETTE_BLACK, PALETTE_BLACK],
];

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

struct LevelState {
    pub level_index: u32,
    pub selected_colors: Vec<Color>,
    pub objective_colors: Vec<Color>,
}

impl LevelState {
    pub fn new(level_index: u32) -> Self {
        Self {
            level_index,
            selected_colors: default(),
            objective_colors: Self::prepare_objective(level_index),
        }
    }

    pub fn reset(&mut self) {
        self.selected_colors.clear();
    }

    fn prepare_objective(level_index: u32) -> Vec<Color> {
        if level_index > OBJECTIVES_DATA.len() as u32 {
            panic!("Reached the end of objectives");
        }

        OBJECTIVES_DATA[level_index as usize].into()
    }
}

#[derive(Component)]
struct ColorSelector {
    color: Color,
}

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
                .with_children(|top_bar| {
                    top_bar
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

                    top_bar
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
                .with_children(|bottom_bar| {
                    PALETTE_DATA.iter().for_each(|color| {
                        bottom_bar
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
    commands.remove_resource::<LevelState>();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn exit_clicked(
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

fn color_button_clicked(
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

fn player_color_update(
    mut player_color_query: Query<&mut UiColor, With<PlayerColor>>,
    level: Option<Res<LevelState>>,
) {
    if let Some(level) = level {
        for mut ui_color in player_color_query.iter_mut() {
            let new_color = mix_colors(&level.selected_colors);
            if ui_color.0 != new_color {
                info!("setting color {:?}", new_color);
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

struct PrepareLevelEvent;
struct ResetLevelEvent;
struct StartLevelEvent(u32);
struct PlayerColorsChanged;

fn prepare_first_level(mut events: EventWriter<PrepareLevelEvent>) {
    events.send(PrepareLevelEvent);
}

fn prepare_level(
    mut commands: Commands,
    level: Option<Res<LevelState>>,
    mut prepare_evr: EventReader<PrepareLevelEvent>,
    mut start_evw: EventWriter<StartLevelEvent>,
) {
    for _ in prepare_evr.iter() {
        let level_index =
            level.as_ref().map_or(0, |level| level.level_index + 1);
        let new_level = LevelState::new(level_index);
        commands.insert_resource(new_level);

        start_evw.send(StartLevelEvent(level_index));
        debug!("Prepared level {}", level_index);
    }
}

fn reset_level(
    mut level: Option<ResMut<LevelState>>,
    mut evr: EventReader<ResetLevelEvent>,
) {
    if evr.iter().count() < 1 {
        return;
    }
    
    if let Some(ref mut level) = level {
        level.reset();
    }
}

fn check_level_finished(
    level: Option<ResMut<LevelState>>,
    mut evr: EventReader<PlayerColorsChanged>,
    mut prepare_evw: EventWriter<PrepareLevelEvent>,
    mut reset_evw: EventWriter<ResetLevelEvent>,
) {
    if evr.iter().count() < 1 {
        return;
    }

    if let Some(ref level) = level {
        let player_color = mix_colors(&level.selected_colors);
        let objective_color = mix_colors(&level.objective_colors);

        if player_color == objective_color {
            prepare_evw.send(PrepareLevelEvent);
        } else if level.selected_colors.len() >= level.objective_colors.len() {
            // todo: send a level failure event instead and let that be handled.
            reset_evw.send(ResetLevelEvent);
        }
    }
}
