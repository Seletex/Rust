// ============================================================================
// AUDIO - Música de fondo + Efectos de sonido (FX) reactivos a eventos
// ============================================================================
// Arquitectura:
// - Música (LOOP): se reproduce al cambiar estados (Menu/Juego/Victoria/Derrota)
//   Usa componente MusicaFondo para limpiar anterior al cambiar
// - FX (DESPAWN): sonidos instantáneos por eventos (salto, colisión, toro)
//   No interfieren con música, se auto-destruyen al terminar

use bevy::prelude::*;
use bevy::audio::{Volume, PlaybackSettings, AudioPlayer};
use crate::components::MusicaFondo;
use crate::events::{GameState, EventoSaltoJugador, EventoColisionObstaculo, EventoToroAtrapaJugador};

/// Colección de handles a archivos de audio (cargados en Startup)
#[derive(Resource)]
#[allow(dead_code)]
pub struct ColeccionAudioProcedural {
    pub menu: Handle<AudioSource>,    // Música menú principal
    pub juego: Handle<AudioSource>,   // Música durante partida
    pub victoria: Handle<AudioSource>, // Música victoria
    pub derrota: Handle<AudioSource>,  // Música derrota/game over
    pub pasos: Handle<AudioSource>,    // No usado (legacy)
    pub salto: Handle<AudioSource>,    // FX salto
}

/// Despawn todas las entidades con MusicaFondo (limpia música anterior)
fn detener_musica_anterior(commands: &mut Commands, query: &Query<Entity, With<MusicaFondo>>) {
    for entidad in query.iter() {
        commands.entity(entidad).despawn();
    }
}

// --- Startup: Carga todos los audios e inicia música menú ---
pub fn cargar_audios(mut commands: Commands, asset_server: Res<AssetServer>) {
    let coleccion = ColeccionAudioProcedural {
        menu: asset_server.load("audio/menu.mp3"),
        juego: asset_server.load("audio/juego.mp3"),
        victoria: asset_server.load("audio/victoria.mp3"),
        derrota: asset_server.load("audio/derrota.mp3"),
        pasos: asset_server.load("audio/pasos.mp3"),
        salto: asset_server.load("audio/salto.mp3"),
    };

    commands.insert_resource(coleccion);

    // Música menú inmediata (LOOP, vol 0.4)
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/menu.mp3")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.4)),
        MusicaFondo,  // Marca para limpieza futura
    ));
}

// --- OnEnter(Jugando): Cambia a música juego (LOOP, vol 0.5) ---
fn reproducir_musica_juego(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        info!("🎵 Cambiando a música de JUEGO (LOOP)");
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.juego.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.5)),
            MusicaFondo,
        ));
    }
}

// --- OnEnter(Victoria): Música victoria ONCE (vol 1.0) ---
fn reproducir_audio_victoria(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        info!("🎵 Reproduciendo música de VICTORIA (ONCE) - vol 1.0");
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.victoria.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(1.0)),
        ));
    }
}

// --- OnEnter(GameOver): Música derrota ONCE (vol 1.0) ---
fn reproducir_audio_derrota(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        info!("🎵 Reproduciendo música de DERROTA (ONCE) - vol 1.0");
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.derrota.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(1.0)),
        ));
    }
}

// ============================================================================
// FX INSTANTÁNEOS (DESPAWN) - Reaccionan a eventos via MessageReader
// ============================================================================

/// EventoSaltoJugador → FX salto (vol 0.4, no choca con música)
fn sonido_salto(
    mut commands: Commands,
    mut ev_salto: MessageReader<EventoSaltoJugador>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_salto.read() {
        info!("🔊 FX: Salto");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.salto.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.4)),
            ));
        }
    }
}

/// EventoColisionObstaculo → FX colisión (vol 0.5)
fn sonido_colision(
    mut commands: Commands,
    mut ev_colision: MessageReader<EventoColisionObstaculo>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_colision.read() {
        info!("🔊 FX: Colisión obstáculo");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.derrota.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.5)),
            ));
        }
    }
}

/// EventoToroAtrapaJugador → FX toro (vol 0.7)
fn sonido_toro(
    mut commands: Commands,
    mut ev_toro: MessageReader<EventoToroAtrapaJugador>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_toro.read() {
        info!("🔊 FX: Toro atrapa");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.derrota.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.7)),
            ));
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cargar_audios)
            // Transiciones de música por estado
            .add_systems(OnEnter(GameState::Jugando), reproducir_musica_juego)
            .add_systems(OnEnter(GameState::Victoria), reproducir_audio_victoria)
            .add_systems(OnEnter(GameState::GameOver), reproducir_audio_derrota)
            // FX instantáneos por eventos (solo en Jugando)
            .add_systems(Update, sonido_salto.run_if(in_state(GameState::Jugando)))
            .add_systems(Update, sonido_colision.run_if(in_state(GameState::Jugando)))
            .add_systems(Update, sonido_toro.run_if(in_state(GameState::Jugando)));
    }
}