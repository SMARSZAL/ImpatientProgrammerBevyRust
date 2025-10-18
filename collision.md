# Collision System

This Bevy project tracks traversable space through a custom collision subsystem that sits alongside the procedural tilemap generator. The following sections describe the data model, how the collision map is constructed, how gameplay systems query it, and which debug tools are available when working on the feature.

## Modules at a Glance
- `src/collision/mod.rs` exposes the public API and re-exports the pieces described below.
- `src/collision/tile_types.rs` defines `TileType` and the `TileMarker` component that tags spawned tiles with their collision semantics.
- `src/collision/map.rs` implements the `CollisionMap` resource and the helper methods used to test walkability and resolve movement.
- `src/collision/debug.rs` (compiled only in debug builds) provides gizmo overlays and logging helpers.
- `src/map/assets.rs` attaches `TileMarker` components to spawned sprites.
- `src/map/generate.rs` builds the `CollisionMap` once tiles are generated, performs post-processing, and registers the resource.
- `src/player/systems.rs` is the primary consumer: both player spawning and per-frame movement rely on the collision map.

## Tile Typing Pipeline

### Tile Definitions
`TileType` enumerates every tile category the collision system cares about (`Dirt`, `Grass`, `YellowGrass`, `Shore`, `Empty`, `Water`, `Tree`, `Rock`). The helper `is_walkable` treats water and prop tiles (`Tree`, `Rock`) as blocking while everything else is navigable (`src/collision/tile_types.rs`).

### Marker Components on Spawned Tiles
Procedural generation produces many sprites. When each sprite is instantiated, the asset loader injects a `TileMarker` component that stores the `TileType` corresponding to that sprite (`src/map/assets.rs`). This metadata is how the collision builder later understands the spatial layout without needing to inspect textures or sprite indices.

## Building the Collision Map

### Scheduling and Guards
The `build_collision_map` system runs in Bevy’s `Update` schedule once the procedural generator has spawned tiles. It is guarded by the `CollisionMapBuilt` resource to ensure work happens exactly once and to avoid re-running the expensive query on subsequent frames (`src/map/generate.rs`).

### Determining Map Bounds
Spawned tiles are read via `Query<(&TileMarker, &Transform)>`. Because the procedural generator can place elements outside the nominal `GRID_X × GRID_Y` bounds, the system computes the actual min/max grid coordinates by projecting each tile’s world transform back onto the logical grid (`grid_origin_x/y` and `TILE_SIZE` are shared with the renderer). The collision map is then created with `CollisionMap::with_origin(actual_width, actual_height, TILE_SIZE, grid_origin_x, grid_origin_y)`, ensuring both the dimensions and origin match what exists in the scene.

### Layer Consolidation
Tiles arrive in multiple vertical layers (terrain, water, props, etc.). The collision builder folds them into a 2D representation by tracking the highest `Transform::translation.z` at each `(x, y)` coordinate. Only the topmost tile contributes to collision, ensuring that, for example, a water tile sitting above dirt marks the terrain as non-walkable. After layer reduction, the chosen `TileType` value is written into the `CollisionMap.tiles` vector using local indices that start at `(min_x, min_y)`.

### Post-processing Shores
Once every grid cell carries its top tile, `convert_water_edges_to_shore` scans the map and converts water tiles that touch any walkable neighbor (including diagonals) into `Shore`. This step creates natural coastlines that the player can traverse, turning hard water boundaries into walkable beaches. The function operates on a temporary list to avoid mutating in-place while still inspecting neighbors.

### Resource Registration
After post-processing completes, the populated `CollisionMap` is inserted as a Bevy resource. From that point on it is accessible to any system via `Res<CollisionMap>` or `Option<Res<CollisionMap>>`. Debug logging during the build step reports tile counts and the final walkable/unwalkable split, which is useful for validating generation output.

## CollisionMap Resource API

The resource stores:
- `tiles: Vec<TileType>` — flattened grid in row-major order.
- `width` and `height` — grid dimensions in tiles.
- `tile_size` — world units per tile (64.0).
- `grid_origin_x`/`grid_origin_y` — world coordinates of the grid’s bottom-left corner.

