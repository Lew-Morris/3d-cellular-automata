use bevy::{
    prelude::*,
    // diagnostic::{
    //     FrameTimeDiagnosticsPlugin,
    //     LogDiagnosticsPlugin
    // }
};
use bevy_egui::EguiPlugin;
// use bevy_fly_camera::FlyCameraPlugin;

use render::*;
use rotating_camera::RotatingCameraPlugin;
use setup::*;

use crate::state_changed::StateChangedEvent;

// use bevy_fly_camera::FlyCameraPlugin;

mod state_changed;
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
    task_pool_settings.io.percent = 1.0f32;

    // todo! add pause functionality
    // type Paused = bool;
    // let state: Paused = true;

    App::new()
        .add_event::<StateChangedEvent>()
        .insert_resource(task_pool_settings)
        // .insert_resource(state)
        // todo! Add a setting to change the background colour, requires restart
        .insert_resource(ClearColor(Color::rgb(0.0f32, 0.0f32, 0.0f32))) // Black background color
        // .insert_resource(ClearColor(Color::rgb(0.9f32, 0.9f32, 0.9f32))) // Off-White background color
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(RotatingCameraPlugin)
        // .add_plugin(FlyCameraPlugin)
        .add_plugin(CustomMaterialPlugin)
        .add_plugin(cells::SimsPlugin)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default()) // Debugging
        // .add_plugin(LogDiagnosticsPlugin::default()) // Debugging
        .add_startup_system(setup)
        .run();
}