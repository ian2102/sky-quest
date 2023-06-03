mod game;
mod menu;
mod player;
mod prelude;

use crate::game::gameplay::GamePlugin;
use crate::menu::menu::MenuPlugin;
use crate::player::player::PlayerPlugin;
use crate::prelude::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    window::{PresentMode, WindowPlugin},
};
use bevy_rapier3d::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sky Quest".into(),
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<GameState>()
        .add_plugin(MenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(bevy::diagnostic::SystemInformationDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
