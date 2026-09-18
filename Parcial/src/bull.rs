use bevy::prelude::*;

pub struct BullPlugin;

impl Plugin for BullPlugin {
    fn build(&self, _app: &mut App) {
        // All bull logic moved to world.rs detectar_colisiones to avoid query conflicts
    }
}