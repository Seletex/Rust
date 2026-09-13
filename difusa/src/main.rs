mod components;
mod events;
mod systems;
mod states;

use bevy::prelude::*;
use events::*;
use states::GameState;
use systems::audio_systems::*;
use systems::game_logic::*;
use systems::setup::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Juego Bevy - Modularizado".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_message::<EventoMovimientoJugador>()
        .add_message::<EventoJuegoPerdido>()
        .add_message::<EventoJuegoGanado>()
        .add_systems(Startup, inicializar_entidades_y_audios)
        .add_systems(OnEnter(GameState::Inicio), reproducir_musica_inicio)
        .add_systems(
            OnEnter(GameState::Jugando),
            (reproducir_musica_juego, reiniciar_posiciones),
        )
        .add_systems(OnEnter(GameState::Victoria), reproducir_audio_victoria)
        .add_systems(OnEnter(GameState::Derrota), reproducir_audio_derrota)
        .add_systems(
            Update,
            esperar_inicio_juego.run_if(in_state(GameState::Inicio)),
        )
        .add_systems(
            Update,
            esperar_reinicio_juego.run_if(
                in_state(GameState::Victoria).or_else(in_state(GameState::Derrota)),
            ),
        )
        .add_systems(
            Update,
            (
                controlar_jugador,
                perseguir_jugador,
                aplicar_velocidad,
                reproducir_sonido_pasos,
            )
                .run_if(in_state(GameState::Jugando)),
        )
        .add_systems(
            PostUpdate,
            (
                validar_limites,
                detectar_colisiones,
                procesar_eventos_partida,
            )
                .run_if(in_state(GameState::Jugando)),
        )
        .run();
}