use bevy::prelude::*;

use crate::collision::CollisionMap;
use crate::characters::physics::Velocity;
use crate::config::player::{COLLIDER_RADIUS};

/// A circular collider for collision detection.
/// 
/// The collider position is offset from the entity's transform,
/// typically to represent the character's feet position.
#[derive(Component, Debug, Clone)]
pub struct Collider {
    /// Radius of the circular collider in world units
    pub radius: f32,
    /// Offset from entity center (e.g., Vec2(0, -25) for feet)
    pub offset: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: COLLIDER_RADIUS,
            offset: Vec2::ZERO,
        }
    }
}

impl Collider {
    /// Get the world position of this collider given an entity's transform.
    pub fn world_position(&self, transform: &Transform) -> Vec2 {
        transform.translation.truncate() + self.offset
    }
}

pub fn validate_movement(
    map: Option<Res<CollisionMap>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Velocity, &Collider)>,
) {
    let Some(map) = map else { return };

    for (transform, mut velocity, collider) in query.iter_mut() {
        // Skip if not moving
        if !velocity.is_moving() {
            continue;
        }

        // Current collider position
        let current_pos = collider.world_position(transform);
        
        // Desired new position based on velocity
        let delta = velocity.0 * time.delta_secs();
        let desired_pos = current_pos + delta;

        // Use swept collision to find valid position
        let valid_pos = map.sweep_circle(current_pos, desired_pos, collider.radius);

        // Calculate what velocity would get us to valid_pos
        let actual_delta = valid_pos - current_pos;
        
        // Only update velocity if collision modified our path
        if (actual_delta - delta).length_squared() > 0.001 {
            // Convert position delta back to velocity
            let dt = time.delta_secs();
            if dt > 0.0 {
                velocity.0 = actual_delta / dt;
            }
        }
    }
}