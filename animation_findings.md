# Player Animation Findings

## References Reviewed
- `bevy/examples/2d/sprite_sheet.rs`: canonical looping sprite-sheet animation that relies on an `AnimationIndices` component and a reusable `AnimationTimer`.
- `bevy/examples/2d/sprite_animation.rs`: event-driven animation that restarts timers via `run_if` input conditions and cleanly encapsulates FPS and frame windows in an `AnimationConfig`.

```rust
// bevy/examples/2d/sprite_sheet.rs
for (indices, mut timer, mut sprite) in &mut query {
    timer.tick(time.delta());

    if timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        atlas.index = if atlas.index == indices.last {
            indices.first
        } else {
            atlas.index + 1
        };
    }
}
```

## Current Implementation (`src/player.rs`)
- Movement and animation are decoupled across `move_player` and `animate_player`.
- A `Facing` enum drives row selection; each row is assumed to contain exactly `WALK_FRAMES` (9) frames.
- `AnimationState` tracks `moving` and `was_moving` flags so that `animate_player` can detect start/stop edges and snap to the first column when changing direction.

```rust
// src/player.rs
let target_row = row_zero_based(anim.facing);
let mut current_col = atlas.index % WALK_FRAMES;

if current_row != target_row {
    atlas.index = row_start_index(anim.facing);
    current_col = 0;
    timer.reset();
}
```

## Opportunities to Improve
- **Data-driven frame windows per facing:** Instead of hard-coding row math, mirror the example’s `AnimationIndices` so each `Facing` owns a `(first, last)` pair. That removes the `% WALK_FRAMES` math and lets us map multiple clips (idle, walk, run) without new helpers.

```rust
#[derive(Component)]
struct DirectionAnimations {
    clips: EnumMap<Facing, AnimationIndices>,
    active: Facing,
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut DirectionAnimations, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.get_single_mut() else { return; };
    timer.tick(time.delta());

    if timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas
    {
        let indices = anim.clips[anim.active];
        atlas.index = if atlas.index >= indices.last { indices.first } else { atlas.index + 1 };
    }
}
```

- **Expose FPS per clip:** The example’s `AnimationConfig::timer_from_fps` pattern would let us slow idle frames (e.g. 6 FPS) but keep walking snappy (10–12 FPS) without extra branching.
- **Run conditions for animation systems:** We can wrap `animate_player` in `.run_if(any_with_component::<Player>())` and optionally gate frame advancement behind `state.moving` to avoid ticking timers when idle.
- **Prep for richer states:** With direction clips isolated we can layer transitions (idle vs. walk, future attack) or swap sprite sheets without touching the animation system.
