use bevy::prelude::*;

#[derive(States, Hash, Debug, Clone, Eq, PartialEq)]
pub enum GameState {
    Setup,
    Playing,
}

pub fn game_state_plugin(app: &mut App) {
    app.insert_state(GameState::Setup).add_systems(
        Update,
        (|mut next_state: ResMut<NextState<GameState>>| next_state.set(GameState::Playing))
            .run_if(in_state(GameState::Setup)),
    );
}
