#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct FogMaterial {
    player_pos: vec2<f32>,
    vision_radius: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: FogMaterial;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Get distance from fragment to player position in world space
    let dist = distance(mesh.world_position.xy, material.player_pos);
    
    // Multi-stage fog effect:
    // 0 to vision_radius: fully visible (alpha = 0)
    // vision_radius to 1.5x: gradual dimming (alpha = 0 to 0.6)
    // 1.5x to 2.5x: heavy dimming (alpha = 0.6 to 0.85)
    // beyond 2.5x: nearly black (alpha = 0.85 to 0.95)
    
    var alpha: f32 = 0.0;
    
    if (dist < material.vision_radius) {
        // Inside vision circle - fully visible
        alpha = 0.0;
    } else if (dist < material.vision_radius * 1.5) {
        // Near fog - gradual dimming
        let t = (dist - material.vision_radius) / (material.vision_radius * 0.5);
        alpha = smoothstep(0.0, 1.0, t) * 0.6;
    } else if (dist < material.vision_radius * 2.5) {
        // Mid fog - heavier dimming
        let t = (dist - material.vision_radius * 1.5) / (material.vision_radius * 1.0);
        alpha = 0.6 + smoothstep(0.0, 1.0, t) * 0.25;
    } else {
        // Far fog - nearly black
        let t = min((dist - material.vision_radius * 2.5) / (material.vision_radius * 0.5), 1.0);
        alpha = 0.85 + smoothstep(0.0, 1.0, t) * 0.1;
    }
    
    // Return black with calculated alpha (0 = visible, 1 = dark)
    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}
