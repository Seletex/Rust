use bevy::prelude::*;

#[derive(Component)]
pub struct Posicion {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocidad {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Jugador;

#[derive(Component)]
pub struct Toro;

#[derive(Component)]
pub struct Obstaculo;

#[derive(Component)]
pub struct Meta;

#[derive(Component)]
pub struct MusicaFondo;