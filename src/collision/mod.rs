// src/collision/mod.rs
//! Collision detection system
//! 
//! This module handles all collision-related functionality:
//! - Tile types and walkability
//! - Collision map building and queries
//! - Debug visualization

pub mod tile_types;
pub mod map;

// Debug visualization - only in debug builds
#[cfg(debug_assertions)]
pub mod debug;

// Re-export commonly used types
pub use tile_types::{TileType, TileMarker};
pub use map::CollisionMap;

#[cfg(debug_assertions)]
pub use debug::{DebugCollisionEnabled, toggle_debug_collision, debug_draw_collision, debug_player_position, debug_log_tile_info};
