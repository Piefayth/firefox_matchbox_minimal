use bevy::prelude::*;
use common::*;

mod common;
mod network;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ui::UiPlugin)
        .add_plugin(network::NetworkPlugin)
        .add_state(GameState::Login)
        .insert_resource(LoginState {
            name: String::from("insert name"),
            room: String::from("default room"),
        })
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}