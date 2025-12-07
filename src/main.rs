use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use systems::startup::setup;

use crate::{bundles::{elf::ElfBundle, goal::GoalBundle, player::PlayerBundle}, systems::update::{move_player_from_input, translate_grid_coords_entities}};

mod bundles;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player_from_input, translate_grid_coords_entities))
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<ElfBundle>("Elf")
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .run();
}
