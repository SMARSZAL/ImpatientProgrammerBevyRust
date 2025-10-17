# Step 3: Build the Collision Map

## 🎯 Goal
Create a system that runs after tiles spawn, queries all `TileTypeMarker` components, and builds the `Map` resource.

## 💡 Strategy

The WFC generation happens in the background. We need to detect when it's done and tiles have spawned, then build our collision map.

We'll use a **state-based approach**:
1. Track if collision map is built yet
2. Every frame, check if tiles exist and map isn't built yet
3. When tiles exist, build the map once
4. Set a flag so we don't rebuild unnecessarily

## 🏔️ Understanding the Layer Problem

**Critical Concept**: Your map has **5 z-layers** stacked vertically:

```
Layer 4 (z≈4): Props (trees, rocks, plants)
Layer 3 (z≈3): Water
Layer 2 (z≈2): Yellow grass  
Layer 1 (z≈1): Green grass
Layer 0 (z≈0): Dirt (base)
```

At any given (x,y) grid position, you might have **multiple tiles**:
- Position (10, 5): Dirt (z=0) + Grass (z=1) + Water (z=3)
- Position (8, 12): Dirt (z=0) + Grass (z=1) + Tree (z=4)

**The Problem**: Which tile should determine walkability?

**The Solution**: **Top layer wins!** 
- The highest z-value tile is what the player sees
- That's what they should collide with
- Water over grass = can't walk (water blocks)
- Tree over grass = can't walk (tree blocks)
- Just grass = can walk

This is the standard approach for 2D layered tilemaps.

## 📝 Implementation

### 3.1 Add collision system to `src/map/generate.rs`

Add this at the end of the file:

```rust
// At the top of the file, add new imports:
use crate::map::{Map, TileTypeMarker, TileType};
use std::collections::HashMap;

// At the end of the file, add these systems:

/// Resource to track if we've built the collision map yet
#[derive(Resource, Default)]
pub struct CollisionMapBuilt(pub bool);

/// System that builds the collision map from spawned tiles
/// Runs once after WFC generation completes and tiles are spawned
/// 
/// IMPORTANT: This handles the multi-layer problem!
/// Your map has 5 z-layers (dirt, grass, yellow grass, water, props).
/// At each (x,y) position, we only keep the TOPMOST layer for collision.
/// Example: If there's dirt (z=0), grass (z=1), and water (z=3) at position (10,5),
/// we only mark it as Water (the highest/visible layer).
pub fn build_collision_map(
    mut commands: Commands,
    mut built: ResMut<CollisionMapBuilt>,
    tile_query: Query<(&TileTypeMarker, &Transform)>,
) {
    // Skip if already built
    if built.0 {
        return;
    }
    
    // Check if we have any tiles yet
    let tile_count = tile_query.iter().count();
    if tile_count == 0 {
        // WFC hasn't generated tiles yet, wait
        return;
    }
    
    info!("Building collision map from {} tiles...", tile_count);
    
    // Create the map (matching generate.rs constants)
    let mut map = Map::new(GRID_X as i32, GRID_Y as i32, TILE_SIZE);
    
    // Calculate grid origin (map is centered, from setup_generator)
    let grid_origin_x = -TILE_SIZE * GRID_X as f32 / 2.0;
    let grid_origin_y = -TILE_SIZE * GRID_Y as f32 / 2.0;
    
    // Track the highest z-layer at each (x,y) position
    // This solves the multi-layer problem: only the topmost visible tile matters!
    let mut layer_tracker: HashMap<(i32, i32), (TileType, f32)> = HashMap::new();
    
    // Scan all tiles and keep only the highest z-layer per position
    for (marker, transform) in tile_query.iter() {
        // Convert world position to grid coordinates
        let world_x = transform.translation.x;
        let world_y = transform.translation.y;
        let world_z = transform.translation.z; // Check z-height for layering
        
        let grid_x = ((world_x - grid_origin_x) / TILE_SIZE).floor() as i32;
        let grid_y = ((world_y - grid_origin_y) / TILE_SIZE).floor() as i32;
        
        let key = (grid_x, grid_y);
        
        // Only keep the tile with the HIGHEST z value at this position
        // This ensures water on top of dirt takes precedence
        match layer_tracker.get(&key) {
            Some((_, existing_z)) if world_z <= *existing_z => {
                // Lower or equal layer found, ignore this tile
                continue;
            }
            _ => {
                // Higher layer or first tile at this position - keep it
                layer_tracker.insert(key, (marker.tile_type, world_z));
            }
        }
    }
    
    info!("Processed {} tiles into {} unique grid positions", 
          tile_count, layer_tracker.len());
    
    // Now populate the map with only the top layer at each position
    for ((grid_x, grid_y), (tile_type, z_height)) in layer_tracker.iter() {
        map.set_tile(*grid_x, *grid_y, *tile_type);
        
        // Debug: print water tiles with their layer info
        if *tile_type == TileType::Water {
            debug!("Water tile at grid ({}, {}) z={:.1}", 
                   grid_x, grid_y, z_height);
        }
    }
    
    // Count walkable vs unwalkable
    let mut walkable = 0;
    let mut unwalkable = 0;
    for tile in &map.tiles {
        if tile.is_walkable() {
            walkable += 1;
        } else {
            unwalkable += 1;
        }
    }
    
    info!("Collision map built! Walkable: {}, Unwalkable: {}", 
          walkable, unwalkable);
    
    // Insert the map as a resource
    commands.insert_resource(map);
    
    // Mark as built
    built.0 = true;
}
```

