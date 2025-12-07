use bevy::{ecs::bundle::Bundle, sprite::Sprite};
use bevy_ecs_ldtk::LdtkEntity;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}