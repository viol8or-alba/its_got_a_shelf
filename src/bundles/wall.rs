use bevy::ecs::{bundle::Bundle, component::Component};
use bevy_ecs_ldtk::LdtkIntCell;

#[derive(Default, Component)]
pub(crate) struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
pub(crate) struct WallBundle {
    wall: Wall,
}