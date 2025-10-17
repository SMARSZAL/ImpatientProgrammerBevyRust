# Step 2: Tag Tiles During Spawn

## 🎯 Goal
Modify the tile spawning code to attach `TileTypeMarker` components using the **existing** `components_spawner` mechanism.

## 💡 Key Insight

We already have this in `SpawnableAsset`:
```rust
components_spawner: fn(&mut EntityCommands),
```

We'll use this to attach our `TileTypeMarker` component! No new spawning mechanism needed.

## 📝 Implementation

### 2.1 Modify `src/map/assets.rs`

We need to:
1. Add `tile_type` field to `SpawnableAsset`
2. Store grid position with the asset
3. Create a helper to generate the spawner function

#### Add imports at the top:
```rust
// At the top of src/map/assets.rs, add:
use crate::map::{TileType, TileTypeMarker};
```

#### Modify `SpawnableAsset` struct:

```rust
// Find the SpawnableAsset struct and update it:
#[derive(Clone)]
pub struct SpawnableAsset {
    /// Name of the sprite inside our tilemap atlas
    sprite_name: &'static str,
    /// Offset in grid coordinates (for multi-tile objects)
    grid_offset: GridDelta,
    /// Offset in world coordinates (fine positioning)
    offset: Vec3,
    /// Function to add custom components (like collision, physics, etc.)
    components_spawner: fn(&mut EntityCommands),
    /// NEW: The tile type for collision detection
    tile_type: Option<TileType>,
}
```

#### Update `SpawnableAsset::new()`:

```rust
impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            components_spawner: |_| {}, // Default: no extra components
            tile_type: None,  // NEW: Default to None
        }
    }

    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }
    
    // NEW: Builder method to set tile type
    pub fn with_tile_type(mut self, tile_type: TileType) -> Self {
        self.tile_type = Some(tile_type);
        self
    }
}
```

#### Update `load_assets()` function:

Find the `load_assets` function and modify the loop to attach the marker:

```rust
// Find this function and update it:
pub fn load_assets(
    tilemap_handles: &TilemapHandles,
    assets_definitions: Vec<Vec<SpawnableAsset>>,
) -> ModelsAssets<Sprite> {
    let mut models_assets = ModelsAssets::<Sprite>::new();
    for (model_index, assets) in assets_definitions.into_iter().enumerate() {
        for asset_def in assets {
            let SpawnableAsset {
                sprite_name,
                grid_offset,
                offset,
                components_spawner,
                tile_type,  // NEW: Extract tile_type
            } = asset_def;

            let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
                panic!("Unknown atlas sprite '{}'", sprite_name);
            };

            // NEW: Create a spawner that includes the marker if we have a tile type
            let enhanced_spawner: fn(&mut EntityCommands) = match tile_type {
                Some(tile_ty) => {
                    // Create a new spawner that:
                    // 1. Calls the original spawner
                    // 2. Adds the TileTypeMarker
                    move |entity: &mut EntityCommands| {
                        components_spawner(entity);
                        
                        // Add the marker component with tile type
                        // We'll set grid_position later when we know the actual position
                        entity.insert(TileTypeMarker {
                            tile_type: tile_ty,
                            grid_position: IVec3::ZERO, // Will be set correctly by the spawner
                        });
                    }
                }
                None => components_spawner,
            };

            models_assets.add(
                model_index,
                ModelAsset {
                    assets_bundle: tilemap_handles.sprite(atlas_index),
                    grid_offset,
                    world_offset: offset,
                    spawn_commands: enhanced_spawner,  // Use enhanced spawner
                },
            )
        }
    }
    models_assets
}
```

### 2.2 Modify `src/map/rules.rs`

Now we tag each tile when we create it. This is straightforward - just add `.with_tile_type()` to each `SpawnableAsset::new()` call.

#### Add import at top:
```rust
// At the top of src/map/rules.rs, add:
use crate::map::TileType;
```

#### Tag terrain tiles:

Find each terrain tile creation and add the tile type:

```rust
// In build_dirt_layer() around line 12-24:
terrain_model_builder
    .create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.dirt.material,
            x_neg: terrain_sockets.dirt.material,
            z_pos: terrain_sockets.dirt.layer_up,
            z_neg: terrain_sockets.dirt.layer_down,
            y_pos: terrain_sockets.dirt.material,
            y_neg: terrain_sockets.dirt.material,
        },
        vec![SpawnableAsset::new("dirt").with_tile_type(TileType::Dirt)],  // <-- ADD
    )
    .with_weight(20.);
```

```rust
// In build_grass_layer() around line 184-199:
terrain_model_builder
    .create_model(
        SocketsCartesian3D::Multiple {
            x_pos: vec![terrain_sockets.grass.material],
            x_neg: vec![terrain_sockets.grass.material],
            z_pos: vec![
                terrain_sockets.grass.layer_up,
                terrain_sockets.grass.grass_fill_up,
            ],
            z_neg: vec![terrain_sockets.grass.layer_down],
            y_pos: vec![terrain_sockets.grass.material],
            y_neg: vec![terrain_sockets.grass.material],
        },
        vec![SpawnableAsset::new("green_grass").with_tile_type(TileType::Grass)],  // <-- ADD
    )
    .with_weight(5.);
```

```rust
// In build_yellow_grass_layer() around line 51-63:
terrain_model_builder
    .create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.grass.material,
            x_neg: terrain_sockets.grass.material,
            z_pos: terrain_sockets.yellow_grass.layer_up,
            z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
            y_pos: terrain_sockets.grass.material,
            y_neg: terrain_sockets.grass.material,
        },
        vec![SpawnableAsset::new("yellow_grass").with_tile_type(TileType::YellowGrass)],  // <-- ADD
    )
    .with_weight(5.);
```

