use bevy::prelude::*;

use crate::{color_mixer::mix_colors, AppState};

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoardState {
            selected_colors: Vec::new(),
        })
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame).with_system(teardown),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(exit_clicked)
                .with_system(color_button_clicked)
                .with_system(player_color_update),
        );
    }
}
#[derive(Component)]
struct GameUIRoot;

#[derive(Component)]
struct MenuButton;

#[derive(Component)]
struct TargetColor;

#[derive(Component)]
struct PlayerColor;

struct BoardState {
    selected_colors: Vec<Color>,
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
                            color: Color::YELLOW.into(),
                            ..default()
                        })
                        .insert(TargetColor);

                    top_bar
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(45.0),
                                    Val::Percent(100.0),
                                ),
                                ..default()
                            },
                            color: Color::CYAN.into(),
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
                            //color: Color::RED.into(),
                            color: Color::rgb(1.0, 0.0, 1.0).into(),
                            ..default()
                        })
                        .insert(ColorSelector {
                            //color: Color::rgb(1.0, 0.0, 0.0)
                            color: Color::rgb(1.0, 0.0, 1.0).into(),
                        });
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
                            //color: Color::GREEN.into(),
                            color: Color::YELLOW.into(),
                            ..default()
                        })
                        .insert(ColorSelector {
                            //color: Color::rgb(0.0, 1.0, 0.0)
                            color: Color::YELLOW.into(),
                        });
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
                            //color: Color::BLUE.into(),
                            color: Color::CYAN.into(),
                            ..default()
                        })
                        .insert(ColorSelector {
                            //color: Color::rgb(0.0, 0.0, 1.0)
                            color: Color::CYAN.into(),
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

fn exit_clicked(
    mut app_state: ResMut<State<AppState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
) {
    let clicked = query.iter().any(|interaction| match interaction {
        Interaction::Clicked => true,
        _ => false,
    });

    if clicked {
        info!("exit");
        app_state.set(AppState::MainMenu).unwrap();
    }
}

fn color_button_clicked(
    interaction_query: Query<
        (&Interaction, &ColorSelector),
        (Changed<Interaction>, With<Button>),
    >,
    mut board: ResMut<BoardState>,
) {
    for (interaction, color_selection) in &interaction_query {
        match *interaction {
            Interaction::Clicked => {
                board.selected_colors.push(color_selection.color);
            }
            _ => (),
        }
    }
}

fn player_color_update(
    mut player_color_query: Query<&mut UiColor, With<PlayerColor>>,
    board: Res<BoardState>,
) {
    for mut ui_color in player_color_query.iter_mut() {
        let new_color = mix_colors(&board.selected_colors);
        if ui_color.0 != new_color {
            info!("setting color {:?}", new_color);
            ui_color.0 = new_color
        }
    }
}
