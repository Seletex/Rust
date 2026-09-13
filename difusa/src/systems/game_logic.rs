use bevy::prelude::*;
use crate::components::*;
use crate::events::*;

pub fn aplicar_velocidad(
    mut q_jugador: Query<(&mut Posicion, &Velocidad, Option<&mut Transform>), With<Jugador>>,
    mut q_otros: Query<(&mut Posicion, Option<&Velocidad>, Option<&mut Transform>), Without<Jugador>>,
    obstaculos_query: Query<&Posicion, (With<Obstaculo>, Without<Jugador>)>,
) {
    if let Ok((mut pos_j, vel_j, transform_j)) = q_jugador.single_mut() {
        let radio_jugador = 16.0;
        let mitad_cuadrado = 16.0;

        let nueva_x = pos_j.x + vel_j.x;
        let nueva_y = pos_j.y + vel_j.y;

        let mut choco_x = false;
        let mut choco_y = false;

        for pos_o in &obstaculos_query {
            let cerca_x_tentativa = nueva_x.clamp(pos_o.x - mitad_cuadrado, pos_o.x + mitad_cuadrado);
            let cerca_y_actual = pos_j.y.clamp(pos_o.y - mitad_cuadrado, pos_o.y + mitad_cuadrado);
            let dist_sq_x = (nueva_x - cerca_x_tentativa).powi(2) + (pos_j.y - cerca_y_actual).powi(2);

            if dist_sq_x < radio_jugador * radio_jugador {
                choco_x = true;
            }

            let cerca_x_actual = pos_j.x.clamp(pos_o.x - mitad_cuadrado, pos_o.x + mitad_cuadrado);
            let cerca_y_tentativa = nueva_y.clamp(pos_o.y - mitad_cuadrado, pos_o.y + mitad_cuadrado);
            let dist_sq_y = (pos_j.x - cerca_x_actual).powi(2) + (nueva_y - cerca_y_tentativa).powi(2);

            if dist_sq_y < radio_jugador * radio_jugador {
                choco_y = true;
            }
        }

        if !choco_x {
            pos_j.x = nueva_x;
        }
        if !choco_y {
            pos_j.y = nueva_y;
        }

        if let Some(mut t) = transform_j {
            t.translation.x = pos_j.x;
            t.translation.y = pos_j.y;
        }
    }

    for (mut pos, vel, transform) in &mut q_otros {
        if let Some(v) = vel {
            pos.x += v.x;
            pos.y += v.y;

            if let Some(mut t) = transform {
                t.translation.x = pos.x;
                t.translation.y = pos.y;
            }
        }
    }
}

pub fn detectar_colisiones(
    jugador_query: Query<&Posicion, With<Jugador>>,
    toro_query: Query<&Posicion, (With<Toro>, Without<Jugador>)>,
    meta_query: Query<&Posicion, With<Meta>>,
    mut ev_perder: MessageWriter<EventoJuegoPerdido>,
    mut ev_ganar: MessageWriter<EventoJuegoGanado>,
) {
    if let Ok(pos_j) = jugador_query.single() {
        let dist_toro_sq = (16.0 + 22.0_f32).powi(2);
        for pos_t in &toro_query {
            let dist_sq = (pos_j.x - pos_t.x).powi(2) + (pos_j.y - pos_t.y).powi(2);
            if dist_sq < dist_toro_sq {
                ev_perder.write(EventoJuegoPerdido);
                return;
            }
        }

        let dist_meta_sq = (16.0 + 16.0_f32).powi(2);
        for pos_m in &meta_query {
            let dist_sq = (pos_j.x - pos_m.x).powi(2) + (pos_j.y - pos_m.y).powi(2);
            if dist_sq < dist_meta_sq {
                ev_ganar.write(EventoJuegoGanado);
                return;
            }
        }
    }
}

pub fn reiniciar_posiciones(
    mut q_jugador: Query<(&mut Posicion, Option<&mut Transform>), With<Jugador>>,
    mut q_toro: Query<(&mut Posicion, Option<&mut Transform>), With<Toro>>,
) {
    if let Ok((mut pos_j, t_j)) = q_jugador.single_mut() {
        pos_j.x = -250.0;
        pos_j.y = 0.0;
        if let Some(mut t) = t_j {
            t.translation.x = pos_j.x;
            t.translation.y = pos_j.y;
        }
    }

    if let Ok((mut pos_t, t_t)) = q_toro.single_mut() {
        pos_t.x = -380.0;
        pos_t.y = -180.0;
        if let Some(mut t) = t_t {
            t.translation.x = pos_t.x;
            t.translation.y = pos_t.y;
        }
    }
}

pub fn esperar_inicio_juego(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(crate::states::GameState::Jugando);
    }
}

pub fn esperar_reinicio_juego(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        next_state.set(crate::states::GameState::Inicio);
    }
}

pub fn controlar_jugador(
    keys: Res<Input<KeyCode>>,
    mut q_j: Query<&mut Velocidad, With<Jugador>>,
    mut ev_mov: MessageWriter<EventoMovimientoJugador>,
) {
    if let Ok(mut vel) = q_j.single_mut() {
        let mut moviendose = false;
        let speed = 3.5;

        let mut vx = 0.0;
        let mut vy = 0.0;
        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            vx -= speed;
        }
        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            vx += speed;
        }
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            vy += speed;
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            vy -= speed;
        }

        if vx != 0.0 || vy != 0.0 {
            moviendose = true;
        }

        vel.x = vx;
        vel.y = vy;

        ev_mov.write(EventoMovimientoJugador { moviendose });
    }
}

pub fn perseguir_jugador(
    jugador_q: Query<&Posicion, With<Jugador>>,
    mut toro_q: Query<(&mut Velocidad, &Posicion), With<Toro>>,
) {
    if let Ok(pos_j) = jugador_q.single() {
        if let Ok((mut vel_t, pos_t)) = toro_q.single_mut() {
            let dir_x = pos_j.x - pos_t.x;
            let dir_y = pos_j.y - pos_t.y;
            let len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.0001);
            let speed = 2.0;
            vel_t.x = dir_x / len * speed;
            vel_t.y = dir_y / len * speed;
        }
    }
}

pub fn validar_limites(mut q_pos: Query<(&mut Posicion, Option<&mut Transform>)>) {
    let bound_x = 400.0;
    let bound_y = 300.0;

    for (mut pos, t) in &mut q_pos {
        pos.x = pos.x.clamp(-bound_x, bound_x);
        pos.y = pos.y.clamp(-bound_y, bound_y);
        if let Some(mut tr) = t {
            tr.translation.x = pos.x;
            tr.translation.y = pos.y;
        }
    }
}

pub fn procesar_eventos_partida(
    mut ev_perder: MessageReader<EventoJuegoPerdido>,
    mut ev_ganar: MessageReader<EventoJuegoGanado>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    if ev_perder.read().next().is_some() {
        next_state.set(crate::states::GameState::Derrota);
    }
    if ev_ganar.read().next().is_some() {
        next_state.set(crate::states::GameState::Victoria);
    }
}