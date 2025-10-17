# Quick Reference Card

A condensed cheat sheet for the collision system.

## 📋 Files Modified

| File | What Changed | Lines Added |
|------|-------------|-------------|
| `src/map/tile_marker.rs` | **NEW** - TileType enum & marker component | ~60 |
| `src/map/collision.rs` | **NEW** - Map resource | ~70 |
| `src/map/mod.rs` | Export new modules | ~3 |
| `src/map/assets.rs` | Add tile_type field, enhanced spawner | ~20 |
| `src/map/rules.rs` | Tag tiles with `.with_tile_type()` | ~40 |
| `src/map/generate.rs` | Collision map builder system | ~50 |
| `src/player/systems.rs` | Add collision check to movement | ~10 |
| `src/main.rs` | Add collision system | ~3 |

**Total**: ~256 lines added/modified

---

## 🗺️ Key Data Structures

### TileType Enum
```rust
enum TileType {
    Dirt,         // walkable
    Grass,        // walkable
    YellowGrass,  // walkable
    Water,        // NOT walkable ⚠️
    Tree,         // NOT walkable
    Rock,         // NOT walkable
    Plant,        // walkable (small)
    Stump,        // walkable
    Empty,        // walkable (void)
}
```

### Map Resource
```rust
struct Map {
    tiles: Vec<TileType>,  // Flat array, row-major
    width: i32,            // 25
    height: i32,           // 18
    tile_size: f32,        // 32.0
}
```

### Key Methods
```rust
map.is_walkable(x, y) -> bool
map.world_to_grid(Vec2) -> IVec2
map.is_world_pos_walkable(Vec2) -> bool
```

---

## 🔑 Critical Code Locations

### Tagging Tiles (rules.rs)
```rust
vec![SpawnableAsset::new("water").with_tile_type(TileType::Water)]
vec![SpawnableAsset::new("dirt").with_tile_type(TileType::Dirt)]
vec![SpawnableAsset::new("green_grass").with_tile_type(TileType::Grass)]
```

### Collision Check (player/systems.rs)
```rust
let can_move = if let Some(map) = map.as_ref() {
    map.is_world_pos_walkable(new_pos)
} else {
    true  // No map yet, allow movement
};

if can_move {
    transform.translation.x = new_pos.x;
    transform.translation.y = new_pos.y;
}
```

### Map Builder (generate.rs)
```rust
fn build_collision_map(
    mut commands: Commands,
    mut built: ResMut<CollisionMapBuilt>,
    tile_query: Query<(&TileTypeMarker, &Transform)>,
) {
    if built.0 { return; }
    if tile_query.iter().count() == 0 { return; }
    
    // CRITICAL: Handle multi-layer problem!
    // Track highest z-layer at each (x,y) position
    let mut layer_tracker: HashMap<(i32, i32), (TileType, f32)> = HashMap::new();
    
    for (marker, transform) in tile_query.iter() {
        let world_z = transform.translation.z;
        let key = (grid_x, grid_y);
        
        // Only keep highest z-value tile per position
        match layer_tracker.get(&key) {
            Some((_, existing_z)) if world_z <= *existing_z => continue,
            _ => { layer_tracker.insert(key, (marker.tile_type, world_z)); }
        }
    }
    
    // Populate map from top layers only
    for ((x, y), (tile_type, _)) in layer_tracker.iter() {
        map.set_tile(*x, *y, *tile_type);
    }
    
    commands.insert_resource(map);
    built.0 = true;
}
```

---

## ✅ Testing Checklist

After implementation:

- [ ] Compile: `cargo build`
- [ ] Run: `cargo run`
- [ ] Wait for "Collision map built!" in logs
- [ ] Walk on grass - should work ✅
- [ ] Walk on dirt - should work ✅
- [ ] Walk into water - should stop ⛔
- [ ] Walk into trees - should stop ⛔
- [ ] Walk into rocks - should stop ⛔
- [ ] Walk through plants - should work ✅
- [ ] Walk animation stops when blocked ✅

---

## 🐛 Quick Troubleshooting

| Problem | Quick Fix |
|---------|----------|
| Player walks on water | Check Step 2: tag water tiles with `.with_tile_type(TileType::Water)` |
| Player can't move at all | Check spawn position (0, 0) is walkable |
| Map never builds | Add `build_collision_map` to Update systems in main.rs |
| Compile error on TileType | Add `use crate::map::TileType;` |
| Wrong grid coordinates | Verify GRID_X=25, GRID_Y=18, TILE_SIZE=32.0 |

---

## 📊 Expected Output

