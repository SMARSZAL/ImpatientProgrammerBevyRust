// src/combat/mod.rs
mod player_combat;
mod power_type;
mod systems;

pub use player_combat::PlayerCombat;
pub use power_type::{PowerType, PowerVisuals};
pub use systems::{ProjectileEffect, debug_switch_power, handle_power_input};

use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_power_input, debug_switch_power));
    }
}