// ============================================================================
// PLAYER - Plugin vacío (lógica movida a world.rs por query conflicts)
// ============================================================================
// Originalmente tenía sistemas separados (mover, gravedad, salto, aplicar_velocidad)
// pero causaban conflictos B0001 al acceder mutably a mismos componentes
// Ahora TODA la lógica de jugador está en world.rs::detectar_colisiones
// Este plugin existe como placeholder por arquitectura modular

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        // Toda la lógica de movimiento/salto/colisiones está en world.rs::detectar_colisiones
        // para evitar conflictos de queries mutables (error B0001)
    }
}