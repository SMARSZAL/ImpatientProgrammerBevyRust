# Troubleshooting Guide

## 🔧 Common Issues and Solutions

### Issue 1: Player Can Walk on Water

**Symptoms**: Player passes through water tiles like they're not there.

**Possible Causes**:

1. **Map not built yet**
   - Check logs for "Collision map built!" message
   - Should appear 1-2 seconds after game starts
   - **Solution**: Wait a moment after launch before testing

2. **Water tiles not tagged**
   - Check Step 2 implementation
   - **Debug**: Add print in `load_assets`:
     ```rust
     if tile_type == Some(TileType::Water) {
         println!("Tagged water: {}", sprite_name);
     }
     ```
   - **Solution**: Ensure ALL water variants have `.with_tile_type(TileType::Water)`

3. **World-to-grid conversion incorrect**
   - **Debug**: Print player position and grid check:
     ```rust
     let grid_pos = map.world_to_grid(new_pos);
     println!("Player at world {:?} = grid {:?}", new_pos, grid_pos);
     ```
   - **Solution**: Verify map origin calculation matches `setup_generator()`

4. **is_walkable() logic inverted**
   - **Debug**: Check TileType::Water returns false:
     ```rust
     assert!(!TileType::Water.is_walkable());
     ```
   - **Solution**: Fix match statement in `is_walkable()`

---

### Issue 2: Player Gets Stuck / Can't Move

**Symptoms**: Player freezes, can't move in any direction.

**Possible Causes**:

1. **Player spawned on unwalkable tile**
   - **Debug**: Check spawn position:
     ```rust
     // In spawn_player, add:
     println!("Player spawned at ({}, {})", 0.0, 0.0);
     ```
   - **Solution**: Verify (0, 0) is on walkable terrain (should be center of map)

2. **All tiles marked unwalkable**
   - **Debug**: Check map stats after building:
     ```rust
     println!("Walkable: {}, Unwalkable: {}", walkable_count, unwalkable_count);
     ```
   - **Solution**: Should have mostly walkable tiles. If all unwalkable, check tagging.

3. **Map dimensions wrong**
   - **Debug**: Print map size:
     ```rust
     println!("Map: {}x{} = {} tiles", map.width, map.height, map.tiles.len());
     ```
   - **Expected**: 25x18 = 450 tiles
   - **Solution**: Use correct GRID_X, GRID_Y, TILE_SIZE constants

---

### Issue 3: Compilation Errors

**Error**: `cannot find type 'TileType' in this scope`
- **File**: rules.rs, assets.rs, or generate.rs
- **Solution**: Add import at top of file:
  ```rust
  use crate::map::TileType;
  ```

**Error**: `no field 'tile_type' on type 'SpawnableAsset'`
- **File**: Probably assets.rs
- **Solution**: Add field to struct:
  ```rust
  pub struct SpawnableAsset {
      // ... existing fields ...
      tile_type: Option<TileType>,
  }
  ```

**Error**: `cannot find value 'MAP_TILE_SIZE' in this scope`
- **File**: player/systems.rs
- **Solution**: Import or use fully qualified path:
  ```rust
  use crate::map::TILE_SIZE as MAP_TILE_SIZE;
  // or just reference: crate::map::generate::TILE_SIZE
  ```

**Error**: `expected struct 'World', found struct 'Commands'`
- **File**: generate.rs in build_collision_map
- **Solution**: Use `commands.insert_resource()` not `world.insert_resource()`

---

### Issue 4: Map Never Builds

**Symptoms**: No "Collision map built!" message in logs.

**Possible Causes**:

1. **System not added to Update**
   - **Check**: main.rs should have:
     ```rust
     .add_systems(Update, build_collision_map)
     ```

2. **Tiles never spawn**
   - **Debug**: Check if WFC is working at all (do you see the map?)
   - **Solution**: If no map renders, issue is with WFC setup, not collision

3. **Query returns 0 tiles**
   - **Debug**: Add print in system:
     ```rust
     println!("Tile count: {}", tile_query.iter().count());
     ```
   - **Solution**: If always 0, tiles aren't being tagged (back to Step 2)

---

### Issue 5: Collision Detection Feels "Off"

**Symptoms**: Player stops before reaching water, or can slightly overlap water.

**Possible Causes**:

1. **Player position not centered**
   - Player sprite might be 64x64 but collision checks center point
   - **Solution**: Check multiple points (corners) or adjust collision radius

