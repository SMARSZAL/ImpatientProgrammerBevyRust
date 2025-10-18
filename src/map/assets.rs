use crate::collision::{TileMarker, TileType};
use crate::inventory::{ItemKind, Pickable};
use crate::map::tilemap::TILEMAP;
use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

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
    /// The tile type for collision detection
    tile_type: Option<TileType>,
}

impl SpawnableAsset {
    pub fn new(sprite_name: &'static str) -> Self {
        Self {
            sprite_name,
            grid_offset: GridDelta::new(0, 0, 0),
            offset: Vec3::ZERO,
            components_spawner: |_| {}, // Default: no extra components
            tile_type: None,            // Default to None
        }
    }

    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }

    /// Builder method to set tile type for collision detection
    pub fn with_tile_type(mut self, tile_type: TileType) -> Self {
        self.tile_type = Some(tile_type);
        self
    }

    pub fn with_components_spawner(mut self, spawner: fn(&mut EntityCommands)) -> Self {
        self.components_spawner = spawner;
        self
    }

    pub fn with_pickable(self, kind: ItemKind) -> Self {
        match kind {
            ItemKind::TreeStump2 => self.with_components_spawner(add_tree_stump_2_pickup),
            ItemKind::Plant1 => self.with_components_spawner(add_plant_1_pickup),
            ItemKind::Plant2 => self.with_components_spawner(add_plant_2_pickup),
            ItemKind::Plant3 => self.with_components_spawner(add_plant_3_pickup),
            ItemKind::Plant4 => self.with_components_spawner(add_plant_4_pickup),
        }
    }
}

fn add_pickable(entity: &mut EntityCommands, kind: ItemKind) {
    entity.insert(Pickable::new(kind));
}

fn add_tree_stump_2_pickup(entity: &mut EntityCommands) {
    add_pickable(entity, ItemKind::TreeStump2);
}

fn add_plant_1_pickup(entity: &mut EntityCommands) {
    add_pickable(entity, ItemKind::Plant1);
}

fn add_plant_2_pickup(entity: &mut EntityCommands) {
    add_pickable(entity, ItemKind::Plant2);
}

fn add_plant_3_pickup(entity: &mut EntityCommands) {
    add_pickable(entity, ItemKind::Plant3);
}

fn add_plant_4_pickup(entity: &mut EntityCommands) {
    add_pickable(entity, ItemKind::Plant4);
}

#[derive(Clone)]
pub struct TilemapHandles {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TilemapHandles {
    pub fn sprite(&self, atlas_index: usize) -> Sprite {
        Sprite::from_atlas_image(
            self.image.clone(),
            TextureAtlas::from(self.layout.clone()).with_index(atlas_index),
        )
    }
}

pub fn prepare_tilemap_handles(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets_directory: &str,
    tilemap_file: &str,
) -> TilemapHandles {
    let image = asset_server.load::<Image>(format!("{assets_directory}/{tilemap_file}"));
    let mut layout = TextureAtlasLayout::new_empty(TILEMAP.atlas_size());
    for index in 0..TILEMAP.sprites.len() {
        layout.add_texture(TILEMAP.sprite_rect(index));
    }
    let layout = atlas_layouts.add(layout);

    TilemapHandles { image, layout }
}

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
                tile_type,
            } = asset_def;

            let Some(atlas_index) = TILEMAP.sprite_index(sprite_name) else {
                panic!("Unknown atlas sprite '{}'", sprite_name);
            };

            // Get the appropriate spawner based on tile type
            let spawner: fn(&mut EntityCommands) = if let Some(tile_ty) = tile_type {
                // Create a spawner function for this specific tile type
                match tile_ty {
                    TileType::Dirt => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Dirt,
                        });
                    },
                    TileType::Grass => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Grass,
                        });
                    },
                    TileType::YellowGrass => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::YellowGrass,
                        });
                    },
                    TileType::Water => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Water,
                        });
                    },
                    TileType::Shore => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Shore,
                        });
                    },
                    TileType::Tree => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Tree,
                        });
                    },
                    TileType::Rock => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Rock,
                        });
                    },
                    TileType::Empty => |entity: &mut EntityCommands| {
                        entity.insert(TileMarker {
                            tile_type: TileType::Empty,
                        });
                    },
                }
            } else {
                components_spawner
            };

            models_assets.add(
                model_index,
                ModelAsset {
                    assets_bundle: tilemap_handles.sprite(atlas_index),
                    grid_offset,
                    world_offset: offset,
                    spawn_commands: spawner,
                },
            )
        }
    }
    models_assets
}
