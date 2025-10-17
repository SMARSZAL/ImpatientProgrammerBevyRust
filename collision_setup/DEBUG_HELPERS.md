# Debug Helpers

Useful debug systems and functions to help verify collision is working correctly.

## 🎨 Visual Debug Systems

### 1. Draw Unwalkable Tiles (Red Overlay)

Shows red transparent squares over all unwalkable tiles.

```rust
// Add to main.rs or a debug module
use bevy::prelude::*;
use crate::map::Map;

fn debug_draw_collision_overlay(
    map: Option<Res<Map>>,
    mut gizmos: Gizmos,
) {
    let Some(map) = map.as_ref() else { return };
    
    let grid_origin_x = -(map.width as f32 * map.tile_size) / 2.0;
    let grid_origin_y = -(map.height as f32 * map.tile_size) / 2.0;
    
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.is_walkable(x, y) {
                let world_x = grid_origin_x + (x as f32 * map.tile_size) + (map.tile_size / 2.0);
                let world_y = grid_origin_y + (y as f32 * map.tile_size) + (map.tile_size / 2.0);
                
                gizmos.rect_2d(
                    Vec2::new(world_x, world_y),
                    0.0,
                    Vec2::splat(map.tile_size * 0.9),
                    Color::srgba(1.0, 0.0, 0.0, 0.3), // Red, 30% opacity
                );
            }
        }
    }
}

// Add to app:
.add_systems(Update, debug_draw_collision_overlay)
```

**What you'll see**: Red semi-transparent squares over water, trees, rocks.

---

### 2. Player Grid Position Indicator

Shows which grid cell the player is currently in.

```rust
use bevy::prelude::*;
use crate::{map::Map, player::Player};

fn debug_player_grid_indicator(
    player: Query<&Transform, With<Player>>,
    map: Option<Res<Map>>,
    mut gizmos: Gizmos,
) {
    let Some(map) = map.as_ref() else { return };
    let Ok(transform) = player.get_single() else { return };
    
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let grid = map.world_to_grid(pos);
    
    if map.in_bounds(grid.x, grid.y) {
        // Draw yellow circle around player
        gizmos.circle_2d(pos, 20.0, Color::srgb(1.0, 1.0, 0.0));
        
        // Draw grid cell outline
        let grid_origin_x = -(map.width as f32 * map.tile_size) / 2.0;
        let grid_origin_y = -(map.height as f32 * map.tile_size) / 2.0;
        let cell_center = Vec2::new(
            grid_origin_x + (grid.x as f32 * map.tile_size) + (map.tile_size / 2.0),
            grid_origin_y + (grid.y as f32 * map.tile_size) + (map.tile_size / 2.0),
        );
        
        gizmos.rect_2d(
            cell_center,
            0.0,
            Vec2::splat(map.tile_size),
            Color::srgb(1.0, 1.0, 0.0), // Yellow
        );
    }
}

// Add to app:
.add_systems(Update, debug_player_grid_indicator)
```

**What you'll see**: Yellow box around the tile player is currently on.

---

### 3. Walkability Rays

Shows green lines toward walkable adjacent tiles, red lines toward blocked tiles.

