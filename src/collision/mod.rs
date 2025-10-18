// src/collision/mod.rs
//! Collision detection system
//!
//! This module handles all collision-related functionality:
//! - Tile types and walkability
//! - Collision map building and queries
//! - Debug visualization

pub mod map;
pub mod tile_types;

// Debug visualization - only in debug builds
#[cfg(debug_assertions)]
pub mod debug;

// Re-export commonly used types
pub use map::CollisionMap;
pub use tile_types::{TileMarker, TileType};

#[cfg(debug_assertions)]
pub use debug::{
    DebugCollisionEnabled, debug_draw_collision, debug_log_tile_info, debug_player_position,
    toggle_debug_collision,
};
