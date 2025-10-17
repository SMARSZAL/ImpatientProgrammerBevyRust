// src/map/tile_marker.rs
use bevy::prelude::*;

/// Marker component attached to tile entities during spawn
/// to track what type of terrain/prop they represent.
#[derive(Component, Clone, Copy, Debug)]
pub struct TileTypeMarker {
    pub tile_type: TileType,
    /// Grid position where this tile was placed (x, y, z layer)
    pub grid_position: IVec3,
}

/// Represents the type of a tile for collision/interaction purposes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    // Terrain types (walkable)
    Dirt,
    Grass,
    YellowGrass,
    
    // Water (NOT walkable)
    Water,
    Shore,        // NEW: Walkable edge of water
    
    // Props (NOT walkable)
    Tree,
    Rock,
    Plant,
    Stump,
    
    // Empty/void space (walkable - represents areas with no tile)
    Empty,
}

impl TileType {
    /// Returns true if the player can walk on this tile type
    pub fn is_walkable(&self) -> bool {
        !matches!(self, TileType::Water | TileType::Tree | TileType::Rock)
    }
    
    /// Returns friction multiplier (for future use)
    /// 1.0 = normal speed, < 1.0 = slower
    pub fn friction(&self) -> f32 {
        match self {
            TileType::Dirt => 1.0,
            TileType::Grass => 0.85,
            TileType::YellowGrass => 0.7,
            _ => 1.0,
        }
    }
}

