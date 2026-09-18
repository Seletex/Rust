use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Jugando,
    GameOver,
    Victoria,
}

#[derive(Message)]
pub struct EventoSaltoJugador;

#[derive(Message)]
#[allow(dead_code)]
pub struct EventoColisionObstaculo {
    pub entidad_obstaculo: Entity,
}

#[derive(Message)]
pub struct EventoToroAtrapaJugador;

#[derive(Message)]
pub struct EventoJuegoIniciado;

#[derive(Message)]
pub struct EventoJuegoReiniciado;

#[derive(Message)]
pub struct EventoLlegadaMeta;