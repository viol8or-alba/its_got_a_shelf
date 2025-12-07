use std::collections::HashSet;

use bevy::ecs::resource::Resource;
use bevy_ecs_ldtk::GridCoords;

#[derive(Default, Resource)]
pub(crate) struct LevelWalls {
    pub(crate) wall_locations: HashSet<GridCoords>,
    pub(crate) level_width: i32,
    pub(crate) level_height: i32,
}

impl LevelWalls {
    pub(crate) fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}