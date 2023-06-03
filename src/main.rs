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
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy::input::common_conditions::input_toggle_active;
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
        //.add_plugin(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Key9)),)
        .add_plugin(MenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(bevy::diagnostic::SystemInformationDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_system(bevy::window::close_on_esc)
        .run();
}
