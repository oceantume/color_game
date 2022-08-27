use bevy::prelude::*;

use crate::{color_mixer::mix_colors, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetLevelEvent>()
            .add_event::<PrepareLevelEvent>()
            .add_event::<StartLevelEvent>()
            .add_event::<PlayerColorsChanged>()
            .add_event::<LevelFailedEvent>()
            .add_event::<GameOverEvent>()
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
                    // NOTE: UI update needs to run before prepare_level because the resource
                    // is inserted via command on the previous frame, and the event can be received
                    // in the same frame.
                    .after("ui_update")
                    .with_system(prepare_level)
                    .with_system(reset_level)
                    .with_system(check_level_finished)
                    .with_system(check_game_over)
                    .with_system(show_game_over)
                    .with_system(update_remaining_lives)
                    .with_system(show_level_failed),
            );
    }
}

const PALETTE_WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
const PALETTE_RED: Color = Color::rgb(1.0, 0.0, 0.0);
const PALETTE_YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
const PALETTE_BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
const PALETTE_BLACK: Color = Color::rgb(0.0, 0.0, 0.0);

pub const PALETTE_DATA: [Color; 5] = [
    PALETTE_WHITE,
    PALETTE_RED,
    PALETTE_YELLOW,
    PALETTE_BLUE,
    PALETTE_BLACK,
];

// TODO: add more objectives.
// TODO: generate randomized objectives with increasing difficulty.
pub const OBJECTIVES_DATA: [&'static [Color]; 11] = [
    &[PALETTE_BLUE, PALETTE_YELLOW],
    &[PALETTE_BLACK, PALETTE_YELLOW],
    &[PALETTE_RED, PALETTE_YELLOW],
    &[PALETTE_BLUE, PALETTE_WHITE],
    &[PALETTE_RED, PALETTE_WHITE],
    &[PALETTE_RED, PALETTE_YELLOW, PALETTE_YELLOW],
    &[PALETTE_YELLOW, PALETTE_BLUE, PALETTE_WHITE],
    &[PALETTE_RED, PALETTE_BLUE, PALETTE_WHITE],
    &[PALETTE_YELLOW, PALETTE_BLACK, PALETTE_BLACK],
    &[PALETTE_RED, PALETTE_RED, PALETTE_YELLOW, PALETTE_WHITE],
    &[PALETTE_RED, PALETTE_BLUE, PALETTE_YELLOW, PALETTE_BLACK],
];

pub struct GameState {
    pub lives_remaining: u32,
}

pub struct LevelState {
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
pub struct ColorSelector {
    pub color: Color,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GameState { lives_remaining: 5 });
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<LevelState>();
}

pub struct PrepareLevelEvent;
pub struct ResetLevelEvent;
pub struct StartLevelEvent(u32);
pub struct PlayerColorsChanged;
pub struct LevelFailedEvent;
pub struct GameOverEvent;

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
    mut failed_evw: EventWriter<LevelFailedEvent>,
) {
    if evr.iter().count() < 1 {
        return;
    }

    if let Some(ref level) = level {
        let player_color = mix_colors(&level.selected_colors);
        let objective_color = mix_colors(&level.objective_colors);

        if player_color == objective_color {
            // todo: send a level success event instead so we can show the success.
            prepare_evw.send(PrepareLevelEvent);
        } else if level.selected_colors.len() >= level.objective_colors.len() {
            // todo: send a level failure event instead so we can show the failure.
            failed_evw.send(LevelFailedEvent);
        }
    }
}

fn show_level_failed(
    mut failed_evr: EventReader<LevelFailedEvent>,
    mut reset_evw: EventWriter<ResetLevelEvent>,
) {
    if failed_evr.iter().count() > 0 {
        // todo: show level failed on UI before resetting
        reset_evw.send(ResetLevelEvent);
    }
}

fn update_remaining_lives(
    mut game: ResMut<GameState>,
    mut evr: EventReader<LevelFailedEvent>,
) {
    for _ in evr.iter() {
        game.lives_remaining -= 1;
    }
}

fn check_game_over(
    game: ResMut<GameState>,
    mut evw: EventWriter<GameOverEvent>,
) {
    if game.lives_remaining == 0 {
        evw.send(GameOverEvent);
    }
}

fn show_game_over(
    mut evr: EventReader<GameOverEvent>,
    mut app_state: ResMut<State<AppState>>,
) {
    if evr.iter().count() > 0 {
        // todo: show game over popup before leaving to main menu
        app_state.set(AppState::MainMenu).unwrap();
    }
}
