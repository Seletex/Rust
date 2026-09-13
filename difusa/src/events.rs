use bevy::prelude::*;

#[derive(Message)]
pub struct EventoMovimientoJugador {
    pub moviendose: bool,
}

#[derive(Message)]
pub struct EventoJuegoPerdido;

#[derive(Message)]
pub struct EventoJuegoGanado;