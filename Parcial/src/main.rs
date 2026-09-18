// ============================================================================
// MAIN - Punto de entrada del juego Extreme Pamplona
// ============================================================================
// Configura la aplicación Bevy, registra plugins, estados y mensajes,
// y ejecuta el bucle principal del juego.

mod audio;
mod bull;
mod components;
mod events;
mod game;
mod player;
mod resources;
mod ui;
mod world;

use bevy::prelude::*;
use crate::events::*;
use crate::game::GameplayPlugin;
use crate::resources::ConfigJuego;

fn main() {
    App::new()
        // Color de fondo de la ventana (gris oscuro)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        
        // Inicializa configuración del juego (velocidades, gravedad, límites, etc.)
        .init_resource::<ConfigJuego>()
        
        // Configura plugins por defecto de Bevy + ventana personalizada
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Extreme Pamplona - Bevy ECS".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        
        // Plugin principal que agrupa toda la lógica del juego
        .add_plugins(GameplayPlugin)
        
        // Inicializa máquina de estados del juego
        .init_state::<GameState>()
        
        // Registra mensajes (colas de eventos) para comunicación entre sistemas
        .add_message::<EventoSaltoJugador>()
        .add_message::<EventoColisionObstaculo>()
        .add_message::<EventoToroAtrapaJugador>()
        .add_message::<EventoJuegoIniciado>()
        .add_message::<EventoJuegoReiniciado>()
        .add_message::<EventoLlegadaMeta>()
        
        // Inicia el bucle principal (update + render)
        .run();
}