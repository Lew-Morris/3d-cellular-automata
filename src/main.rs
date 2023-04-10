use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// use bevy_fly_camera::FlyCameraPlugin;
use bevy_flycam::prelude::*;

use render::*;
use setup::*;
// use rotating_camera::RotatingCameraPlugin;
use crate::state_changed::StateChangedEvent;

mod cells;
mod neighbours;
mod render;
mod rotating_camera;
mod rule;
mod setup;
mod state_changed;
mod utilities;

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
        // .add_plugin(RotatingCameraPlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::LShift,
            move_descend: KeyCode::LControl,
            ..Default::default()
        })
        // .add_plugin(FlyCameraPlugin)
        .add_plugin(CustomMaterialPlugin)
        .add_plugin(cells::SimsPlugin)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default()) // Debugging
        // .add_plugin(LogDiagnosticsPlugin::default()) // Debugging
        .add_startup_system(setup)
        .run();
}
