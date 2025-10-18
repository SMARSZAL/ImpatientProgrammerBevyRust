// src/collision/tile_types.rs
use bevy::prelude::*;

/// Component to mark entities with their collision type
#[derive(Component, Debug, Clone)]
pub struct TileMarker {
    pub tile_type: TileType,
}

/// Tile types for collision detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    // Walkable terrain
    Dirt,
    Grass,
    YellowGrass,
    Shore, // Water edges
    Empty, // No tile

    // Non-walkable obstacles
    Water,
    Tree,
    Rock,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        !matches!(self, TileType::Water | TileType::Tree | TileType::Rock)
    }
}
