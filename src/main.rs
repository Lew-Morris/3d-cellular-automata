use bevy::{
    prelude::*,
    // diagnostic::{
    //     FrameTimeDiagnosticsPlugin,
    //     LogDiagnosticsPlugin
    // }
};
use bevy_egui::EguiPlugin;

use render::*;
use rotating_camera::RotatingCameraPlugin;
use setup::*;

use crate::state_changed::StateChangedEvent;

// use bevy_fly_camera::FlyCameraPlugin;

mod tests;
pub mod state_changed;
mod render;
mod utilities;
mod neighbours;
mod rotating_camera;
mod rule;
mod cells;
mod setup;

fn main() {
    let mut task_pool_settings = TaskPoolOptions::default();
    task_pool_settings.async_compute.percent = 1.0f32;
    task_pool_settings.compute.percent = 0.0f32;
    task_pool_settings.io.percent = 1.0f32; // always use 1

    App::new()
        .add_event::<StateChangedEvent>()
        .insert_resource(task_pool_settings)
        .insert_resource(ClearColor(Color::rgb(0.9f32, 0.9f32, 0.9f32))) // Background color
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RotatingCameraPlugin)
        // .add_plugin(FlyCameraPlugin) // todo! implement flying camera
        .add_plugin(CustomMaterialPlugin)
        .add_plugin(cells::SimsPlugin)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default()) // Debugging
        // .add_plugin(LogDiagnosticsPlugin::default()) // Debugging
        .add_startup_system(setup)
        .run();
}