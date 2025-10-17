# Collision Setup - Complete Guide Index

## 📚 Documentation Overview

This folder contains everything you need to implement water collision detection in your Bevy game.

---

## 🎯 Start Here

**[README.md](README.md)** - Overview, architecture, and getting started

---

## 📖 Implementation Guides (Follow in Order)

1. **[STEP_1_DATA_STRUCTURES.md](STEP_1_DATA_STRUCTURES.md)**
   - Create TileType enum
   - Create Map resource
   - Setup basic collision structures
   - **Time**: ~15 minutes
   - **Result**: Data structures compile and pass tests

2. **[STEP_2_TAG_TILES.md](STEP_2_TAG_TILES.md)**
   - Modify SpawnableAsset to include tile type
   - Tag tiles during model creation using existing `components_spawner`
   - **Time**: ~30 minutes
   - **Result**: Tiles are tagged with TileTypeMarker components

3. **[STEP_3_BUILD_MAP.md](STEP_3_BUILD_MAP.md)**
   - Create system to build collision map from spawned tiles
   - Convert tile positions to grid coordinates
   - **Time**: ~20 minutes
   - **Result**: Map resource built and available, logs show tile counts

4. **[STEP_4_PLAYER_COLLISION.md](STEP_4_PLAYER_COLLISION.md)**
   - Modify player movement to check collision map
   - Block movement into unwalkable tiles
   - **Time**: ~15 minutes
   - **Result**: Player cannot walk on water! ✨

**Total Time**: ~1.5 hours

---

## 🔧 Support Guides

### For When Things Go Wrong

**[TROUBLESHOOTING.md](TROUBLESHOOTING.md)**
- Common issues and their solutions
- Compilation errors
- Runtime problems
- Verification techniques
- Debug checklist

### For Understanding What's Happening

**[DEBUG_HELPERS.md](DEBUG_HELPERS.md)**
- Visual debug systems (gizmos, overlays)
- Console debug systems (logging, stats)
- Testing helpers (teleport, force positions)
- Performance warnings

### For Quick Lookups

**[QUICK_REFERENCE.md](QUICK_REFERENCE.md)**
- Condensed cheat sheet
- Key code snippets
- File change summary
- Testing checklist
- Common patterns

---

## 📊 Guide Characteristics

### Step-by-Step Guides
- ✅ Complete code snippets (copy-paste ready)
- ✅ Explanation of what each part does
- ✅ Verification steps after each change
- ✅ "What you should see" expected outputs
- ✅ Common pitfalls for each step

### Support Guides
- 🐛 Issue → Cause → Solution format
- 🔍 Debug tools with examples
- 📋 Checklists and quick lookups
- 💡 Best practices and tips

---

## 🗂️ File Structure

```
collision_setup/
├── INDEX.md                      ← You are here
├── README.md                     ← Start here
├── STEP_1_DATA_STRUCTURES.md    ← First step
├── STEP_2_TAG_TILES.md           ← Second step
├── STEP_3_BUILD_MAP.md           ← Third step
├── STEP_4_PLAYER_COLLISION.md   ← Final step
├── TROUBLESHOOTING.md            ← When stuck
├── DEBUG_HELPERS.md              ← Debug tools
└── QUICK_REFERENCE.md            ← Quick lookup
```

---

## 🎯 Learning Path

### If you're new to the codebase:
1. Read README.md for architecture overview
2. Follow Step 1-4 sequentially
3. Use debug helpers to visualize
4. Check troubleshooting if issues arise

### If you're experienced:
1. Skim README.md for approach
2. Use QUICK_REFERENCE.md for code snippets
3. Implement all steps at once
4. Use DEBUG_HELPERS.md to verify

### If something breaks:
1. Check which step you're on
2. Go to TROUBLESHOOTING.md
3. Find your issue (or similar)
4. Apply the solution
5. Use debug helpers to verify

---

## 📦 What Gets Created

### New Files (you'll create these):
```
src/map/tile_marker.rs      ~60 lines
src/map/collision.rs         ~70 lines
```

### Modified Files:
```
src/map/mod.rs              +3 lines (exports)
src/map/assets.rs           +20 lines (tile_type field)
src/map/rules.rs            +40 lines (tagging)
src/map/generate.rs         +50 lines (builder system)
src/player/systems.rs       +10 lines (collision check)
src/main.rs                 +3 lines (system registration)
```

**Total impact**: ~256 lines added/modified across 8 files

---

## ✅ Success Criteria

When you've successfully implemented collision:

- ✅ Code compiles without errors
- ✅ Game runs without panicking
- ✅ Console shows "Collision map built!" message
- ✅ Player can walk on dirt/grass/yellow grass
- ✅ Player **cannot** walk on water
- ✅ Player stops cleanly at water edge
- ✅ Walk animation stops when blocked
- ✅ Can walk parallel to water (not diagonal blocking)

---

## 🚀 After Completion

Once water collision works, you can extend with:

- **Friction**: Different speeds on different terrain (easy)
- **Sound effects**: Play sound when hitting obstacle (easy)
- **Visual feedback**: Screen shake, particles (medium)
- **Tree/rock collision**: Already works! (they're marked non-walkable)
- **Tripping mechanic**: Random chance on certain tiles (medium)
- **Advanced movement**: Sliding along walls (hard)

---

## 📝 Notes

### Design Decisions

This implementation:
- ✅ Uses **existing** `components_spawner` mechanism (no new spawn system)
- ✅ Follows **roguelike tutorial** pattern (Vec-based, simple)
- ✅ Builds map **once** (efficient, not every frame)
- ✅ Uses **top layer wins** rule (tree on grass = non-walkable)
- ✅ Is **tile-based** (not continuous physics)
- ✅ Checks **center point** of player (not corners/edges)

### Why No Avian Physics?

For this use case:
- ❌ Overkill (we just need yes/no walkable)
- ❌ Requires refactoring tile spawn
- ❌ Adds runtime overhead
- ❌ Complex integration with WFC
- ✅ Custom solution is simpler
- ✅ Custom solution is faster
- ✅ Direct control over behavior

Physics engines are great for:
- Dynamic objects interacting
- Realistic momentum/forces
- Complex collision shapes
- Ragdoll physics

But our needs are simple: "Can I walk here?"

---

## 💬 Feedback

This guide is designed to be:
- **Self-contained**: Everything you need is here
- **Verified**: Each step has a verification section
- **Debuggable**: Multiple tools to see what's happening
- **Extensible**: Easy to add more features later

If you get stuck or find unclear instructions, check TROUBLESHOOTING.md first!

---

## 🎓 Key Concepts Explained

### Tile-Based Collision
- World divided into grid of tiles
- Each tile has a type (water, grass, etc.)
- Type determines walkability
- O(1) lookup (very fast!)

### Component-Based Tagging
- Tiles spawn with marker component
- Marker stores tile type
- Can query all marked tiles
- ECS handles the heavy lifting

### Resource-Based Queries
- Map stored as ECS resource
- Available everywhere
- Single source of truth
- No global variables needed

### System Ordering
- Generation → Tagging → Building → Checking
- Each phase depends on previous
- Flags prevent re-running
- Efficient and clean

---

**Ready to implement? Start with [README.md](README.md)!**

