use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        // All player movement moved to world.rs detectar_colisiones to avoid query conflicts
    }
}