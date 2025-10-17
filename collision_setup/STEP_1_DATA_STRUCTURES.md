# Step 1: Create Data Structures

## 🎯 Goal
Create the basic data structures for collision detection: `TileType` enum and `Map` resource.

## 📦 What We're Building

Two new files with simple, clean structures:
1. A marker component to tag tiles
2. An enum to represent tile types
3. A Map resource to query walkability

## 📝 Implementation

### 1.1 Create `src/map/tile_marker.rs`

**Purpose**: This component gets attached to tile entities during spawn to remember what type they are.

```rust
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
        match self {
            // Water blocks movement
            TileType::Water => false,
            
            // Large props block movement
            TileType::Tree | TileType::Rock => false,
            
            // Everything else is walkable for now
            TileType::Dirt 
            | TileType::Grass 
            | TileType::YellowGrass
            | TileType::Plant      // Small plants don't block
            | TileType::Stump      // Stumps don't block
            | TileType::Empty => true,
        }
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
```

### 1.2 Create `src/map/collision.rs`

**Purpose**: The Map resource that the player movement system will query.

```rust
// src/map/collision.rs
use bevy::prelude::*;
use super::tile_marker::TileType;

/// Collision map resource that stores walkability information
/// for the entire game map in a simple 2D grid.
#[derive(Resource)]
pub struct Map {
    /// Flat array of tile types, row-major order (like the tutorial!)
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub tile_size: f32,  // 32.0 pixels per tile
}

impl Map {
    /// Create a new empty map filled with Empty tiles
    pub fn new(width: i32, height: i32, tile_size: f32) -> Self {
        let size = (width * height) as usize;
        Self {
            tiles: vec![TileType::Empty; size],
            width,
            height,
            tile_size,
        }
    }
    
    /// Convert 2D grid coordinates to 1D array index
    /// Same pattern as the roguelike tutorial!
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
    
    /// Check if grid coordinates are within bounds
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    
    /// Check if a grid position is walkable
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false; // Out of bounds = not walkable
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx].is_walkable()
    }
    
    /// Set tile type at grid position
    pub fn set_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if !self.in_bounds(x, y) {
            return;
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile_type;
    }
    
    /// Convert world position (in pixels) to grid coordinates
    /// This accounts for the centered grid from generate.rs
    pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        // The map is centered at origin, so we need to offset
        // Grid origin is at (-width * tile_size / 2, -height * tile_size / 2)
        let grid_origin_x = -(self.width as f32 * self.tile_size) / 2.0;
        let grid_origin_y = -(self.height as f32 * self.tile_size) / 2.0;
        
        let relative_x = world_pos.x - grid_origin_x;
        let relative_y = world_pos.y - grid_origin_y;
        
        IVec2::new(
            (relative_x / self.tile_size).floor() as i32,
            (relative_y / self.tile_size).floor() as i32,
        )
    }
    
    /// Check if a world position is walkable
    pub fn is_world_pos_walkable(&self, world_pos: Vec2) -> bool {
        let grid_pos = self.world_to_grid(world_pos);
        self.is_walkable(grid_pos.x, grid_pos.y)
    }
}
```

### 1.3 Update `src/map/mod.rs`

**Purpose**: Export the new modules so they're accessible.

```rust
// src/map/mod.rs
pub mod assets;
pub mod generate;
pub mod models;
pub mod rules;
pub mod sockets;
pub mod tilemap;
pub mod tile_marker;  // NEW!
pub mod collision;    // NEW!

// Re-export commonly used types
pub use tile_marker::{TileType, TileTypeMarker};
pub use collision::Map;
```

## ✅ Verification

### Step 1: Compile Check

```bash
cargo build
```

**Expected**: Should compile with no errors. You might get warnings about unused code - that's fine!

### Step 2: Check Module Structure

```bash
# List the new files
ls -la src/map/tile_marker.rs
ls -la src/map/collision.rs
```

**Expected**: Both files exist

### Step 3: Quick Sanity Test

Add this temporary test to the bottom of `src/map/collision.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map_creation() {
        let map = Map::new(25, 18, 32.0);
        assert_eq!(map.tiles.len(), 25 * 18);
        assert!(map.is_walkable(0, 0)); // Empty is walkable
    }
    
    #[test]
    fn test_tile_walkability() {
        assert!(TileType::Dirt.is_walkable());
        assert!(TileType::Grass.is_walkable());
        assert!(!TileType::Water.is_walkable());
        assert!(!TileType::Tree.is_walkable());
    }
    
    #[test]
    fn test_world_to_grid() {
        let map = Map::new(25, 18, 32.0);
        // Map is centered, so (0, 0) world = (12, 9) grid approximately
        let grid_pos = map.world_to_grid(Vec2::ZERO);
        println!("Center world position maps to grid: {:?}", grid_pos);
        assert!(map.in_bounds(grid_pos.x, grid_pos.y));
    }
}
```

Then run:

```bash
cargo test map::collision::tests
```

**Expected**: All 3 tests pass!

## 🎓 What We Learned

- ✅ Created a simple `TileType` enum with walkability logic
- ✅ Created a `Map` resource with grid-to-world coordinate conversion
- ✅ Pattern matches roguelike tutorial (simple Vec, not complex HashMap)
- ✅ World-to-grid conversion accounts for centered map
- ✅ All functions are documented and straightforward

## 🐛 Common Issues

**Issue**: "cannot find type `IVec2` in this scope"
- **Fix**: Add `use bevy::prelude::*;` at the top of the file

**Issue**: "unresolved import `super::tile_marker`"
- **Fix**: Make sure you created `tile_marker.rs` and updated `mod.rs`

**Issue**: Tests fail with "out of bounds"
- **Fix**: Check grid dimensions (should be 25x18) and tile_size (32.0)

## ➡️ Next Step

Once tests pass and everything compiles:

**👉 Continue to [STEP_2_TAG_TILES.md](STEP_2_TAG_TILES.md)**

This is where we tag tiles during spawn using `components_spawner`!

