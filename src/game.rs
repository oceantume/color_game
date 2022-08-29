use bevy::prelude::*;

use crate::{color_mixer::mix_colors, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetLevelEvent>()
            .add_event::<PrepareLevelEvent>()
            .add_event::<StartLevelEvent>()
            .add_event::<PlayerColorsChanged>()
            .add_event::<LevelSucceededEvent>()
            .add_event::<LevelFailedEvent>()
            .add_event::<GameWonEvent>()
            .add_event::<AlertStartedEvent>()
            .add_event::<AlertEndedEvent>()
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
                    .with_system(check_game_lost)
                    .with_system(update_remaining_lives)
                    .with_system(show_level_succeeded)
                    .with_system(show_level_failed)
                    .with_system(show_game_won)
                    .with_system(setup_alert_timer)
                    .with_system(update_alert_timer)
                    .with_system(
                        update_level_after_alert.after(update_alert_timer),
                    ),
            );
    }
}

const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
const RED: Color = Color::rgb(1.0, 0.0, 0.0);
const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);

pub const PALETTE_DATA: [Color; 5] = [WHITE, RED, YELLOW, BLUE, BLACK];

pub const OBJECTIVES_DATA: &'static [&'static [Color]] = &[
    // Complexity: 2
    &[BLUE, YELLOW],
    &[BLACK, YELLOW],
    /*&[RED, YELLOW],
    &[BLUE, WHITE],
    &[RED, WHITE],
    // Complexity: 3
    &[RED, YELLOW, YELLOW],
    &[YELLOW, BLUE, WHITE],
    &[RED, BLUE, WHITE],
    &[YELLOW, BLACK, BLACK],
    &[YELLOW, BLUE, BLUE],
    &[BLUE, WHITE, RED],
    // Complexity: 4
    &[WHITE, RED, RED, YELLOW],
    &[RED, YELLOW, BLUE, BLACK],
    &[WHITE, YELLOW, YELLOW, BLUE],
    &[WHITE, RED, YELLOW, YELLOW],
    &[RED, RED, YELLOW, BLACK],
    // Complexity: 5
    &[YELLOW, BLACK, BLACK, BLACK, BLACK],
    &[WHITE, WHITE, WHITE, YELLOW, RED],
    &[WHITE, RED, RED, RED, YELLOW],
    &[WHITE, WHITE, WHITE, RED, BLUE],
    &[YELLOW, YELLOW, BLUE, BLUE, BLUE],
    &[WHITE, RED, RED, RED, BLUE],
    // Complexity: 6
    &[WHITE, WHITE, WHITE, WHITE, YELLOW, BLUE],
    &[RED, BLUE, YELLOW, YELLOW, BLACK, BLACK],
    &[WHITE, BLACK, BLACK, RED, RED, RED],
    &[RED, RED, BLUE, BLACK, BLACK, BLACK],
    &[YELLOW, YELLOW, BLUE, BLACK, BLACK, BLACK],*/
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
    pub fn new(level_index: u32, colors: Vec<Color>) -> Self {
        Self {
            level_index,
            selected_colors: default(),
            objective_colors: colors,
        }
    }

    pub fn reset(&mut self) {
        self.selected_colors.clear();
    }
    
    pub fn is_last_level(&self) -> bool {
        self.level_index == (OBJECTIVES_DATA.len() - 1) as u32
    }

    pub fn prepare_objective(level_index: u32) -> Option<Vec<Color>> {
        if level_index >= OBJECTIVES_DATA.len() as u32 {
            None
        } else {
            Some(OBJECTIVES_DATA[level_index as usize].into())
        }
    }
}

struct AlertTimer(Timer, AlertType);

#[derive(Component)]
pub struct ColorSelector {
    pub color: Color,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GameState { lives_remaining: 5 });
}

fn teardown(mut commands: Commands) {
    commands.remove_resource::<LevelState>();
    commands.remove_resource::<AlertTimer>();
}

pub struct PrepareLevelEvent;
pub struct ResetLevelEvent;
pub struct StartLevelEvent(pub u32);
pub struct PlayerColorsChanged;
pub struct LevelSucceededEvent;
pub struct LevelFailedEvent;
pub struct GameWonEvent;

