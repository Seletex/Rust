mod audio;
mod components;
mod events;
mod game;
mod player;


use bevy::prelude::*;
use events::*;
use game::GameplayPlugin;
use player::PlayerPlugin;
use crate::audio::AudioPlugin; // Importa el plugin de audio
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        
        .add_plugins( GameplayPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Juego Bevy - Audio Procedural Integrado".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_message::<EventoMovimientoJugador>()
        .add_message::<EventoJuegoPerdido>()
        .add_message::<EventoJuegoGanado>()
        
        .run();
}