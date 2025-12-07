use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use systems::startup::setup;

mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}
