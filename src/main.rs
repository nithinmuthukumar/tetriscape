use bevy::prelude::*;
use tetriscape::camera::CameraPlugin;
use tetriscape::assetloader::AssetLoadPlugin;
use tetriscape::blocks::BlocksPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(BlocksPlugin)
        .run();
}