#[derive(Clone, Copy)]
pub enum AlertType {
    LevelFailed,
    LevelSucceeded,
    GameLost,
    GameWon,
}

pub struct AlertStartedEvent(pub AlertType, pub String);
pub struct AlertEndedEvent(pub AlertType);

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
        let objective_colors = LevelState::prepare_objective(level_index);
        if let Some(objective_colors) = objective_colors {
            let new_level = LevelState::new(level_index, objective_colors);
            commands.insert_resource(new_level);
            start_evw.send(StartLevelEvent(level_index));
        }
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
    mut won_evw: EventWriter<GameWonEvent>,
    mut succeeded_evw: EventWriter<LevelSucceededEvent>,
    mut failed_evw: EventWriter<LevelFailedEvent>,
) {
    if evr.iter().count() < 1 {
        return;
    }

    if let Some(ref level) = level {
        let player_color = mix_colors(&level.selected_colors);
        let objective_color = mix_colors(&level.objective_colors);

        if player_color == objective_color {
            if level.is_last_level() {
                won_evw.send(GameWonEvent);
            } else {
                succeeded_evw.send(LevelSucceededEvent);
            }
        } else if level.selected_colors.len() >= level.objective_colors.len() {
            failed_evw.send(LevelFailedEvent);
        }
    }
}

fn show_level_succeeded(
    mut succeeded_evr: EventReader<LevelSucceededEvent>,
    mut alert_evw: EventWriter<AlertStartedEvent>,
) {
    if succeeded_evr.iter().count() > 0 {
        alert_evw.send(AlertStartedEvent(
            AlertType::LevelSucceeded,
            "Right answer!".into(),
        ))
    }
}

fn show_level_failed(
    mut failed_evr: EventReader<LevelFailedEvent>,
    mut alert_evw: EventWriter<AlertStartedEvent>,
) {
    if failed_evr.iter().count() > 0 {
        alert_evw.send(AlertStartedEvent(
            AlertType::LevelFailed,
            "Wrong answer!".into(),
        ))
    }
}

fn show_game_won(
    mut won_evr: EventReader<GameWonEvent>,
    mut alert_evw: EventWriter<AlertStartedEvent>,
) {
    if won_evr.iter().count() > 0 {
        alert_evw.send(AlertStartedEvent(
            AlertType::GameWon,
            "Congratulations, you won the game! This is quite the feat!".into()
        ))
    }
}

fn setup_alert_timer(
    mut commands: Commands,
    mut evr: EventReader<AlertStartedEvent>,
) {
    if let Some(event) = evr.iter().last() {
        let duration = match event.0 {
            AlertType::LevelFailed => Some(2.5),
            AlertType::LevelSucceeded => Some(1.0),
            AlertType::GameLost => None,
            AlertType::GameWon => None,
        };

        if let Some(duration) = duration {
            commands.insert_resource(AlertTimer(
                Timer::from_seconds(duration, false),
                event.0,
            ));
        }
    }
}

fn update_alert_timer(
    mut commands: Commands,
    time: Res<Time>,
    alert_timer: Option<ResMut<AlertTimer>>,
    mut evw: EventWriter<AlertEndedEvent>,
) {
    alert_timer.map(|mut alert_timer| {
        alert_timer.0.tick(time.delta());

        if alert_timer.0.finished() {
            evw.send(AlertEndedEvent(alert_timer.1));
            commands.remove_resource::<AlertTimer>();
        }
    });
}

fn update_level_after_alert(
    mut evr: EventReader<AlertEndedEvent>,
    mut reset_evw: EventWriter<ResetLevelEvent>,
    mut prepare_evw: EventWriter<PrepareLevelEvent>,
) {
    if let Some(event) = evr.iter().last() {
        match event.0 {
            AlertType::LevelFailed => reset_evw.send(ResetLevelEvent),
            AlertType::LevelSucceeded => prepare_evw.send(PrepareLevelEvent),
            _ => (),
        }
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

fn check_game_lost(
    game: ResMut<GameState>,
    mut alert_evw: EventWriter<AlertStartedEvent>,
) {
    if game.lives_remaining == 0 {
        alert_evw.send(AlertStartedEvent(
            AlertType::GameLost,
            "You lost!".into(),
        ));
    }
}
