// src/map/collision.rs
use bevy::prelude::*;
use super::tile_marker::TileType;

/// Collision map resource that stores walkability information
/// for the entire game map in a simple 2D grid.
#[derive(Resource)]
pub struct Map {
    /// Flat array of tile types, row-major order (like the tutorial!)
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub tile_size: f32,  // 32.0 pixels per tile
    pub grid_origin_x: f32,  // World X coordinate for grid (0, 0)
    pub grid_origin_y: f32,  // World Y coordinate for grid (0, 0)
}

impl Map {
    /// Create a new empty map filled with Empty tiles
    pub fn new(width: i32, height: i32, tile_size: f32) -> Self {
        let size = (width * height) as usize;
        // Default: assume centered map
        let grid_origin_x = -(width as f32 * tile_size) / 2.0;
        let grid_origin_y = -(height as f32 * tile_size) / 2.0;
        Self {
            tiles: vec![TileType::Empty; size],
            width,
            height,
            tile_size,
            grid_origin_x,
            grid_origin_y,
        }
    }
    
    /// Create a map with explicit origin coordinates
    pub fn with_origin(width: i32, height: i32, tile_size: f32, origin_x: f32, origin_y: f32) -> Self {
        let size = (width * height) as usize;
        Self {
            tiles: vec![TileType::Empty; size],
            width,
            height,
            tile_size,
            grid_origin_x: origin_x,
            grid_origin_y: origin_y,
        }
    }
    
    /// Convert 2D grid coordinates to 1D array index
    /// Same pattern as the roguelike tutorial!
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
    
