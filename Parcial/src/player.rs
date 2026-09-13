use bevy::prelude::*;
use crate::components::{Jugador, Posicion, Toro, Velocidad};
use crate::events::{EventoMovimientoJugador, GameState}; // Ajusta si EventoMovimientoJugador está en events.rs o main.rspub struct PlayerPlugin;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mover_jugador,
                hacer_saltar_jugador,
            ).run_if(in_state(GameState::Jugando)) // Ajusta 'Juego' al nombre real de tu variante en GameState
        );
    }
}

fn mover_jugador(
    teclado: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Posicion, &mut Velocidad), With<Jugador>>,
    time: Res<Time>,
) {
    if let Ok((mut pos, _vel)) = query.single_mut() {
        let mut direccion = 0.0;
        if teclado.pressed(KeyCode::KeyD) || teclado.pressed(KeyCode::ArrowRight) {
            direccion += 1.0;
        }
        if teclado.pressed(KeyCode::KeyA) || teclado.pressed(KeyCode::ArrowLeft) {
            direccion -= 1.0;
        }
        pos.x += direccion * 200.0 * time.delta_secs();
    }
}

fn hacer_saltar_jugador(
    teclado: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Posicion, With<Jugador>>,
) {
    if teclado.just_pressed(KeyCode::Space) {
        if let Ok(mut pos) = query.single_mut() {
            pos.y += 50.0;
        }
    }
}

pub fn controlar_jugador(
    teclado: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocidad, With<Jugador>>,
    mut ev_movimiento: MessageWriter<EventoMovimientoJugador>,
) {
    if let Ok(mut vel) = query.single_mut() {
        let mut dir_x = 0.0;
        let mut dir_y = 0.0;

        if teclado.pressed(KeyCode::KeyW) || teclado.pressed(KeyCode::ArrowUp) {
            dir_y += 1.0;
        }
        if teclado.pressed(KeyCode::KeyS) || teclado.pressed(KeyCode::ArrowDown) {
            dir_y -= 1.0;
        }
        if teclado.pressed(KeyCode::KeyA) || teclado.pressed(KeyCode::ArrowLeft) {
            dir_x -= 1.0;
        }
        if teclado.pressed(KeyCode::KeyD) || teclado.pressed(KeyCode::ArrowRight) {
            dir_x += 1.0;
        }

        let rapido = 3.0;
        vel.x = dir_x * rapido;
        vel.y = dir_y * rapido;

        let se_mueve = dir_x != 0.0 || dir_y != 0.0;
        ev_movimiento.write(EventoMovimientoJugador {
            moviendose: se_mueve,
        });
    }
}

pub fn perseguir_jugador(
    pos_jugador_query: Query<&Posicion, (With<Jugador>, Without<Toro>)>,
    mut toro_query: Query<(&Posicion, &mut Velocidad), With<Toro>>,
) {
    if let Ok(pos_jugador) = pos_jugador_query.single() {
        for (pos_toro, mut vel_toro) in &mut toro_query {
            let dx = pos_jugador.x - pos_toro.x;
            let dy = pos_jugador.y - pos_toro.y;
            let distancia = (dx * dx + dy * dy).sqrt();

            if distancia > 0.0 {
                let velocidad_toro = 1.2;
                vel_toro.x = (dx / distancia) * velocidad_toro;
                vel_toro.y = (dy / distancia) * velocidad_toro;
            }
        }
    }
}

pub fn aplicar_velocidad(mut query: Query<(&mut Posicion, &Velocidad, Option<&mut Transform>)>) {
    for (mut pos, vel, transform) in &mut query {
        pos.x += vel.x;
        pos.y += vel.y;

        if let Some(mut t) = transform {
            t.translation.x = pos.x;
            t.translation.y = pos.y;
        }
    }
}

pub fn validar_limites(mut query: Query<&mut Posicion, With<Jugador>>) {
    for mut pos in &mut query {
        pos.x = pos.x.clamp(-380.0, 380.0);
        pos.y = pos.y.clamp(-280.0, 280.0);
    }
}