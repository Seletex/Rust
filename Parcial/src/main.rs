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
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)))
        .init_resource::<ConfigJuego>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Extreme Pamplona - Bevy ECS".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GameplayPlugin)
        .init_state::<GameState>()
        .add_message::<EventoSaltoJugador>()
        .add_message::<EventoColisionObstaculo>()
        .add_message::<EventoToroAtrapaJugador>()
        .add_message::<EventoJuegoIniciado>()
        .add_message::<EventoJuegoReiniciado>()
        .add_message::<EventoLlegadaMeta>()
        .run();
}