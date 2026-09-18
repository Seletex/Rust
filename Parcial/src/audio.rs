use bevy::prelude::*;
use bevy::audio::{Volume, PlaybackSettings, AudioPlayer};
use crate::components::MusicaFondo;
use crate::events::{GameState, EventoSaltoJugador, EventoColisionObstaculo, EventoToroAtrapaJugador, EventoLlegadaMeta};

#[derive(Resource)]
#[allow(dead_code)]
pub struct ColeccionAudioProcedural {
    pub menu: Handle<AudioSource>,
    pub juego: Handle<AudioSource>,
    pub victoria: Handle<AudioSource>,
    pub derrota: Handle<AudioSource>,
    pub pasos: Handle<AudioSource>,
}

fn detener_musica_anterior(commands: &mut Commands, query: &Query<Entity, With<MusicaFondo>>) {
    for entidad in query.iter() {
        commands.entity(entidad).despawn();
    }
}

pub fn cargar_audios(mut commands: Commands, asset_server: Res<AssetServer>) {
    let coleccion = ColeccionAudioProcedural {
        menu: asset_server.load("audio/menu.mp3"),
        juego: asset_server.load("audio/juego.mp3"),
        victoria: asset_server.load("audio/victoria.mp3"),
        derrota: asset_server.load("audio/derrota.mp3"),
        pasos: asset_server.load("audio/pasos.mp3"),
    };

    commands.insert_resource(coleccion);
    // Reproducir música de menú inmediatamente después de cargar
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/menu.mp3")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.5)),
        MusicaFondo,
    ));
}

fn reproducir_musica_juego(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        info!("🎵 Iniciando música de JUEGO");
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.juego.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.6)),
            MusicaFondo,
        ));
    } else {
        warn!("⚠️ ColeccionAudioProcedural no disponible para música de juego");
    }
}

fn reproducir_audio_victoria(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        info!("🎵 Iniciando música de VICTORIA");
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.victoria.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.8)),
        ));
    } else {
        warn!("⚠️ ColeccionAudioProcedural no disponible para música de victoria");
    }
}

fn reproducir_audio_derrota(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        info!("🎵 Iniciando música de DERROTA");
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.derrota.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.8)),
        ));
    } else {
        warn!("⚠️ ColeccionAudioProcedural no disponible para música de derrota");
    }
}

// --- Sistemas de reacción a eventos (sonidos FX) ---

fn sonido_salto(
    mut commands: Commands,
    mut ev_salto: MessageReader<EventoSaltoJugador>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_salto.read() {
        info!("🔊 EventoSaltoJugador recibido - reproduciendo sonido de salto");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.pasos.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.8)),
            ));
        } else {
            warn!("⚠️ ColeccionAudioProcedural no disponible para sonido de salto");
        }
    }
}

fn sonido_colision(
    mut commands: Commands,
    mut ev_colision: MessageReader<EventoColisionObstaculo>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_colision.read() {
        info!("🔊 EventoColisionObstaculo recibido - reproduciendo sonido de colisión");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.derrota.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.8)),
            ));
        } else {
            warn!("⚠️ ColeccionAudioProcedural no disponible para sonido de colisión");
        }
    }
}

fn sonido_toro(
    mut commands: Commands,
    mut ev_toro: MessageReader<EventoToroAtrapaJugador>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_toro.read() {
        info!("🔊 EventoToroAtrapaJugador recibido - reproduciendo sonido de toro");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.derrota.clone()),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.8)),
            ));
        } else {
            warn!("⚠️ ColeccionAudioProcedural no disponible para sonido de toro");
        }
    }
}

fn sonido_victoria(
    mut commands: Commands,
    mut ev_victoria: MessageReader<EventoLlegadaMeta>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    for _ in ev_victoria.read() {
        info!("🔊 EventoLlegadaMeta recibido - reproduciendo sonido de victoria");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.victoria.clone()),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(0.8)),
            ));
        } else {
            warn!("⚠️ ColeccionAudioProcedural no disponible para sonido de victoria");
        }
    }
}

fn sonido_derrota(
    mut commands: Commands,
    mut ev_derrota: MessageReader<EventoToroAtrapaJugador>,
    mut ev_colision: MessageReader<EventoColisionObstaculo>,
    audio: Option<Res<ColeccionAudioProcedural>>,
) {
    let mut play = false;
    for _ in ev_derrota.read() {
        play = true;
    }
    for _ in ev_colision.read() {
        play = true;
    }
    if play {
        info!("🔊 GameOver - reproduciendo sonido de derrota");
        if let Some(audios) = audio.as_ref() {
            commands.spawn((
                AudioPlayer::new(audios.derrota.clone()),
                PlaybackSettings::ONCE.with_volume(Volume::Linear(0.8)),
            ));
        } else {
            warn!("⚠️ ColeccionAudioProcedural no disponible para sonido de derrota");
        }
    }
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cargar_audios)
            .add_systems(OnEnter(GameState::Jugando), reproducir_musica_juego)
            .add_systems(OnEnter(GameState::Victoria), reproducir_audio_victoria)
            .add_systems(OnEnter(GameState::GameOver), reproducir_audio_derrota)
            // Sistemas de reacción a eventos (sonidos FX)
            .add_systems(Update, sonido_salto.run_if(in_state(GameState::Jugando)))
            .add_systems(Update, sonido_colision.run_if(in_state(GameState::Jugando)))
            .add_systems(Update, sonido_toro.run_if(in_state(GameState::Jugando)))
            .add_systems(Update, sonido_victoria.run_if(in_state(GameState::Victoria)))
            .add_systems(Update, sonido_derrota.run_if(in_state(GameState::GameOver)));
    }
}