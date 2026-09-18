// ============================================================================
// EVENTOS - Definición de estados del juego y mensajes para comunicación
// ============================================================================
// Los Messages son colas FIFO que permiten comunicación desacoplada entre sistemas:
// un sistema escribe (MessageWriter) y otro lee (MessageReader) en el siguiente frame.

use bevy::prelude::*;

// --- Estados del juego (Machine de Estados) ---
// Controla qué sistemas se ejecutan mediante .run_if(in_state(...))
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,       // Pantalla inicial - espera Space/Enter
    Jugando,    // Partida activa - sistemas de gameplay
    GameOver,   // Derrota - pantalla de reinicio
    Victoria,   // Victoria - pantalla de reinicio
}

// --- Mensajes de Gameplay (Eventos) ---
// Se usan para comunicación entre sistemas sin acoplamiento directo

/// Emitido cuando el jugador salta (Space en suelo)
/// Audio: reproduce FX salto
#[derive(Message)]
pub struct EventoSaltoJugador;

/// Emitido al chocar con obstáculo (lados o cabeza)
/// Audio: reproduce FX colisión
/// World: resuelve posición + vel.x=0
#[derive(Message)]
pub struct EventoColisionObstaculo {
    pub entidad_obstaculo: Entity,  // Qué obstáculo se chocó
}

/// Emitido cuando el toro atrapa al jugador (distancia < umbral)
/// Audio: FX toro + música derrota
/// State: GameOver
#[derive(Message)]
pub struct EventoToroAtrapaJugador;

/// Emitido al presionar Space/Enter en Menu
/// State: Menu → Jugando
/// Audio: cambia a música juego
#[derive(Message)]
pub struct EventoJuegoIniciado;

/// Emitido al presionar Space/Enter en GameOver/Victoria
/// State: GameOver/Victoria → Jugando
/// Audio: cambia a música juego
#[derive(Message)]
pub struct EventoJuegoReiniciado;

/// Emitido al llegar a la meta (distancia < umbral)
/// Audio: música victoria
/// State: Victoria
#[derive(Message)]
pub struct EventoLlegadaMeta;