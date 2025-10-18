# Chapter 25 — Appendix: Miscellaneous Techniques

A few sharp tools refuse to fit any single chapter. This appendix highlights on-the-fly asset mutation—tweak meshes or textures after they spawn—so you always have a reference when pipelines demand runtime edits. Each section links directly to the example that inspired it.

## Runtime Asset Mutation

### Deep Dive: Live Cosmetic Swaps
`examples/asset/alter_sprite.rs` exposes direct access to texture data, letting Frostbite Skirmish recolor squads mid-replay without duplicating spritesheets. `examples/asset/alter_mesh.rs` complements it by tweaking mesh vertices for exaggerated kill-cam deformation.

#### When to Avoid It
Authoritative multiplayer servers expect assets to remain deterministic—runtime mutations belong in replay tools, spectator overlays, or single-player sandboxes only.



Sometimes the best iteration happens in flight. `examples/asset/alter_mesh.rs` modifies mesh assets after they’re spawned, perfect for procedural deformation, damage decals, or gameplay-driven transformations. Pair it with gizmo overlays from Chapter 22 to preview edits before committing them to disk.

Textures deserve similar flexibility. `examples/asset/alter_sprite.rs` updates texture assets on the fly, enabling color swaps, palette cycling, or dynamic UI skins without reloading the entire sprite. Combined with the asset processing flows from Chapter 19, runtime edits become part of an artist’s toolkit rather than an engine hack.

```rust
fn recolor_units(mut images: ResMut<Assets<Image>>, palette: &Palette) {
    if let Some(image) = images.get_mut(&palette.unit_texture) {
        for pixel in image.data.chunks_exact_mut(4) {
            pixel[0] = palette.tint_r;
            pixel[1] = palette.tint_g;
            pixel[2] = palette.tint_b;
        }
    }
}
```

This loop comes straight from `examples/asset/alter_sprite.rs`, making the Frostbite recolor workflow tangible.


### Game Context: Frostbite Skirmish Replay Editor
Replay tool **Frostbite Skirmish** lets shoutcasters recolor units on demand using `examples/asset/alter_sprite.rs`, tinting squads team-by-team during analysis. For dramatic kill-cam moments, they bend blades with `examples/asset/alter_mesh.rs` to emphasise impacts in slow motion.

#### When to Avoid It
Production multiplayer stays away from runtime mesh edits; authoritative servers expect assets to match shipped data, so on-the-fly deformation is reserved for spectator mode only.

## Practice Prompts
- Use `examples/asset/alter_mesh.rs` to drive morph-like mesh adjustments in response to gameplay, then serialize the results via the reflection tooling in Chapter 20.
- Build a live theming system by extending `examples/asset/alter_sprite.rs` so UI palettes change in real time based on player preferences or seasonal events.

## Runbook
Keep these quick references handy:

```
cargo run --example alter_mesh
cargo run --example alter_sprite
```
