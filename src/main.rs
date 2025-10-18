mod camera;
mod collision;
mod inventory;
mod map;
mod player;

use bevy::{
    prelude::*,
    sprite_render::Material2dPlugin,
    window::{MonitorSelection, Window, WindowMode, WindowPlugin},
};
use bevy_procedural_tilemaps::prelude::*;

use crate::camera::fog::{CircularFogMaterial, VisionRadius, follow_fog, setup_fog_of_war};
use crate::camera::rendering::update_player_depth;
use crate::camera::{configure_camera_projection, follow_camera, setup_camera};
use crate::inventory::Inventory;
use crate::map::generate::{CollisionMapBuilt, build_collision_map, setup_generator};
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use crate::collision::{
    DebugCollisionEnabled, debug_draw_collision, debug_log_tile_info, debug_player_position,
    toggle_debug_collision,
};

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
        .init_resource::<Inventory>()
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_generator,
                setup_fog_of_war,
                configure_camera_projection,
            ),
        )
        .add_systems(
            Update,
            (
                build_collision_map,
                follow_camera,
                follow_fog,
                update_player_depth,
            ),
        );

    // Debug systems - only in debug builds
    #[cfg(debug_assertions)]
    {
        app.init_resource::<DebugCollisionEnabled>().add_systems(
            Update,
            (
                toggle_debug_collision,
                debug_draw_collision,
                debug_player_position,
                debug_log_tile_info,
            ),
        );
    }

    app.run();
}
