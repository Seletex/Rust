// ============================================================================
// GAME - Plugin principal que agrupa toda la lógica del juego
// ============================================================================
// Arquitectura "Microkernel": GameplayPlugin agrupa sub-plugins independientes
// Cada plugin encapsula una responsabilidad (Player, Toro, Mundo, Audio, UI)
// Comunicación via Events/Messages (desacoplado)

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
            // Movimiento/jugador (input, gravedad)
            .add_plugins(PlayerPlugin)
            // Toro (movido a world.rs por query conflicts)
            .add_plugins(BullPlugin)
            // Mundo: entidades, colisiones, toro, estados
            .add_plugins(WorldPlugin)
            // Audio: música + FX reactivos
            .add_plugins(AudioPlugin)
            // UI: menús + input
            .add_plugins(UIPlugin)
            // Eventos de flujo de estado (iniciar/reiniciar)
            .add_systems(Update, handle_game_start.run_if(in_state(GameState::Menu)))
            .add_systems(Update, handle_game_restart.run_if(in_state(GameState::GameOver).or_else(in_state(GameState::Victoria))));
    }
}

/// EventoJuegoIniciado (Space/Enter en Menu) → State: Menu → Jugando
/// AudioPlugin reacciona OnEnter(Jugando) → cambia música
fn handle_game_start(
    mut ev_inicio: MessageReader<EventoJuegoIniciado>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_inicio.read() {
        next_state.set(GameState::Jugando);
    }
}

/// EventoJuegoReiniciado (Space/Enter en GameOver/Victoria) → State: GameOver/Victoria → Jugando
fn handle_game_restart(
    mut ev_reinicio: MessageReader<EventoJuegoReiniciado>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in ev_reinicio.read() {
        next_state.set(GameState::Jugando);
    }
}