```rust
use bevy::prelude::*;
use crate::{map::Map, player::Player};

fn debug_walkability_rays(
    player: Query<&Transform, With<Player>>,
    map: Option<Res<Map>>,
    mut gizmos: Gizmos,
) {
    let Some(map) = map.as_ref() else { return };
    let Ok(transform) = player.get_single() else { return };
    
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let grid = map.world_to_grid(pos);
    
    let grid_origin_x = -(map.width as f32 * map.tile_size) / 2.0;
    let grid_origin_y = -(map.height as f32 * map.tile_size) / 2.0;
    
    // Check 8 directions (cardinal + diagonal)
    let offsets = [
        (0, 1),   // up
        (0, -1),  // down
        (1, 0),   // right
        (-1, 0),  // left
        (1, 1),   // up-right
        (1, -1),  // down-right
        (-1, 1),  // up-left
        (-1, -1), // down-left
    ];
    
    for (dx, dy) in offsets {
        let check_x = grid.x + dx;
        let check_y = grid.y + dy;
        
        if !map.in_bounds(check_x, check_y) {
            continue;
        }
        
        let check_world = Vec2::new(
            grid_origin_x + (check_x as f32 * map.tile_size) + (map.tile_size / 2.0),
            grid_origin_y + (check_y as f32 * map.tile_size) + (map.tile_size / 2.0),
        );
        
        let color = if map.is_walkable(check_x, check_y) {
            Color::srgb(0.0, 1.0, 0.0) // Green = walkable
        } else {
            Color::srgb(1.0, 0.0, 0.0) // Red = blocked
        };
        
        gizmos.line_2d(pos, check_world, color);
    }
}

// Add to app:
.add_systems(Update, debug_walkability_rays)
```

**What you'll see**: Green lines toward walkable tiles, red lines toward water/obstacles.

---

## 📊 Console Debug Systems

### 4. Print Player Tile Info

Prints what tile type player is standing on (every second to avoid spam).

```rust
use bevy::prelude::*;
use crate::{map::Map, player::Player};

#[derive(Resource)]
struct DebugTimer(Timer);

fn debug_print_player_tile(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    player: Query<&Transform, With<Player>>,
    map: Option<Res<Map>>,
) {
    // Initialize timer
    let timer = timer.get_or_insert_with(|| {
        Timer::from_seconds(1.0, TimerMode::Repeating)
    });
    
    timer.tick(time.delta());
    
    if !timer.just_finished() {
        return;
    }
    
    let Some(map) = map.as_ref() else { return };
    let Ok(transform) = player.get_single() else { return };
    
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let grid = map.world_to_grid(pos);
    
    if map.in_bounds(grid.x, grid.y) {
        let idx = map.xy_idx(grid.x, grid.y);
        let tile = &map.tiles[idx];
        let walkable = tile.is_walkable();
        
        println!(
            "Player: world({:.1}, {:.1}) grid({}, {}) tile={:?} walkable={}",
            pos.x, pos.y, grid.x, grid.y, tile, walkable
        );
    }
}

// Add to app:
.add_systems(Update, debug_print_player_tile)
```

**Console output**:
```
Player: world(128.5, -64.2) grid(15, 7) tile=Grass walkable=true
Player: world(96.3, -32.1) grid(14, 8) tile=Water walkable=false
```

---

### 5. Collision Event Logger

Prints message when player tries to walk into unwalkable tile.

```rust
use bevy::prelude::*;
use crate::{map::Map, player::{Player, AnimationState}};

fn debug_log_collision_attempts(
    player: Query<(&Transform, &AnimationState), With<Player>>,
    map: Option<Res<Map>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Some(map) = map.as_ref() else { return };
    let Ok((transform, anim)) = player.get_single() else { return };
    
    // Check if trying to move but not actually moving (= collision!)
    let trying_to_move = input.pressed(KeyCode::ArrowLeft)
        || input.pressed(KeyCode::ArrowRight)
        || input.pressed(KeyCode::ArrowUp)
        || input.pressed(KeyCode::ArrowDown);
    
    if trying_to_move && !anim.moving {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        let grid = map.world_to_grid(pos);
        
        println!("⛔ COLLISION! Player at grid ({}, {}) - trying to move but blocked", 
                 grid.x, grid.y);
    }
}

// Add to app:
.add_systems(Update, debug_log_collision_attempts)
```

**Console output**:
```
⛔ COLLISION! Player at grid (10, 5) - trying to move but blocked
```

---

### 6. Tile Statistics

Print breakdown of tile types in the map (run once after map builds).

