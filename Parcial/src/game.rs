use bevy::prelude::*;
use crate::player::PlayerPlugin;
use crate::bull::BullPlugin;
use crate::world::WorldPlugin;
use crate::audio::AudioPlugin;
use crate::ui::UIPlugin;
use crate::events::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(BullPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(AudioPlugin)
            .add_plugins(UIPlugin)
            .add_systems(Update, handle_game_start.run_if(in_state(GameState::Menu)))
            .add_systems(Update, handle_game_restart.run_if(in_state(GameState::GameOver).or_else(in_state(GameState::Victoria))));
    }
}

fn handle_game_start(
    mut ev_inicio: MessageReader<EventoJuegoIniciado>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_inicio.read() {
        next_state.set(GameState::Jugando);
    }
}

fn handle_game_restart(
    mut ev_reinicio: MessageReader<EventoJuegoReiniciado>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_reinicio.read() {
        next_state.set(GameState::Jugando);
    }
}