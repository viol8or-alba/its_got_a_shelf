use bevy::{asset::AssetServer, camera::{Camera2d, OrthographicProjection, Projection}, ecs::system::{Commands, Res}, transform::components::Transform};
use bevy_ecs_ldtk::LdtkWorldBundle;

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("its_got_a_shelf.ldtk").into(),
        ..Default::default()
    });
}