```rust
use bevy::prelude::*;
use crate::map::{Map, TileType};
use std::collections::HashMap;

fn debug_print_tile_stats(
    map: Option<Res<Map>>,
    mut printed: Local<bool>,
) {
    if *printed {
        return;
    }
    
    let Some(map) = map.as_ref() else { return };
    
    let mut counts: HashMap<String, usize> = HashMap::new();
    
    for tile in &map.tiles {
        let name = format!("{:?}", tile);
        *counts.entry(name).or_insert(0) += 1;
    }
    
    println!("\n=== TILE STATISTICS ===");
    for (tile_type, count) in counts.iter() {
        let percentage = (*count as f32 / map.tiles.len() as f32) * 100.0;
        println!("{}: {} ({:.1}%)", tile_type, count, percentage);
    }
    println!("Total tiles: {}\n", map.tiles.len());
    
    *printed = true;
}

// Add to app:
.add_systems(Update, debug_print_tile_stats)
```

**Console output**:
```
=== TILE STATISTICS ===
Grass: 180 (40.0%)
Dirt: 150 (33.3%)
YellowGrass: 85 (18.9%)
Water: 25 (5.6%)
Tree: 8 (1.8%)
Rock: 2 (0.4%)
Total tiles: 450
```

---

## 🔧 Testing Helpers

### 7. Force Player to Position

Useful for testing specific tiles without walking there.

```rust
// Press 'T' to teleport to different test positions
fn debug_teleport_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    map: Option<Res<Map>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyT) {
        return;
    }
    
    let Some(map) = map.as_ref() else { return };
    let Ok(mut transform) = player.get_single_mut() else { return };
    
    // Test positions: water edge, center, corner, etc.
    static mut TEST_INDEX: usize = 0;
    
    let test_positions = vec![
        (12, 9),   // Center (should be walkable)
        (10, 5),   // Near water (check for water nearby)
        (0, 0),    // Corner
        (24, 17),  // Opposite corner
        (5, 5),    // Random spot
    ];
    
    unsafe {
        let (grid_x, grid_y) = test_positions[TEST_INDEX % test_positions.len()];
        TEST_INDEX += 1;
        
        // Convert grid to world
        let grid_origin_x = -(map.width as f32 * map.tile_size) / 2.0;
        let grid_origin_y = -(map.height as f32 * map.tile_size) / 2.0;
        
        transform.translation.x = grid_origin_x + (grid_x as f32 * map.tile_size) + (map.tile_size / 2.0);
        transform.translation.y = grid_origin_y + (grid_y as f32 * map.tile_size) + (map.tile_size / 2.0);
        
        println!("Teleported to grid ({}, {})", grid_x, grid_y);
    }
}

// Add to app:
.add_systems(Update, debug_teleport_player)
```

---

## 🎮 Quick Setup

To enable ALL debug helpers at once, add this to your `main.rs`:

```rust
// In main(), add a debug mode flag
const DEBUG_COLLISION: bool = true; // Set to false for release

// Then conditionally add systems:
if DEBUG_COLLISION {
    app.add_systems(Update, (
        debug_draw_collision_overlay,
        debug_player_grid_indicator,
        debug_walkability_rays,
        debug_print_player_tile,
        debug_log_collision_attempts,
        debug_print_tile_stats,
        debug_teleport_player,
    ));
}
```

## ⚠️ Performance Warning

These debug systems are **expensive**! They:
- Draw many gizmos every frame (GPU cost)
- Print to console frequently (I/O cost)
- Iterate through all map tiles (CPU cost)

**Disable before measuring real performance or releasing!**

## 🎯 Recommended Debug Workflow

1. **Start**: Enable `debug_print_tile_stats` to verify map has correct tiles
2. **Build**: Enable `debug_draw_collision_overlay` to see unwalkable areas
3. **Test**: Enable `debug_player_grid_indicator` and `debug_walkability_rays`
4. **Polish**: Enable `debug_log_collision_attempts` to verify edge cases
5. **Ship**: Disable ALL debug systems for release

---

**Tip**: Create a separate `debug.rs` module to keep these organized and easy to toggle on/off!

