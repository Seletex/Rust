use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Inicio,
    Jugando,
    Victoria,
    Derrota,
}

#[derive(Message)]
pub struct EventoMovimientoJugador {
    pub moviendose: bool,
}

#[derive(Message)]
pub struct EventoJuegoPerdido;

#[derive(Message)]
pub struct EventoJuegoGanado;