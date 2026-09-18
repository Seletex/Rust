use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;

#[derive(Resource)]
pub struct ConfigJuego {
    pub velocidad_toro: f32,
    pub velocidad_jugador: f32,
    pub fuerza_salto: f32,
    pub gravedad: f32,
    pub umbral_colision: f32,
    pub limite_x: f32,
    pub limite_y: f32,
}

impl Default for ConfigJuego {
    fn default() -> Self {
        Self {
            velocidad_toro: 180.0,
            velocidad_jugador: 300.0,
            fuerza_salto: 400.0,
            gravedad: 800.0,
            umbral_colision: 55.0,
            limite_x: 380.0,
            limite_y: 280.0,
        }
    }
}

#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct Puntuacion {
    pub valor: u32,
    pub mejor: u32,
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct SpriteHandles {
    pub jugador: Handle<ColorMaterial>,
    pub toro: Handle<ColorMaterial>,
    pub obstaculo: Handle<ColorMaterial>,
    pub meta: Handle<ColorMaterial>,
    pub fondo: Handle<ColorMaterial>,
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct MeshHandles {
    pub cuadrado: Handle<Mesh>,
    pub fondo: Handle<Mesh>,
}