### 3.2 Update `src/main.rs`

Add the collision map builder system and resource:

```rust
// Add to imports at the top:
use crate::map::generate::{setup_generator, build_collision_map, CollisionMapBuilt};

// In the main() function, after .add_plugins(...), add:
fn main() {
    // ... existing setup ...
    
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                // ... existing plugin config ...
        )
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        
        // NEW: Initialize collision tracking
        .init_resource::<CollisionMapBuilt>()
        
        .add_systems(Startup, (setup_camera, setup_generator))
        
        // NEW: Add collision map builder in Update
        // It will run every frame until map is built, then stop
        .add_systems(Update, build_collision_map)
        
        .add_plugins(PlayerPlugin)
        .run();
}
```

## ✅ Verification

### Step 1: Compile Check

```bash
cargo build
```

**Expected**: Should compile cleanly.

### Step 2: Run and Check Logs

```bash
cargo run
```

**Expected Output**:
```
INFO bevy_book_game::map::generate: Building collision map from 2250 tiles...
INFO bevy_book_game::map::generate: Processed 2250 tiles into 450 unique grid positions
DEBUG bevy_book_game::map::generate: Water tile at grid (5, 3) z=3.0
DEBUG bevy_book_game::map::generate: Water tile at grid (5, 4) z=3.0
... more water tiles ...
INFO bevy_book_game::map::generate: Collision map built! Walkable: 425, Unwalkable: 25
```

**What to look for**:
- ✅ "Building collision map" shows ~2250 tiles (5 layers × 450 positions)
- ✅ "Processed into 450 unique positions" (layer deduplication works!)
- ✅ Multiple water tiles detected (means tagging worked!)
- ✅ Unwalkable count > 0 (means we have obstacles)
- ✅ Water tiles show z-height (confirms layer tracking)
- ✅ Message appears only ONCE (not every frame)

### Step 3: Verify Map Dimensions

Add temporary debug in `build_collision_map` after creating the map:

```rust
let mut map = Map::new(GRID_X as i32, GRID_Y as i32, TILE_SIZE);

// Debug print
info!("Map dimensions: {}x{}, total tiles: {}", 
      map.width, map.height, map.tiles.len());
```

**Expected**: 
```
INFO Map dimensions: 25x18, total tiles: 450
```