2. **Tile size mismatch**
   - Player TILE_SIZE (64) vs Map TILE_SIZE (32)
   - **Solution**: Use correct tile size for collision checks (Map's 32.0)

3. **Grid rounding issues**
   - floor() vs round() vs ceil() in world_to_grid
   - **Solution**: Use floor() consistently, matches tilemap placement

---

### Issue 6: Performance Issues

**Symptoms**: Game lags, low FPS.

**Possible Causes**:

1. **Collision map rebuilds every frame**
   - **Debug**: Count how many times "Building collision map" appears
   - **Should be**: Once only!
   - **Solution**: Ensure `built.0 = true` is set after building

2. **Debug systems left enabled**
   - Drawing collision gizmos for every tile is expensive
   - **Solution**: Remove debug visualization systems after testing

---

### Issue 7: Player Can Walk on Water with Grass Underneath

**Symptoms**: Player walks on tiles that visually show water, but collision treats them as grass.

**Possible Causes**:

1. **Layer handling not implemented correctly**
   - WFC generates 5 z-layers stacked on top of each other
   - Without proper handling, lower layers can overwrite higher ones
   - **Solution**: Ensure using the HashMap-based layer tracker in Step 3
   - **Verify**: Logs should show "Processed 2250 tiles into 450 unique positions"
   - **Debug**: Check that water tiles have higher z values:
     ```rust
     if marker.tile_type == TileType::Water {
         info!("Water at grid ({}, {}) z={}", grid_x, grid_y, world_z);
     }
     ```

2. **Iteration order problem**
   - If scanning tiles in wrong order, last tile wins instead of highest z
   - **Solution**: Use the match statement to compare z-values explicitly
   - **Verify**: Should see water z≈3.0, grass z≈1.0, dirt z≈0.0

### Issue 8: Player Glitches Through Water Sometimes

**Symptoms**: Occasionally player passes through water boundary.

**Possible Causes**:

1. **High speed / large delta**
   - Player moves fast enough to skip over collision check
   - **Solution**: Check collision at intermediate points or reduce MOVE_SPEED

2. **Diagonal movement**
   - Normalized diagonal is longer than expected
   - **Solution**: Ensure direction.normalize() is used

3. **Frame-dependent movement**
   - Delta time too large on lag spike
   - **Solution**: Clamp maximum delta or use fixed timestep

---

## 🧪 Debug Checklist

When something doesn't work, check in this order:

- [ ] Game compiles without errors
- [ ] Game runs without panicking
- [ ] Map renders on screen (WFC working)
- [ ] Player spawns and is visible
- [ ] "Collision map built!" message appears in logs
- [ ] Unwalkable count > 0 in logs
- [ ] Player can move normally on terrain
- [ ] Water tiles are visible in the map
- [ ] Player movement stops at water edge

## 🔍 Debug Tools

### See What Tile Player Is On

```rust
fn debug_player_tile(
    player: Query<&Transform, With<Player>>,
    map: Option<Res<Map>>,
) {
    let Some(map) = map.as_ref() else { return };
    let Ok(transform) = player.get_single() else { return };
    
    let pos = Vec2::new(transform.translation.x, transform.translation.y);
    let grid = map.world_to_grid(pos);
    
    if map.in_bounds(grid.x, grid.y) {
        let idx = map.xy_idx(grid.x, grid.y);
        let tile = &map.tiles[idx];
        println!("Player on tile {:?} at grid ({}, {})", tile, grid.x, grid.y);
    }
}
```

### Visualize Collision Map

```rust
fn debug_draw_unwalkable(
    map: Option<Res<Map>>,
    mut gizmos: Gizmos,
) {
    let Some(map) = map.as_ref() else { return };
    
    for y in 0..map.height {
        for x in 0..map.width {
            if !map.is_walkable(x, y) {
                let world_pos = /* calculate world position */;
                gizmos.rect_2d(
                    world_pos,
                    0.0,
                    Vec2::splat(map.tile_size * 0.9),
                    Color::srgba(1.0, 0.0, 0.0, 0.3),
                );
            }
        }
    }
}
```

### Count Tiles by Type

```rust
fn count_tile_types(map: Res<Map>) {
    let mut counts = std::collections::HashMap::new();
    for tile in &map.tiles {
        *counts.entry(format!("{:?}", tile)).or_insert(0) += 1;
    }
    for (tile_type, count) in counts {
        println!("{}: {}", tile_type, count);
    }
}
```

## 📝 Getting Help

If you're still stuck:

1. **Check logs carefully** - Most issues show up in console output
2. **Add debug prints** - Trace through the code flow
3. **Test each step** - Verify Step 1, then 2, then 3, then 4
4. **Compare with examples** - Check roguelike tutorial pattern
5. **Simplify** - Comment out complexity, get basic case working first

## 💡 Prevention Tips

- Test after each step before moving to next
- Keep debug prints until everything works
- Use cargo clippy for warnings
- Read compiler errors carefully (they're usually helpful!)
- Commit working code before making large changes

