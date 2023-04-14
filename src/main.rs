use bevy::diagnostic::{FrameTimeDiagnosticsPlugin,};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_flycam::prelude::*;

use render::*;
use setup::*;

mod cells;
mod color_method;
mod neighbours;
mod render;
mod rotating_camera;
mod rule;
mod setup;
mod utilities;

fn main() {
    //
    let mut task_pool_settings = TaskPoolOptions::default();
    // Permit access to all available threads
    task_pool_settings.async_compute.percent = 1.0f32;
    task_pool_settings.compute.percent = 1.0f32;
    task_pool_settings.io.percent = 1.0f32;

    // todo! add pause functionality

    App::new()
        // Add default plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // Set default window settings
            // Adapted from: https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs
            primary_window: Some(Window {
                title: "3D Cellular Automata".into(),
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoNoVsync,
                // WASM config
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        // Add task-pool settings
        .insert_resource(task_pool_settings)
        // Default background color - Black
        .insert_resource(ClearColor(Color::rgb(0.0f32, 0.0f32, 0.0f32)))
        // Add EGUI (Settings UI library) plugin
        .add_plugin(EguiPlugin)
        // Add FlyCam plugin
        .add_plugin(NoCameraPlayerPlugin)
        // Change FlyCam movement
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 25.0,          // default: 12.0
        })
        // Change key bindings
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::LShift,
            move_descend: KeyCode::LControl,
            ..Default::default()
        })
        // Rendering plugin
        .add_plugin(CustomMaterialPlugin)
        .add_plugin(cells::SimsPlugin)
        // Framerate plugin
        .add_plugin(FrameTimeDiagnosticsPlugin::default()) // Debugging
        // Setup the simulation
        .add_startup_system(setup)
        // Begin!
        .run();
}
