# Water Collision Implementation Guide

## 🎯 Goal
Make the player unable to walk on water tiles using a simple tile-based collision system.

## 📋 Overview

We'll implement collision detection in **4 phases**:

1. **Phase 1**: Create the collision data structures (TileType enum, Map resource)
2. **Phase 2**: Tag tiles during spawn using `components_spawner`
3. **Phase 3**: Build the collision map from tagged entities
4. **Phase 4**: Modify player movement to check walkability

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Map Generation (WFC)                                    │
│  ├─ Build models with tile type metadata                │
│  ├─ Spawn tiles with TileTypeMarker component           │
│  └─ Tag via components_spawner (EXISTING mechanism!)    │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  Collision Map Builder (NEW system)                      │
│  ├─ Runs once after tiles spawn                         │
│  ├─ Queries all entities with TileTypeMarker            │
│  ├─ Builds Vec<TileType> from their positions           │
│  └─ Inserts Map resource                                │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  Player Movement (MODIFIED)                              │
│  ├─ Queries Map resource                                │
│  ├─ Converts player position to grid coords             │
│  ├─ Checks if target tile is walkable                   │
│  └─ Allows/blocks movement accordingly                  │
└─────────────────────────────────────────────────────────┘
```

## 📁 Files We'll Create/Modify

### New Files:
- `src/map/collision.rs` - Collision data structures and Map
- `src/map/tile_marker.rs` - Component to tag tiles during spawn

### Files to Modify:
- `src/map/mod.rs` - Export new modules
- `src/map/assets.rs` - Add tile_type to SpawnableAsset
- `src/map/rules.rs` - Tag each tile type during model creation
- `src/map/generate.rs` - Add collision map builder system
- `src/player/systems.rs` - Add walkability check to movement
- `src/main.rs` - May need to add system ordering

## ⏱️ Estimated Time

- **Phase 1**: 15 minutes (data structures)
- **Phase 2**: 30 minutes (tagging tiles)
- **Phase 3**: 20 minutes (building map)
- **Phase 4**: 15 minutes (movement check)
- **Total**: ~1.5 hours

## 📚 Implementation Order

Follow these guides in order:

1. **[STEP_1_DATA_STRUCTURES.md](STEP_1_DATA_STRUCTURES.md)** - Create TileType and Map
2. **[STEP_2_TAG_TILES.md](STEP_2_TAG_TILES.md)** - Tag tiles during spawn
3. **[STEP_3_BUILD_MAP.md](STEP_3_BUILD_MAP.md)** - Build collision map
4. **[STEP_4_PLAYER_COLLISION.md](STEP_4_PLAYER_COLLISION.md)** - Check collisions

## 🧪 Testing Strategy

After each phase, we'll:
1. ✅ Verify code compiles
2. ✅ Run the game
3. ✅ Check specific behavior
4. ✅ Add debug prints to verify data

## 🚀 Quick Start

```bash
# 1. Read STEP_1_DATA_STRUCTURES.md
# 2. Implement the code
# 3. Run: cargo build
# 4. Verify compilation
# 5. Move to next step
```

## 🐛 Troubleshooting

If you get stuck, check:
- **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** - Common issues and solutions
- **[DEBUG_HELPERS.md](DEBUG_HELPERS.md)** - Debug print helpers

## 📝 Notes

- We're using **existing** `components_spawner` - no new spawn mechanism needed!
- This is **phase 1** - just water collision
- Later we can add: friction, prop collision, tripping
- Keep it simple: one tile = one type (top layer wins)

## ✅ Success Criteria

When done, you should:
- ✅ Be able to walk on dirt, grass, yellow grass
- ✅ NOT be able to walk on water
- ✅ Player stops at water edge (doesn't enter water tile)
- ✅ No panic/crash when walking into water

---

**Ready? Let's start with [STEP_1_DATA_STRUCTURES.md](STEP_1_DATA_STRUCTURES.md)!**

