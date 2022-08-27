use bevy::prelude::*;

pub struct GameIndicatorPlugin;

impl Plugin for GameIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label(GameIndicatorLabel)
                .with_system(update_text),
        );
    }
}

/// The system label associated with GameLabel.
/// If your widget depends on GameLabel, its systems should run before this label.
#[derive(SystemLabel, Clone)]
pub struct GameIndicatorLabel;

// contains properties accessible from the outside
#[derive(Component)]
pub struct GameIndicator {
    pub label: String,
    pub value: String,
}

// contains internal state and variables
#[derive(Component)]
struct State {
    text_node: Entity,
}

#[derive(Component)]
struct GameIndicatorText;

pub fn spawn_game_indicator<'a>(
    commands: &'a mut Commands,
    asset_server: &'a Res<AssetServer>,
    indicator: GameIndicator,
) -> Entity {
    let text_node = commands
        .spawn_bundle(TextBundle::from_sections([
            TextSection {
                value: format!("{}: ", indicator.label).into(),
                style: TextStyle {
                    font: asset_server.load("edosz.ttf"),
                    font_size: 24.0,
                    color: Color::BLACK,
                },
            },
            TextSection {
                value: indicator.value.clone(),
                style: TextStyle {
                    font: asset_server.load("edosz.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            },
        ]))
        .insert(GameIndicatorText)
        .id();

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            ..default()
        })
        .add_child(text_node)
        .insert(indicator)
        .insert(State { text_node })
        .id()
}

fn update_text(
    button_q: Query<(&GameIndicator, &State), Changed<GameIndicator>>,
    mut text_q: Query<&mut Text, With<GameIndicatorText>>,
) {
    for (indicator, state) in button_q.iter() {
        if let Ok(mut text) = text_q.get_mut(state.text_node) {
            text.sections[0].value = format!("{}: ", indicator.label).into();
            text.sections[1].value = indicator.value.clone();
        }
    }
}