### Step 4: Test Map Query

Add this temporary system to `main.rs` to test the map:

```rust
// Temporary test system - add to main.rs
fn test_collision_map(map: Option<Res<Map>>) {
    let Some(map) = map else {
        return;
    };
    
    // Test center of map (should be walkable grass/dirt)
    let center_walkable = map.is_walkable(12, 9);
    info!("Center tile (12, 9) walkable: {}", center_walkable);
    
    // Test if we can query the map
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.is_walkable(x, y) {
                debug!("Unwalkable tile at ({}, {})", x, y);
            }
        }
    }
}

// Add to systems:
.add_systems(Update, (build_collision_map, test_collision_map))
```

**Expected**: 
- Logs showing unwalkable tiles
- No panic/crash
- Map queries work

Then **remove** this test system once verified.

## 🎓 What We Learned

- ✅ Built map from spawned entities
- ✅ System runs once then stops (efficient!)
- ✅ World-to-grid conversion works correctly
- ✅ Map resource is available to query
- ✅ Water tiles are correctly marked as unwalkable

## 🐛 Common Issues

**Issue**: "Building collision map from 0 tiles"
- **Cause**: System runs before WFC generates tiles
- **Fix**: This is normal! System will retry next frame. Should succeed within 1-2 seconds.

**Issue**: All tiles marked as walkable (unwalkable count = 0)
- **Cause**: Tiles aren't tagged with TileTypeMarker
- **Fix**: Go back to Step 2, ensure `.with_tile_type()` is added

**Issue**: Map built multiple times
- **Cause**: `built.0 = true` not set
- **Fix**: Ensure you set the flag after building

**Issue**: "Collision map built! Walkable: 0, Unwalkable: 0"
- **Cause**: Map is empty, tiles aren't being scanned
- **Fix**: Check that `tile_query` actually returns results. Add debug print:
  ```rust
  info!("Scanning {} tiles", tile_count);
  ```

**Issue**: Map dimensions wrong
- **Cause**: Wrong constants used
- **Fix**: Use `GRID_X`, `GRID_Y`, `TILE_SIZE` from `generate.rs`

**Issue**: Player can walk on water that has grass underneath
- **Cause**: Not handling layers correctly - grass overwrites water
- **Fix**: Ensure using the HashMap layer tracker approach
- **Verify**: Check logs show "Processed X tiles into 450 unique positions"
- **Debug**: Add print to see which z-value wins:
  ```rust
  debug!("Grid ({}, {}) choosing {:?} at z={} over lower layers", 
         grid_x, grid_y, tile_type, world_z);
  ```

## 🎯 Visual Verification

Want to see the collision map visually? Add this debug system:

```rust
fn debug_draw_collision(
    map: Option<Res<Map>>,
    mut gizmos: Gizmos,
) {
    let Some(map) = map else { return };
    
    let grid_origin_x = -(map.width as f32 * map.tile_size) / 2.0;
    let grid_origin_y = -(map.height as f32 * map.tile_size) / 2.0;
    
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.is_walkable(x, y) {
                // Draw red square for unwalkable tiles
                let world_x = grid_origin_x + (x as f32 * map.tile_size) + (map.tile_size / 2.0);
                let world_y = grid_origin_y + (y as f32 * map.tile_size) + (map.tile_size / 2.0);
                
                gizmos.rect_2d(
                    Vec2::new(world_x, world_y),
                    0.0,
                    Vec2::splat(map.tile_size * 0.9),
                    Color::srgba(1.0, 0.0, 0.0, 0.3),
                );
            }
        }
    }
}

// Add to Update systems
.add_systems(Update, debug_draw_collision)
```

**Expected**: Red transparent squares over water and trees/rocks!

## ➡️ Next Step

Once the collision map is building correctly:

**👉 Continue to [STEP_4_PLAYER_COLLISION.md](STEP_4_PLAYER_COLLISION.md)**

The final step - actually preventing the player from walking on water!

