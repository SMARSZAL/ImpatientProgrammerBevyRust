# Player Refactor Summary

## Why

- The original `src/player.rs` mixed constants, component definitions, and systems in a single 240+ line file.
- Animation logic used ad-hoc arithmetic on atlas indices, making it harder to add new clips or reuse code.
- Unused imports (`sprite::Anchor`) and lack of formatting/tests prompted a cleanup pass.

## What Changed

### Module Structure

- Replaced the monolithic `src/player.rs` with a directory-based module:
  - `src/player/components.rs` now owns player constants, components (`Player`, `Facing`, `AnimationState`, `AnimationTimer`), and the reusable `AnimationClip`/`DirectionalClips` helpers.
  - `src/player/systems.rs` contains the startup, movement, and animation systems plus `PlayerPlugin`.
  - `src/player/mod.rs` wires the submodules and re-exports only `PlayerPlugin`.

```rust
// Before: src/player.rs
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}

// After: src/player/mod.rs
mod components;
mod systems;

pub use systems::PlayerPlugin;
```

### Animation Data

- Introduced `AnimationClip` and `DirectionalClips::walk()` so per-facing frame windows are data-driven rather than computed with `% WALK_FRAMES`.
- `animate_player` now queries the directional clips and keeps atlas indices inside the active clip, snapping to the first frame when movement stops.

```rust
// Before: step through rows and columns manually
let target_row = row_zero_based(anim.facing);
let mut current_col = atlas.index % WALK_FRAMES;
if current_row != target_row {
    atlas.index = row_start_index(anim.facing);
    current_col = 0;
}

// After: use clip helpers
let clip = clips.clip(anim.facing);
if !clip.contains(atlas.index) {
    atlas.index = clip.start();
    timer.reset();
}
```

### Hygiene

- Removed unused `sprite::Anchor` import in `src/map/assets.rs`.
- Ran `cargo fmt --all` and `cargo check` to ensure formatting consistency and clean builds.

## Result

- Player animation data is centralized and ready for additional clips (idle/run/attack).
- `player` module is easier to navigate and maintain, while the external API (`PlayerPlugin`) stays unchanged for `main.rs`.
- Build warnings about unused imports are gone, and `cargo check` runs cleanly.*** End Patch
