use bevy::prelude::*;

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label(GameButtonLabel)
                .with_system(update_text)
                .with_system(handle_button_hover),
        );
    }
}

/// The system label associated with GameButton.
/// If your widget depends on GameButton, its systems should run before this label.
#[derive(SystemLabel, Clone)]
pub struct GameButtonLabel;

// contains properties accessible from the outside
#[derive(Component)]
pub struct GameButton {
    pub text: String,
}

// contains internal state and variables
#[derive(Component)]
struct State {
    text_node: Entity,
}

#[derive(Component)]
struct GameButtonText;

pub fn spawn_game_button<'a>(
    commands: &'a mut Commands,
    asset_server: &'a Res<AssetServer>,
    game_button: GameButton,
) -> Entity {
    let text_node = commands
        .spawn_bundle(TextBundle::from_section(
            game_button.text.clone(),
            TextStyle {
                font: asset_server.load("edosz.ttf"),
                font_size: 30.0,
                color: Color::BLACK,
            },
        ))
        .insert(GameButtonText)
        .id();

    commands
        .spawn_bundle(ButtonBundle {
            color: Color::NONE.into(),
            style: Style {
                padding: UiRect::all(Val::Px(10.0)),
                size: Size::new(Val::Undefined, Val::Px(10.0 + 35.0)),
                ..default()
            },
            ..default()
        })
        .insert(game_button)
        .insert(State { text_node })
        .add_child(text_node)
        .id()
}

fn update_text(
    button_q: Query<(&GameButton, &State), Changed<GameButton>>,
    mut text_q: Query<&mut Text, With<GameButtonText>>,
) {
    for (button, state) in button_q.iter() {
        if let Ok(mut text) = text_q.get_mut(state.text_node) {
            text.sections[0].value = button.text.clone();
        }
    }
}

fn handle_button_hover(
    button_q: Query<(&State, &Interaction), Changed<Interaction>>,
    mut text_q: Query<&mut Text, With<GameButtonText>>,
) {
    for (state, interaction) in button_q.iter() {
        if let Ok(mut text) = text_q.get_mut(state.text_node) {
            match interaction {
                Interaction::Clicked => (),
                Interaction::Hovered => {
                    text.sections.iter_mut().for_each(|s| {
                        s.style.color = Color::WHITE.into();
                        s.style.font_size = 35.0;
                    });
                }
                Interaction::None => {
                    text.sections.iter_mut().for_each(|s| {
                        s.style.color = Color::BLACK.into();
                        s.style.font_size = 30.0;
                    });
                }
            }
        }
    }
}
