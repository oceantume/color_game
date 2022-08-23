use bevy::prelude::*;

use crate::AppState;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu).with_system(teardown),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(play),
        );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                font_size: 40.0,
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
                                font_size: 20.0,
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
                        size: Size::new(Val::Auto, Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        //justify_content: JustifyContent::SpaceAround,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|menu| {
                    menu.spawn_bundle(ButtonBundle {
                        color: Color::NONE.into(),
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|play_btn| {
                        play_btn
                            .spawn_bundle(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("edosz.ttf"),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            ));
                    })
                    .insert(PlayButton);
                    menu.spawn_bundle(ButtonBundle {
                        color: Color::NONE.into(),
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|play_btn| {
                        play_btn.spawn_bundle(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font: asset_server.load("edosz.ttf"),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        ));
                    });
                });
        });
}

fn teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn play(
    mut app_state: ResMut<State<AppState>>,
    query: Query<&Interaction, With<PlayButton>>,
) {
    let clicked = query.iter().any(|interaction| match interaction {
        Interaction::Clicked => true,
        _ => false,
    });

    if clicked {
        app_state.set(AppState::InGame).unwrap();
    }
}