### Console Logs
```
INFO bevy_book_game::map::generate: Building collision map from 2250 tiles...
INFO bevy_book_game::map::generate: Processed 2250 tiles into 450 unique grid positions
DEBUG bevy_book_game::map::generate: Water tile at grid (5, 3) z=3.0
INFO bevy_book_game::map::generate: Collision map built! Walkable: 425, Unwalkable: 25
```

### Map Statistics
- Input tiles: ~2250 (5 layers × 450 positions)
- Unique positions: 450 (25 × 18)
- Walkable: ~90-95% (grass, dirt, yellow grass, plants)
- Unwalkable: ~5-10% (water, trees, rocks)
- Layer deduplication: 5:1 ratio (2250 → 450)

---

## 🚀 Extension Ideas

Once water collision works:

### Easy Additions
```rust
// Friction (slower on grass)
let friction = map.get_tile_friction(grid_pos);
let adjusted_delta = delta * friction;

// Sound effects
if !can_move {
    audio.play("bonk.ogg");
}

// Visual feedback
if !can_move {
    // Shake screen, spawn particle, etc.
}
```

### Medium Additions
- Sliding along walls (try X-only if XY blocked)
- Collision with smaller radius (circle collision)
- Different speeds (walk/run toggle)

### Advanced Additions
- Tripping mechanic (random on certain tiles)
- Push-back on collision (bounce effect)
- Terrain-based animation (wade through water edge)

---

## 📚 Pattern Reference

This implementation follows the **roguelike tutorial pattern**:

1. **Simple enum** for tile types
2. **Flat Vec** for map storage (not HashMap)
3. **xy_idx()** function for 2D→1D conversion
4. **is_walkable()** method for collision query
5. **ECS resource** for global map access

Why this pattern?
- ✅ Simple and understandable
- ✅ Fast (array access, no hashing)
- ✅ Memory efficient (contiguous storage)
- ✅ Cache-friendly (linear reads)
- ✅ Easy to extend (add fields to enum)

---

## 🎓 Core Concepts

### Multi-Layer Handling (CRITICAL!)

Your map has **5 z-layers** stacked vertically:
```
Layer 4: Props (trees, rocks)     z ≈ 4.0
Layer 3: Water                     z ≈ 3.0
Layer 2: Yellow grass              z ≈ 2.0
Layer 1: Green grass               z ≈ 1.0
Layer 0: Dirt (base)               z ≈ 0.0
```

**Problem**: At position (10, 5) you might have:
- Dirt at z=0 (walkable)
- Grass at z=1 (walkable)  
- Water at z=3 (NOT walkable)

**Solution**: "Top layer wins" - only the highest z-value matters!
```rust
// HashMap tracks highest z per position
layer_tracker.insert((x, y), (TileType, z_height));
// Water (z=3) overwrites grass (z=1) overwrites dirt (z=0)
// Final: position marked as Water (not walkable)
```

This ensures collision matches what player sees visually.

### World vs Grid Coordinates

```
World Space (pixels):        Grid Space (tiles):
    ^                            ^
    |                            |
    | (-100, 50)                 | (10, 12)
    |                            |
    +-------> x                  +-------> x
   (0,0) center                 (0,0) top-left
```

### Conversion Formula
```rust
// World to grid (with centered origin)
grid_x = (world_x - grid_origin_x) / tile_size
grid_y = (world_y - grid_origin_y) / tile_size

// Grid to array index
index = (y * width) + x
```

### Component Flow
```
SpawnableAsset
    ↓ (components_spawner)
TileTypeMarker on Entity
    ↓ (build_collision_map)
Map Resource (Vec<TileType>)
    ↓ (move_player)
Collision Check
```

---

## 💡 Best Practices

1. **Always check map bounds** before array access
2. **Use Option<Res<Map>>** to handle missing map gracefully
3. **Build map once**, not every frame (use flag)
4. **Tag ALL variants** of a tile (water corners, water sides, etc.)
5. **Test at map edges** and corners
6. **Add debug visualizations** during development
7. **Remove debug code** before release

---

## 📞 Need Help?

1. Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues
2. Use [DEBUG_HELPERS.md](DEBUG_HELPERS.md) for visualization tools
3. Review step-by-step guides:
   - [STEP_1_DATA_STRUCTURES.md](STEP_1_DATA_STRUCTURES.md)
   - [STEP_2_TAG_TILES.md](STEP_2_TAG_TILES.md)
   - [STEP_3_BUILD_MAP.md](STEP_3_BUILD_MAP.md)
   - [STEP_4_PLAYER_COLLISION.md](STEP_4_PLAYER_COLLISION.md)

---

**Remember**: Keep it simple! This is a tile-based game, not a physics simulation. Simple checks are fast and reliable.

