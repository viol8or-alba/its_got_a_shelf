use bevy::{ecs::{bundle::Bundle, component::Component}, sprite::Sprite};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};

#[derive(Default, Component)]
pub(crate) struct Goal;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct GoalBundle {
    goal: Goal,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}
