# Step 4: Player Collision Detection

## 🎯 Goal
Modify the player movement system to check the collision map and prevent walking on water.

## 💡 Strategy

Simple approach:
1. Calculate where player WANTS to move
2. Check if that position is walkable
3. If walkable: allow movement
4. If not walkable: block movement

## 📝 Implementation

### 4.1 Modify `src/player/systems.rs`

We need to update the `move_player` function to query the Map resource and check walkability.

#### Add import at the top:

```rust
// At the top of the file, add:
use crate::map::{Map, TILE_SIZE as MAP_TILE_SIZE};
```

#### Update the `move_player` function:

Find the `move_player` function (around line 46) and update it:

```rust
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Option<Res<Map>>,  // NEW: Query the collision map
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        // Calculate the proposed new position
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let new_pos = current_pos + delta;
        
        // NEW: Check if the new position is walkable
        let can_move = if let Some(map) = map.as_ref() {
            map.is_world_pos_walkable(new_pos)
        } else {
            // No map yet, allow movement (safety fallback)
            true
        };
        
        // Only move if the destination is walkable
        if can_move {
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
            anim.moving = true;

            // Update facing direction
            if direction.x.abs() > direction.y.abs() {
                anim.facing = if direction.x > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            } else {
                anim.facing = if direction.y > 0.0 {
                    Facing::Up
                } else {
                    Facing::Down
                };
            }
        } else {
            // Blocked by unwalkable tile - stop moving
            anim.moving = false;
        }
    } else {
        anim.moving = false;
    }
}
```

That's it! This is the **only** change needed to add collision detection.

### 4.2 (Optional) Add Debug Feedback

If you want to see when collision happens, add some debug output:

```rust
// Add this after the can_move check:
if !can_move {
    let grid_pos = map.as_ref().unwrap().world_to_grid(new_pos);
    debug!("Collision! Blocked at grid ({}, {})", grid_pos.x, grid_pos.y);
}
```

### 4.3 (Optional) Better Edge Handling

The above works, but the player might feel "sticky" at edges. Here's an improved version that checks a few pixels ahead:

```rust
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Option<Res<Map>>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let new_pos = current_pos + delta;
        
        // Check collision using the player's center point
        // You could also check multiple points (corners) for more accuracy
        let can_move = if let Some(map) = map.as_ref() {
            // Check the destination
            let dest_walkable = map.is_world_pos_walkable(new_pos);
            
            // Optional: also check current position to handle being "stuck"
            // This ensures if player somehow gets on unwalkable tile, they can escape
            if !dest_walkable {
                let current_walkable = map.is_world_pos_walkable(current_pos);
                // Allow movement if currently on unwalkable tile (let them escape)
                !current_walkable
            } else {
                true
            }
        } else {
            true // No map yet, allow movement
        };
        
        if can_move {
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
            anim.moving = true;

            if direction.x.abs() > direction.y.abs() {
                anim.facing = if direction.x > 0.0 {
                    Facing::Right
                } else {
                    Facing::Left
                };
            } else {
                anim.facing = if direction.y > 0.0 {
                    Facing::Up
                } else {
                    Facing::Down
                };
            }
        } else {
            anim.moving = false;
        }
    } else {
        anim.moving = false;
    }
}
```

## ✅ Verification

### Step 1: Compile Check

```bash
cargo build
```

**Expected**: Clean compile!

### Step 2: Run the Game

```bash
cargo run
```

**Expected**: Game runs normally.

### Step 3: Test Water Collision

1. **Wait for map to generate** (1-2 seconds)
2. **Walk around** with arrow keys
3. **Find water** (blue tiles)
4. **Try to walk into water**

**Expected Behavior**:
- ✅ Player can walk on dirt (brown)
- ✅ Player can walk on grass (green)
- ✅ Player can walk on yellow grass
- ✅ Player **CANNOT** walk on water (blue)
- ✅ Player stops at water's edge
- ✅ Walk animation stops when blocked
- ✅ Can walk parallel to water edge
- ✅ No crash or panic

### Step 4: Test Tree/Rock Collision

Walk into trees and rocks:

