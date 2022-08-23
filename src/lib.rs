use bevy::prelude::*;

mod color_mixer;
use crate::color_mixer::mix_colors;

pub const LAUNCHER_TITLE: &str = "Bevy Jam #2";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
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
    .insert_resource(BoardState {
        selected_colors: Vec::new(),
    })
    .add_plugins(DefaultPlugins)
    .add_state(AppState::InGame)
    .add_startup_system(setup)
    .add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(setup_board)
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(color_button_clicked)
            .with_system(player_color_update)
    );
    app
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_board(mut commands: Commands) {
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
        });
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
    mut player_color_query: Query<(&mut UiColor), With<PlayerColor>>,
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
