# Collision Documentation Work Log

These notes describe the documentation pass completed for the collision subsystem. They capture what was added, where the information lives, and the reasoning behind each decision so future contributors can trace the workflow.

## Added Files
- `collision.md` — new top-level markdown document with a comprehensive overview of the collision architecture.

## Goals
- Produce a single reference that explains how collision data flows from procedural map generation to gameplay systems.
- Make it easier for teammates to reason about tile semantics, collision queries, and debug tooling without reading every source file.

## Implementation Details

### Source Analysis
- Reviewed the collision module (`src/collision/*`), procedural map generation (`src/map/generate.rs`, `src/map/assets.rs`), and the player systems (`src/player/systems.rs`) to extract behavior details.
- Took line-number snapshots while reviewing to ensure the documentation points to correct APIs and logic decisions.

### Documentation Structure (`collision.md`)
1. **Module inventory:** High-level map of relevant files to orient readers quickly.
2. **Tile typing pipeline:** Explanation of `TileType` semantics and how `TileMarker` components are attached during asset spawning.
3. **Collision map build process:** Step-by-step coverage of the `build_collision_map` system, including bounds detection, z-layer consolidation, and shoreline post-processing.
4. **Runtime API:** Detailed description of `CollisionMap` storage, point queries, circle clearance checks, and swept movement (`try_move_circle`).
5. **Gameplay integration:** How spawning and movement in `src/player/systems.rs` consume the collision map (feet-based offsets, collider radius, spiral search).
6. **Debug utilities:** Overview of F3 toggle, gizmo overlays, and logging helpers available in debug builds.
7. **Engine wiring:** Summary of how `main.rs` schedules the systems and ensures the collision resource is ready before gameplay logic runs.

### Rationale
- **Holistic view:** Developers inspecting bugs or planning features often need to know both how data is produced (map generation) and consumed (player movement). Keeping that context in one document reduces onboarding time.
- **Explaining heuristics:** Documented why the code inflates collider radius for certain tiles and how axis-aligned sliding works to preserve intended movement feel.
- **Debug discoverability:** Highlighted the conditional debug systems so new contributors know they exist and how to activate them (F3).
- **Future-proofing:** Noted that new tile types only require updating `TileMarker` assignments, clarifying where to extend the system.

### Verification
- Re-read the generated markdown (`sed -n '1,200p' collision.md`) to ensure clarity, accuracy, and formatting consistency.
- Cross-referenced the described behaviors with code snippets using `nl -ba` outputs to avoid stale references.

## Follow-up Ideas
- Capture animated GIFs or screenshots of the debug gizmos for the documentation once running the game.
- Expand the notes with performance considerations if larger maps or additional agents are introduced.
- Consider a similar write-up for the pickup subsystem to complete the documentation set.