**Expected**:
- ✅ Cannot walk through big trees
- ✅ Cannot walk through rocks
- ✅ CAN walk through small plants (they're marked walkable)
- ✅ CAN walk through stumps (they're marked walkable)

### Step 5: Edge Cases

Test these scenarios:

1. **Corner collision**: Try to walk diagonally into water corner
   - **Expected**: Blocked, can't enter
   
2. **Rapid direction change**: Tap arrows rapidly near water
   - **Expected**: No glitches, clean blocking

3. **Start position**: Make sure player doesn't spawn on water
   - **Expected**: Player visible and can move

## 🎓 What We Learned

- ✅ Simple collision check in movement system
- ✅ No complex physics needed for tile-based collision
- ✅ Optional<Res<Map>> handles case where map isn't ready yet
- ✅ Clean separation: collision map is independent of movement logic

## 🐛 Common Issues

**Issue**: Player can still walk on water
- **Cause 1**: Map not built yet. Wait 2-3 seconds for "Collision map built!" message.
- **Cause 2**: Water tiles not tagged. Check Step 2, ensure all water tiles have `.with_tile_type(TileType::Water)`
- **Cause 3**: World-to-grid conversion wrong. Check map origin calculation.
- **Fix**: Add debug print to see what tile type is being checked:
  ```rust
  if let Some(map) = map.as_ref() {
      let grid_pos = map.world_to_grid(new_pos);
      let tile = &map.tiles[map.xy_idx(grid_pos.x, grid_pos.y)];
      println!("Checking position {:?}, tile: {:?}, walkable: {}", 
               grid_pos, tile, tile.is_walkable());
  }
  ```

**Issue**: Player gets stuck and can't move at all
- **Cause**: Player spawned on unwalkable tile, or collision check is inverted
- **Fix**: Check player spawn position in spawn_player() - should be (0, 0, PLAYER_Z)
- **Fix**: Verify `is_walkable()` logic is correct (Water = false)

**Issue**: Collision feels "sticky" or unresponsive
- **Cause**: Checking too early/late in movement
- **Fix**: Use the "improved version" that checks both current and destination

**Issue**: "cannot find value `MAP_TILE_SIZE`"
- **Cause**: Import wrong
- **Fix**: Should import from map module: `use crate::map::TILE_SIZE as MAP_TILE_SIZE;`
  Or just reference it as `crate::map::generate::TILE_SIZE`

## 🎯 Visual Debug Helpers

Want to see collision checks in real-time? Add this system:

```rust
fn debug_player_collision(
    player: Query<&Transform, With<Player>>,
    map: Option<Res<Map>>,
    mut gizmos: Gizmos,
) {
    let Some(map) = map.as_ref() else { return };
    let Ok(transform) = player.get_single() else { return };
    
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let grid_pos = map.world_to_grid(pos);
    
    // Draw player grid position
    gizmos.circle_2d(pos, 20.0, Color::srgb(1.0, 1.0, 0.0));
    
    // Check 4 directions
    let offsets = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // up, down, right, left
    for (dx, dy) in offsets {
        let check_x = grid_pos.x + dx;
        let check_y = grid_pos.y + dy;
        
        if map.is_walkable(check_x, check_y) {
            // Draw green line toward walkable tiles
            let check_world = Vec2::new(
                (check_x as f32 + 0.5) * map.tile_size - (map.width as f32 * map.tile_size / 2.0),
                (check_y as f32 + 0.5) * map.tile_size - (map.height as f32 * map.tile_size / 2.0),
            );
            gizmos.line_2d(pos, check_world, Color::srgb(0.0, 1.0, 0.0));
        } else {
            // Draw red line toward unwalkable tiles
            let check_world = Vec2::new(
                (check_x as f32 + 0.5) * map.tile_size - (map.width as f32 * map.tile_size / 2.0),
                (check_y as f32 + 0.5) * map.tile_size - (map.height as f32 * map.tile_size / 2.0),
            );
            gizmos.line_2d(pos, check_world, Color::srgb(1.0, 0.0, 0.0));
        }
    }
}

// Add to Update systems
.add_systems(Update, debug_player_collision)
```

**Expected**: Yellow circle around player, green lines toward walkable tiles, red lines toward water!

## 🎉 Success!

If the player:
- ✅ Walks normally on terrain
- ✅ Cannot enter water tiles
- ✅ Stops cleanly at water edge
- ✅ Animation works correctly

**Congratulations! Water collision is working!** 🎊

## 🚀 What's Next?

Now that basic collision works, you could add:

1. **Friction system** (different speeds on different terrain)
   - Easy: just multiply `delta` by `map.get_friction(grid_pos)`
   
2. **Sound effects** when hitting obstacles
   - Play "bonk" sound when can_move == false

3. **Visual feedback** when blocked
   - Screen shake, particle effect, etc.

4. **Sliding along walls** (advanced)
   - Try X movement if XY movement blocked
   - Try Y movement if XY movement blocked

5. **Tripping mechanic** on certain tiles
   - Random chance on rocks/stumps

But for now, **water collision is complete!** ✨

---

## 📚 Reference

All 4 steps implemented:
- ✅ [STEP_1_DATA_STRUCTURES.md](STEP_1_DATA_STRUCTURES.md) - Created Map and TileType
- ✅ [STEP_2_TAG_TILES.md](STEP_2_TAG_TILES.md) - Tagged tiles during spawn
- ✅ [STEP_3_BUILD_MAP.md](STEP_3_BUILD_MAP.md) - Built collision map
- ✅ [STEP_4_PLAYER_COLLISION.md](STEP_4_PLAYER_COLLISION.md) - Added collision check

**Total code changed**: ~150 lines across 5 files!

