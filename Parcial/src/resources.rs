// ============================================================================
// RECURSOS - Datos globales accesibles desde cualquier System
// ============================================================================
// Los Resources son singletons globales (uno por app). Se acceden con Res/ResMut.
// Diferencia con Components: no están ligados a una Entity específica.

use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;

/// Configuración central del gameplay (velocidades, física, límites)
/// Se usa .init_resource::<ConfigJuego>() en main() para valores por defecto
/// Cualquier system puede leer/modificar con Res<ConfigJuego> / ResMut<ConfigJuego>
#[derive(Resource)]
pub struct ConfigJuego {
    pub velocidad_toro: f32,      // Pixeles/seg - velocidad persecución toro
    pub velocidad_jugador: f32,   // Pixeles/seg - movimiento horizontal jugador
    pub fuerza_salto: f32,        // Impulso inicial Y al saltar
    pub gravedad: f32,            // Aceleración Y negativa (px/s²)
    pub umbral_colision: f32,     // Radio colisión circular toro/meta (px)
    pub limite_x: f32,            // Límite horizontal mundo (± desde centro)
    pub limite_y: f32,            // Límite vertical mundo (± desde centro)
}

impl Default for ConfigJuego {
    fn default() -> Self {
        Self {
            velocidad_toro: 100.0,      // Toro más lento (era 180.0)
            velocidad_jugador: 300.0,
            fuerza_salto: 400.0,
            gravedad: 800.0,
            umbral_colision: 55.0,
            limite_x: 380.0,
            limite_y: 280.0,
        }
    }
}

/// Puntuación (no usada aún, preparada para futuro)
#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct Puntuacion {
    pub valor: u32,
    pub mejor: u32,
}

/// Handles a materiales de color (para sprites procedimentales)
/// Se crean en Startup y se clonan al spawnear entidades
#[derive(Resource)]
#[allow(dead_code)]
pub struct SpriteHandles {
    pub jugador: Handle<ColorMaterial>,
    pub toro: Handle<ColorMaterial>,
    pub obstaculo: Handle<ColorMaterial>,
    pub meta: Handle<ColorMaterial>,
    pub fondo: Handle<ColorMaterial>,
}

/// Handles a mallas (geometría)
/// Mesh cuadrado 64x64 para entidades, rectángulo 800x600 para fondo
#[derive(Resource)]
#[allow(dead_code)]
pub struct MeshHandles {
    pub cuadrado: Handle<Mesh>,
    pub fondo: Handle<Mesh>,
}