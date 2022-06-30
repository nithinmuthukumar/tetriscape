use bevy::prelude::*;
use tetriscape::camera::CameraPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .run();
}

