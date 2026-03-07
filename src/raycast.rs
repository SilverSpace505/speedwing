use bevy::prelude::*;

use crate::grid_map::GridMap;

pub struct Raycaster;

impl Raycaster {
    pub fn raycast(grid_map: &GridMap, start: Vec2, dir: Vec2, distance: f32, step: f32) -> f32 {
        let dir = dir.normalize_or_zero();
        let mut current = step;
        while current <= distance {
            if grid_map
                .get_world(start.x + current * dir.x, start.y + current * dir.y)
                .is_some_and(|v| v >= 0.5)
            {
                return current;
            }

            current += step
        }
        distance
    }
}
