use bevy::audio::Volume;
use bevy::prelude::*;

use crate::components::*;
use crate::events::*;

fn detener_musica_anterior(
    commands: &mut Commands,
    query: &Query<Entity, With<MusicaFondo>>,
) {
    for entidad in query.iter() {
        commands.entity(entidad).despawn();
    }
}

pub fn reproducir_musica_inicio(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        detener_musica_anterior(&mut commands, &musica_query);

        commands.spawn((
            AudioPlayer::new(audios.menu.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.4)),
            MusicaFondo,
        ));
    }
}

pub fn reproducir_musica_juego(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        detener_musica_anterior(&mut commands, &musica_query);

        commands.spawn((
            AudioPlayer::new(audios.juego.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(0.5)),
            MusicaFondo,
        ));
    }
}

pub fn reproducir_audio_victoria(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        detener_musica_anterior(&mut commands, &musica_query);

        commands.spawn((
            AudioPlayer::new(audios.victoria.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.7)),
            EfectoSonido,
        ));
    }
}

pub fn reproducir_audio_derrota(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        detener_musica_anterior(&mut commands, &musica_query);

        commands.spawn((
            AudioPlayer::new(audios.derrota.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.7)),
            EfectoSonido,
        ));
    }
}

pub fn reproducir_sonido_pasos(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    mut ev_movimiento: MessageReader<EventoMovimientoJugador>,
) {
    if let Some(audios) = audios {
        for evento in ev_movimiento.read() {
            if evento.moviendose {
                commands.spawn((
                    AudioPlayer::new(audios.pasos.clone()),
                    PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.1)),
                ));
            }
        }
    }
}