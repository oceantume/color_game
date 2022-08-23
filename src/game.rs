use bevy::prelude::*;

use crate::{color_mixer::mix_colors, AppState};

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PrepareLevelEvent>()
            .add_event::<StartLevelEvent>()
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
                    .with_system(exit_clicked)
                    .with_system(color_button_clicked)
                    .with_system(player_color_update)
                    .with_system(prepare_level),
            );
    }
}

const PALETTE_DATA: [Color; 5] = [
    Color::rgb(0.0, 0.0, 0.0),
    Color::rgb(1.0, 0.0, 0.0),
    Color::rgb(1.0, 1.0, 0.0),
    Color::rgb(0.0, 0.0, 1.0),
    Color::rgb(1.0, 1.0, 1.0),
];

// TODO: add more objectives.
// TODO: generate randomized objectives with increasing difficulty.
const OBJECTIVES_DATA: [&'static [Color]; 1] =
    [&[PALETTE_DATA[3], PALETTE_DATA[4]]];

#[derive(Component)]
struct GameUIRoot;

#[derive(Component)]
struct MenuButton;

#[derive(Component)]
struct ObjectiveColor;

#[derive(Component)]
struct PlayerColor;

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
) {
    if let Some(board) = board.as_mut() {
        for (interaction, color_selection) in &interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    board.selected_colors.push(color_selection.color);
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

struct PrepareLevelEvent;
struct StartLevelEvent(u32);

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
