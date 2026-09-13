use bevy::audio::Volume;
use bevy::prelude::*;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::io::Cursor;
#[derive(Resource)]
pub struct ColeccionAudio {
    pub menu: Handle<AudioSource>,
    pub juego: Handle<AudioSource>,
    pub victoria: Handle<AudioSource>,
    pub derrota: Handle<AudioSource>,
    pub pasos: Handle<AudioSource>,
}
use crate::components::MusicaFondo;
use crate::events::{EventoMovimientoJugador, GameState};

#[derive(Resource)]
pub struct ColeccionAudioProcedural {
    pub menu: Handle<AudioSource>,
    pub juego: Handle<AudioSource>,
    pub victoria: Handle<AudioSource>,
    pub derrota: Handle<AudioSource>,
    pub pasos: Handle<AudioSource>,
}



pub fn crear_wav_en_memoria(generar_muestra: impl Fn(f32) -> f32, duracion_secs: f32) -> Vec<u8> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut cursor, spec).expect("Error creando WavWriter");
        let num_samples = (spec.sample_rate as f32 * duracion_secs) as usize;

        for i in 0..num_samples {
            let t = i as f32 / spec.sample_rate as f32;
            let sample_f32 = generar_muestra(t).clamp(-1.0, 1.0);
            let sample_i16 = (sample_f32 * 32767.0) as i16;
            writer.write_sample(sample_i16).unwrap();
        }
        writer.finalize().expect("Error al finalizar WAV");
    }

    cursor.into_inner()
}

fn detener_musica_anterior(commands: &mut Commands, query: &Query<Entity, With<MusicaFondo>>) {
    for entidad in query.iter() {
        commands.entity(entidad).despawn();
    }
}

fn reproducir_musica_inicio(
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
pub fn cargar_audios(mut commands: Commands, asset_server: Res<AssetServer>) {
    let coleccion = ColeccionAudio {
        menu: asset_server.load("audio/menu.mp3"),
        juego: asset_server.load("audio/juego.mp3"),
        victoria: asset_server.load("audio/victoria.mp3"),
        derrota: asset_server.load("audio/derrota.mp3"),
        pasos: asset_server.load("audio/pasos.mp3"),
    };

    commands.insert_resource(coleccion);
}

fn reproducir_musica_juego(
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

fn reproducir_audio_victoria(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.victoria.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.7)),
            MusicaFondo,
        ));
    }
}

fn reproducir_audio_derrota(
    mut commands: Commands,
    audios: Option<Res<ColeccionAudioProcedural>>,
    musica_query: Query<Entity, With<MusicaFondo>>,
) {
    if let Some(audios) = audios {
        detener_musica_anterior(&mut commands, &musica_query);
        commands.spawn((
            AudioPlayer::new(audios.derrota.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(0.7)),
            MusicaFondo,
        ));
    }
}

fn reproducir_sonido_pasos(
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
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Inicio), reproducir_musica_inicio)
            .add_systems(OnEnter(GameState::Jugando), reproducir_musica_juego)
            .add_systems(OnEnter(GameState::Victoria), reproducir_audio_victoria)
            .add_systems(OnEnter(GameState::Derrota), reproducir_audio_derrota)
            .add_systems(
                Update,
                reproducir_sonido_pasos.run_if(in_state(GameState::Jugando)),
            );
    }
}