// ============================================================================
// COMPONENTES ECS - Datos puros que se adjuntan a las Entidades
// ============================================================================
// En ECS, los Componentes son SOLO datos (structs). La lógica vive en Systems.
// Bevy usa Queries para acceder a componentes de forma eficiente y paralela.

use bevy::prelude::*;

/// Posición en coordenadas de mundo (píxeles, centro de pantalla = 0,0)
/// Se sincroniza con Transform.translation cada frame
#[derive(Component)]
pub struct Posicion {
    pub x: f32,
    pub y: f32,
}

/// Velocidad en píxeles/segundo (independiente de frame rate)
/// Se integra cada frame: pos += vel * delta_time
#[derive(Component)]
pub struct Velocidad {
    pub x: f32,
    pub y: f32,
}

/// Marcador: Entidad controlada por el jugador
/// Usado en Queries con With<Jugador> para filtrar
#[derive(Component)]
pub struct Jugador;

/// Marcador: Entidad del toro que persigue al jugador
#[derive(Component)]
pub struct Toro;

/// Marcador: Obstáculo sólido (plataforma/pared)
/// Colisión AABB: push horizontal, aterrizar arriba, golpear cabeza
#[derive(Component)]
pub struct Obstaculo;

/// Marcador: Zona de victoria (meta/refugio)
/// Colisión circular simple (distancia < umbral)
#[derive(Component)]
pub struct Meta;

/// Marcador: Entidad que reproduce música de fondo
/// Usada para limpiar música anterior al cambiar estados
#[derive(Component)]
pub struct MusicaFondo;

/// Estado: ¿El jugador está tocando suelo?
/// Bool en tupla para mutabilidad fácil: en_suelo.0 = true/false
/// Se usa para permitir salto solo en suelo
#[derive(Component)]
pub struct EnSuelo(pub bool);

/// Tamaño visual del sprite (para debugging futuro)
#[derive(Component)]
#[allow(dead_code)]
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}