    /// Check if grid coordinates are within bounds
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    
    /// Check if a grid position is walkable
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if !self.in_bounds(x, y) {
            return false; // Out of bounds = not walkable
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx].is_walkable()
    }
    
    /// Set tile type at grid position
    pub fn set_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if !self.in_bounds(x, y) {
            return;
        }
        let idx = self.xy_idx(x, y);
        self.tiles[idx] = tile_type;
    }
    
    /// Convert world position (in pixels) to grid coordinates
    /// This accounts for the grid origin stored in the map
    pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        let relative_x = world_pos.x - self.grid_origin_x;
        let relative_y = world_pos.y - self.grid_origin_y;
        
        IVec2::new(
            (relative_x / self.tile_size).floor() as i32,
            (relative_y / self.tile_size).floor() as i32,
        )
    }
    
    /// Check if a world position is walkable
    pub fn is_world_pos_walkable(&self, world_pos: Vec2) -> bool {
        let grid_pos = self.world_to_grid(world_pos);
        self.is_walkable(grid_pos.x, grid_pos.y)
    }
    
    /// Check if a world position is walkable with a margin/buffer
    /// This allows the player to walk close to obstacles without exact collision
    /// margin_tiles: how many tiles away from obstacles the player must stay (0.5 = half a tile)
    pub fn is_world_pos_walkable_with_margin(&self, world_pos: Vec2, margin_tiles: f32) -> bool {
        // Check main position
        let grid_pos = self.world_to_grid(world_pos);
        if !self.is_walkable(grid_pos.x, grid_pos.y) {
            return false;
        }
        
        // If no margin, we're done
        if margin_tiles <= 0.0 {
            return true;
        }
        
        // Check a circle of points around the position for obstacles
        // Sample points in a circle around the player
        let num_samples = 8;
        let margin_world = margin_tiles * self.tile_size;
        
        for i in 0..num_samples {
            let angle = (i as f32 / num_samples as f32) * std::f32::consts::TAU;
            let check_x = world_pos.x + margin_world * angle.cos();
            let check_y = world_pos.y + margin_world * angle.sin();
            
            let check_grid = self.world_to_grid(Vec2::new(check_x, check_y));
            if !self.is_walkable(check_grid.x, check_grid.y) {
                return false; // Too close to an obstacle
            }
        }
        
        true
    }
    
    /// Check if a world position is walkable with adaptive margins
    /// Stricter near water edges (Shore), looser for other terrain
    pub fn is_world_pos_walkable_smart(&self, world_pos: Vec2) -> bool {
        // Check the main position first
        let grid_pos = self.world_to_grid(world_pos);
        let main_tile = self.get_tile(grid_pos.x, grid_pos.y);
        
        if let Some(tile) = main_tile {
            if !tile.is_walkable() {
                return false; // Standing on unwalkable tile
            }
            
            // If standing on Shore, be very strict about it
            // Check a tighter circle of points to prevent walking into water
            if tile == TileType::Shore {
                // Check 12 points in a circle (more dense than before)
                let num_samples = 12;
                let margin_world = 0.25 * self.tile_size; // Larger margin for shore
                
                for i in 0..num_samples {
                    let angle = (i as f32 / num_samples as f32) * std::f32::consts::TAU;
                    let check_x = world_pos.x + margin_world * angle.cos();
                    let check_y = world_pos.y + margin_world * angle.sin();
                    
                    let check_grid = self.world_to_grid(Vec2::new(check_x, check_y));
                    if !self.is_walkable(check_grid.x, check_grid.y) {
                        return false; // Too close to water or obstacle
                    }
                }
                return true;
            }
            
            // For other tiles, use smaller margin
            let num_samples = 8;
            let margin_world = 0.15 * self.tile_size;
            
            for i in 0..num_samples {
                let angle = (i as f32 / num_samples as f32) * std::f32::consts::TAU;
                let check_x = world_pos.x + margin_world * angle.cos();
                let check_y = world_pos.y + margin_world * angle.sin();
                
                let check_grid = self.world_to_grid(Vec2::new(check_x, check_y));
                if !self.is_walkable(check_grid.x, check_grid.y) {
                    return false;
                }
            }
            true
        } else {
            false // Out of bounds = NOT walkable (safety!)
        }
    }
    
    /// New robust collision: circle collider at world_pos with radius_world
    /// Tests actual circle-AABB overlap instead of point sampling
    pub fn is_world_pos_clear_circle(&self, world_pos: Vec2, radius_world: f32) -> bool {
        // Treat out-of-bounds as solid (prevents walking off map)
        if !self.is_world_pos_within_bounds(world_pos, radius_world) {
            return false;
        }

        if radius_world <= 0.0 {
            return self.is_world_pos_walkable(world_pos);
        }

        // Compute grid range that the circle overlaps
        let min_gx = ((world_pos.x - radius_world - self.grid_origin_x) / self.tile_size).floor() as i32;
        let max_gx = ((world_pos.x + radius_world - self.grid_origin_x) / self.tile_size).floor() as i32;
        let min_gy = ((world_pos.y - radius_world - self.grid_origin_y) / self.tile_size).floor() as i32;
        let max_gy = ((world_pos.y + radius_world - self.grid_origin_y) / self.tile_size).floor() as i32;

        for gy in min_gy..=max_gy {
            for gx in min_gx..=max_gx {
                if !self.in_bounds(gx, gy) {
                    return false; // Solid border
                }
                let tile = self.get_tile(gx, gy);
                if let Some(t) = tile {
                    if !t.is_walkable() && self.circle_intersects_tile(world_pos, radius_world, gx, gy) {
                        // Extra strictness for shore: increase effective radius
                        let effective_radius = if t == TileType::Shore {
                            radius_world + 0.1 * self.tile_size
                        } else {
                            radius_world
                        };
                        
                        if self.circle_intersects_tile(world_pos, effective_radius, gx, gy) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// Swept movement: move in small steps and slide on obstacles
    /// Returns the farthest valid position along the path
    pub fn try_move_circle(
        &self,
        start: Vec2,
        desired_end: Vec2,
        radius_world: f32,
    ) -> Vec2 {
        let delta = desired_end - start;
        let delta_len = delta.length();
        
        // No movement needed
        if delta_len < 0.001 {
            return start;
        }
        
        // Step size: quarter tile per substep prevents tunneling
        let max_step = self.tile_size * 0.25;
        let steps = (delta_len / max_step).ceil().max(1.0) as i32;
        let step_v = delta / steps as f32;

        let mut p = start;
        for _ in 0..steps {
            let candidate = p + step_v;
            
            // Try direct movement
            if self.is_world_pos_clear_circle(candidate, radius_world) {
                p = candidate;
            } else {
                // Try sliding on X axis
                let try_x = Vec2::new(candidate.x, p.y);
                if self.is_world_pos_clear_circle(try_x, radius_world) {
                    p = try_x;
                    continue;
                }
                
                // Try sliding on Y axis
                let try_y = Vec2::new(p.x, candidate.y);
                if self.is_world_pos_clear_circle(try_y, radius_world) {
                    p = try_y;
                    continue;
                }
                
                // Blocked, stop moving
                break;
            }
        }
        p
    }

    /// Check if circle at position is within map bounds (with radius)
    fn is_world_pos_within_bounds(&self, world_pos: Vec2, radius_world: f32) -> bool {
        let left = self.grid_origin_x;
        let right = self.grid_origin_x + self.width as f32 * self.tile_size;
        let bottom = self.grid_origin_y;
        let top = self.grid_origin_y + self.height as f32 * self.tile_size;

        world_pos.x - radius_world >= left
            && world_pos.x + radius_world <= right
            && world_pos.y - radius_world >= bottom
            && world_pos.y + radius_world <= top
    }

    /// Test if circle intersects tile's AABB
    fn circle_intersects_tile(&self, center: Vec2, radius: f32, gx: i32, gy: i32) -> bool {
        let min = Vec2::new(
            self.grid_origin_x + gx as f32 * self.tile_size,
            self.grid_origin_y + gy as f32 * self.tile_size,
        );
        let max = min + Vec2::splat(self.tile_size);

        // Closest point on tile AABB to circle center
        let cx = center.x.clamp(min.x, max.x);
        let cy = center.y.clamp(min.y, max.y);

        let dx = center.x - cx;
        let dy = center.y - cy;
        dx * dx + dy * dy <= radius * radius
    }
    
    /// Get a tile at grid coordinates without bounds checking
    fn get_tile(&self, x: i32, y: i32) -> Option<TileType> {
        if self.in_bounds(x, y) {
            let idx = self.xy_idx(x, y);
            Some(self.tiles[idx])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map_creation() {
        let map = Map::new(25, 18, 32.0);
        assert_eq!(map.tiles.len(), 25 * 18);
        assert!(map.is_walkable(0, 0)); // Empty is walkable
    }
    
    #[test]
    fn test_tile_walkability() {
        assert!(TileType::Dirt.is_walkable());
        assert!(TileType::Grass.is_walkable());
        assert!(!TileType::Water.is_walkable());
        assert!(!TileType::Tree.is_walkable());
    }
    
    #[test]
    fn test_world_to_grid() {
        let map = Map::new(25, 18, 32.0);
        // Map is centered, so (0, 0) world = (12, 9) grid approximately
        let grid_pos = map.world_to_grid(Vec2::ZERO);
        println!("Center world position maps to grid: {:?}", grid_pos);
        assert!(map.in_bounds(grid_pos.x, grid_pos.y));
    }
}