```rust
// In build_water_layer() around line 327-342:
terrain_model_builder
    .create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.water.material,
            x_neg: terrain_sockets.water.material,
            z_pos: terrain_sockets.water.layer_up,
            z_neg: terrain_sockets.water.layer_down,
            y_pos: terrain_sockets.water.material,
            y_neg: terrain_sockets.water.material,
        },
        vec![SpawnableAsset::new("water").with_tile_type(TileType::Water)],  // <-- ADD THIS IS THE KEY ONE!
    )
    .with_weight(10. * WATER_WEIGHT);
```

#### Tag grass/water edge tiles:

**Important**: Edge tiles (corners, sides) should get the same type as their main tile:

```rust
// Yellow grass edges - all should be YellowGrass type
vec![SpawnableAsset::new("yellow_grass_corner_out_tl").with_tile_type(TileType::YellowGrass)]
vec![SpawnableAsset::new("yellow_grass_side_t").with_tile_type(TileType::YellowGrass)]
// etc...

// Water edges - all should be Water type (so you can't walk on water edges!)
vec![SpawnableAsset::new("water_corner_out_tl").with_tile_type(TileType::Water)]
vec![SpawnableAsset::new("water_side_t").with_tile_type(TileType::Water)]
// etc...

// Grass edges - all should be Grass type
vec![SpawnableAsset::new("green_grass_corner_out_tl").with_tile_type(TileType::Grass)]
vec![SpawnableAsset::new("green_grass_side_t").with_tile_type(TileType::Grass)]
// etc...
```

#### Tag props:

```rust
// In build_props_layer(), tag the props:

// Trees are not walkable
vec![
    SpawnableAsset::new("small_tree_bottom").with_tile_type(TileType::Tree),
    SpawnableAsset::new("small_tree_top").with_grid_offset(GridDelta::new(0, 1, 0)),
]

// Big trees
vec![
    SpawnableAsset::new("big_tree_1_bl").with_tile_type(TileType::Tree),
    SpawnableAsset::new("big_tree_1_tl").with_grid_offset(GridDelta::new(0, 1, 0)),
]

// Rocks not walkable
vec![SpawnableAsset::new("rock_1").with_tile_type(TileType::Rock)]
vec![SpawnableAsset::new("rock_2").with_tile_type(TileType::Rock)]
// ... etc

// Plants and stumps ARE walkable (you can walk through small plants)
vec![SpawnableAsset::new("plant_1").with_tile_type(TileType::Plant)]
vec![SpawnableAsset::new("tree_stump_1").with_tile_type(TileType::Stump)]
// ... etc
```

## 📋 Checklist

Go through `rules.rs` and add `.with_tile_type()` to:

- [ ] Dirt tile (TileType::Dirt)
- [ ] All green grass tiles (TileType::Grass) - main + all 12 edges/corners
- [ ] All yellow grass tiles (TileType::YellowGrass) - main + all 12 edges/corners  
- [ ] **All water tiles (TileType::Water) - main + all 12 edges/corners** ⚠️ CRITICAL
- [ ] Small tree (TileType::Tree)
- [ ] Big tree 1 (TileType::Tree)
- [ ] Big tree 2 (TileType::Tree)
- [ ] All rocks (TileType::Rock)
- [ ] All plants (TileType::Plant)
- [ ] All stumps (TileType::Stump)

**Note**: For multi-tile objects (like trees with top/bottom), only tag the **bottom** tile. The top part doesn't need a marker.

## ✅ Verification

### Step 1: Compile Check

```bash
cargo build
```

**Expected**: Should compile! You might get warnings about unused `TileTypeMarker` - that's fine, we'll use it in Step 3.

### Step 2: Check one model visually

Add a temporary debug print in `load_assets`:

```rust
// In load_assets(), after creating enhanced_spawner:
if let Some(ty) = tile_type {
    println!("Tagged {} as {:?}", sprite_name, ty);
}
```

Then run the game:

```bash
cargo run
```

**Expected**: You should see output like:
```
Tagged dirt as Dirt
Tagged green_grass as Grass
Tagged water as Water
Tagged rock_1 as Rock
...
```

This confirms tiles are being tagged!

### Step 3: Count water tiles

Add a counter:

```rust
// In load_assets(), before the function:
let mut water_count = 0;

// Then in the loop:
if tile_type == Some(TileType::Water) {
    water_count += 1;
}

// After the loop:
println!("Total water tiles tagged: {}", water_count);
```

**Expected**: Should see "Total water tiles tagged: 13" (1 main + 8 corners + 4 sides)

## 🎓 What We Learned

- ✅ Used existing `components_spawner` mechanism
- ✅ No new spawn system needed!
- ✅ Tagged tiles at the source (during model creation)
- ✅ Water tiles (including edges) are marked as Water
- ✅ Props are marked appropriately

## 🐛 Common Issues

**Issue**: "cannot find type `TileType` in this scope" in rules.rs
- **Fix**: Add `use crate::map::TileType;` at top of rules.rs

**Issue**: Compile error about `tile_type` field not found
- **Fix**: Make sure you added `tile_type: Option<TileType>` to `SpawnableAsset` struct

**Issue**: Not all water tiles are tagged
- **Fix**: Remember to tag ALL water variants (corners, sides, not just main tile)

## ➡️ Next Step

Once tiles are being tagged during spawn:

**👉 Continue to [STEP_3_BUILD_MAP.md](STEP_3_BUILD_MAP.md)**

We'll build the `Map` resource from these tagged entities!