Key methods in `src/collision/map.rs`:

```rust
pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2
pub fn is_walkable(&self, x: i32, y: i32) -> bool
pub fn is_world_pos_walkable(&self, world_pos: Vec2) -> bool
```

These provide quick coordinate conversions and single-point walkability tests. For agent movement the code uses the circle-oriented helpers:

### Circle Clearance (`is_world_pos_clear_circle`)
- Rejects immediately if the proposed circle would extend outside map bounds (`is_world_pos_within_bounds`).
- If the radius is zero (point query), it devolves to `is_world_pos_walkable`.
- Calculates the AABB of tiles that might intersect the circle and iterates over them.
- For non-walkable tiles, it inflates the test radius slightly based on tile type: shores get 10% extra padding, trees and rocks get 5%, everything else uses the raw radius. This keeps motion feeling natural—players can skim past props with a bit of leeway while still respecting water boundaries.
- Performs an exact circle-vs-AABB check by projecting the circle center onto the tile rectangle and comparing squared distances.

The method returns `true` only if the circle avoids every blocking tile in its search window.

### Swept Movement (`try_move_circle`)
- Accepts a start and desired end position (both interpreted as circle centers) plus the collider radius.
- Divides the motion delta into sub-steps no larger than 25% of a tile to avoid tunneling through thin obstacles, then marches one step at a time.
- At each step it tries to place the circle at the full candidate position. If blocked, it attempts axis-aligned sliding—first preserving Y, then preserving X—to emulate sliding along walls.
- Stops stepping once motion is blocked on both axes or the destination is reached.

This routine returns the furthest legal position the agent can reach this frame and is used both for player spawning and per-frame motion.

## Player Interaction

### Spawning
`spawn_player` waits until `CollisionMap` exists before creating the player entity. It calls `find_walkable_spawn_position`, which:
1. Tests the map center first, using the player’s **feet** position (sprite center minus half the scaled sprite height) to align collision with the bottom of the sprite.
2. If blocked, spirals outward looking for the nearest walkable tile whose surrounding area passes the circle clearance check (radius 16.0 world units).
3. Falls back to random samples or ultimately the center if no valid spot is found.

This ensures the player never spawns inside walls or water.

### Movement
The `move_player` system reads cursor keys, computes a desired velocity, and constructs candidate positions for the player’s feet. It feeds the current and desired foot positions plus the collision radius (16.0) into `CollisionMap::try_move_circle`. The returned feet position is converted back to sprite center coordinates before applying to the `Transform`. If the collider could not advance (because of blocking geometry) the system simply stops the movement animation that frame.

This approach combines continuous-time motion with tile-based collision checks and automatically handles sideways sliding on obstacles thanks to the axis test inside `try_move_circle`.

## Debug Instrumentation (Debug Builds Only)

The collision module exposes several utilities when `cfg(debug_assertions)` is active:
- `DebugCollisionEnabled` resource toggled via the F3 key.
- `debug_draw_collision` renders semi-transparent rectangles over every tile (green for walkable, red for blocking) and adds an on-screen legend.
- `debug_player_position` draws the player’s world position, the actual collider circle (radius 24.0 for visibility), the underlying grid cell, and an overlay if the player is standing on an unwalkable tile.
- `debug_log_tile_info` logs tile type and walkability whenever the player moves to a new grid cell.

These helpers make it easy to cross-check the collision map against the rendered map during development.

## Engine Integration

`main.rs` wires everything together:
- At startup, the game initializes rendering and the procedural generator.
- During `Update`, `build_collision_map` runs alongside camera and fog systems.
- The `PlayerPlugin` (registered before gameplay) depends on the collision map for spawn and per-frame movement, and will simply idle until the resource is available.

Because the collision map is derived from the same assets used to render the world, any future additions (new props, obstacles, or terrain types) only need to assign the appropriate `TileType` in the asset definitions. The existing build step and movement logic will automatically pick up the new semantics.

