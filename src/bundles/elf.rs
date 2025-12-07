use bevy::{ecs::{bundle::Bundle, component::Component}, sprite::Sprite};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};
#[derive(Default, Component)]
pub(crate) struct Elf;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct ElfBundle {
    player: Elf,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}