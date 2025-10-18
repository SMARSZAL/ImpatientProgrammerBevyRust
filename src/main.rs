mod collision;
mod map;
mod player;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowMode, MonitorSelection},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
    camera::Projection,
};
use bevy_procedural_tilemaps::prelude::*;

use crate::map::generate::{setup_generator, build_collision_map, CollisionMapBuilt};
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use crate::collision::{DebugCollisionEnabled, toggle_debug_collision, debug_draw_collision, debug_player_position, debug_log_tile_info};

#[derive(Component)]
struct CameraFollow;

#[derive(Component)]
struct FogOfWar;

// Custom material for circular fog of war vision
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CircularFogMaterial {
    #[uniform(0)]
    player_pos: Vec2,
    #[uniform(0)]
    vision_radius: f32,
}

impl Material2d for CircularFogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circular_fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Resource)]
struct VisionRadius(f32);

fn main() {
    let vision_radius = 320.0;

    let mut app = App::new();
    
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(VisionRadius(vision_radius))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Game".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            Material2dPlugin::<CircularFogMaterial>::default(),
            ProcGenSimplePlugin::<Cartesian3D, Sprite>::default(),
            PlayerPlugin,
        ))
        .init_resource::<CollisionMapBuilt>()
        .add_systems(Startup, (setup_camera, setup_generator, setup_fog_of_war))
        .add_systems(Update, (build_collision_map, follow_player_and_fog, update_player_depth, configure_camera_projection, debug_tile_depths));

    // Debug systems - only in debug builds
    #[cfg(debug_assertions)]
    {
        app.init_resource::<DebugCollisionEnabled>()
            .add_systems(Update, (
                toggle_debug_collision,
                debug_draw_collision,
                debug_player_position,
                debug_log_tile_info,
            ));
    }

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), CameraFollow));
}

/// System to update player depth based on Y position to match tilemap Z system
/// This mirrors the same Z-depth calculation that bevy_procedural_tilemaps uses
/// with with_z_offset_from_y(true)
fn update_player_depth(mut player_query: Query<&mut Transform, With<crate::player::Player>>) {
    for mut transform in player_query.iter_mut() {
        let player_y_world = transform.translation.y;
        let old_z = transform.translation.z;
        
        // Map configuration (from generate.rs)
        const TILE_SIZE: f32 = 64.0;
        const GRID_Y: u32 = 18;
        
        // Based on debug output: tiles have Z range 0.556 to 5.444
        // Let's use a similar range for the player
        let map_height = TILE_SIZE * GRID_Y as f32;
        let map_y0 = -TILE_SIZE * GRID_Y as f32 / 2.0; // Map origin Y (from generate.rs)
        
        // Normalize player Y to [0, 1] across the whole grid height
        let t = ((player_y_world - map_y0) / map_height).clamp(0.0, 1.0);
        
        // Use a Z range similar to tiles (0.556 to 5.444) but slightly higher to draw in front
        let min_z = 0.556;
        let max_z = 5.444;
        let player_z = min_z + (max_z - min_z) * (1.0 - t) + 0.1; // +0.1 to draw above tiles
        
        transform.translation.z = player_z;
        
        // Debug log every 60 frames (about once per second at 60fps)
        static mut FRAME_COUNT: u32 = 0;
        unsafe {
            FRAME_COUNT += 1;
            if FRAME_COUNT % 60 == 0 {
                info!("🎮 Player depth debug - Y: {:.1}, Old Z: {:.3}, New Z: {:.3}, t: {:.3}, map_y0: {:.1}, map_height: {:.1}", 
                      player_y_world, old_z, player_z, t, map_y0, map_height);
            }
        }
    }
}

/// System to configure camera projection to prevent Z-depth culling issues
fn configure_camera_projection(
    mut camera_query: Query<&mut Projection, (With<Camera2d>, With<CameraFollow>)>,
) {
    for mut projection in camera_query.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            // Widen the camera's clip range to prevent objects from being culled
            // This makes debugging less brittle and prevents Z-depth issues
            ortho.near = -2000.0;
            ortho.far = 2000.0;
        }
    }
}

/// Debug system to show tile Z values to understand the depth system
fn debug_tile_depths(
    tile_query: Query<(&Transform, &crate::collision::TileMarker)>,
) {
    // Debug log every 300 frames (about once per 5 seconds at 60fps)
    static mut FRAME_COUNT: u32 = 0;
    unsafe {
        FRAME_COUNT += 1;
        if FRAME_COUNT % 300 == 0 {
            let mut tile_count = 0;
            let mut min_z = f32::MAX;
            let mut max_z = f32::MIN;
            let mut sample_tiles: Vec<(f32, f32, String)> = Vec::new(); // (Y, Z, Type)
            
            for (transform, tile_marker) in tile_query.iter() {
                tile_count += 1;
                let z = transform.translation.z;
                min_z = min_z.min(z);
                max_z = max_z.max(z);
                
                // Collect first 10 tiles as samples
                if sample_tiles.len() < 10 {
                    sample_tiles.push((
                        transform.translation.y,
                        z,
                        format!("{:?}", tile_marker.tile_type)
                    ));
                }
            }
            
            if tile_count > 0 {
                info!("🗺️ Tile depth debug - {} tiles, Z range: {:.3} to {:.3}", 
                      tile_count, min_z, max_z);
                info!("🗺️ Sample tiles (Y, Z, Type):");
                for (y, z, tile_type) in sample_tiles {
                    info!("   Y: {:.1}, Z: {:.3}, Type: {}", y, z, tile_type);
                }
            }
        }
    }
}

fn setup_fog_of_war(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CircularFogMaterial>>,
    vision_radius: Res<VisionRadius>,
) {
    let mesh = meshes.add(Rectangle::new(5000.0, 5000.0));
    let material = materials.add(CircularFogMaterial {
        player_pos: Vec2::ZERO,
        vision_radius: vision_radius.0,
    });
    
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 900.0)),
        FogOfWar,
    ));
}

fn follow_player_and_fog(
    player_query: Query<&Transform, With<crate::player::Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<crate::player::Player>, Without<FogOfWar>)>,
    mut fog_query: Query<(&mut Transform, &MeshMaterial2d<CircularFogMaterial>), (With<FogOfWar>, Without<Camera2d>, Without<crate::player::Player>)>,
    mut materials: ResMut<Assets<CircularFogMaterial>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y);

    // Update camera with smooth following
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        let lerp_speed = 0.1;
        camera_transform.translation.x += (player_pos.x - camera_transform.translation.x) * lerp_speed;
        camera_transform.translation.y += (player_pos.y - camera_transform.translation.y) * lerp_speed;
        
        // Snap to pixel boundaries for crisp rendering
        camera_transform.translation.x = camera_transform.translation.x.round();
        camera_transform.translation.y = camera_transform.translation.y.round();
        camera_transform.translation.z = 1000.0;
    }

    // Update fog of war overlay
    if let Ok((mut fog_transform, material_handle)) = fog_query.single_mut() {
        fog_transform.translation.x = player_pos.x;
        fog_transform.translation.y = player_pos.y;
        fog_transform.translation.z = 900.0;

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.player_pos = player_pos;
        }
    }
}
