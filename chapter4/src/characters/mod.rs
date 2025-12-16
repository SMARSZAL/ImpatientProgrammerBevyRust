pub mod animation;
pub mod config;
pub mod spawn;
pub mod state; 
pub mod facing;  
pub mod input; 
pub mod physics;  
pub mod collider;
mod rendering;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use config::CharactersList;
use crate::state::GameState;


pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .add_systems(Startup, spawn::spawn_player)
            .add_systems(Update, (
                input::handle_player_input,
                spawn::switch_character,
                input::update_jump_state,
                animation::on_state_change_update_animation,
                collider::validate_movement,
                physics::apply_velocity,
                rendering::update_player_depth,
                animation::animations_playback,
            ).chain().run_if(in_state(GameState::Playing)));
    }
}

