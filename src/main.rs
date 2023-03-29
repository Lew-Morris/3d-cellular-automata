use bevy::{
    prelude::*,
    // diagnostic::{
    //     FrameTimeDiagnosticsPlugin,
    //     LogDiagnosticsPlugin
    // }
};
use bevy_egui::EguiPlugin;

use render::*;
use rotating_camera::{
    RotatingCameraPlugin
};
use setup::*;

use crate::cell_event::CellStatesChangedEvent;

pub mod cell_event;
mod render;
mod helper;
mod neighbours;
mod rotating_camera;
mod rule;
mod cells;
mod setup;

fn main() {
    let mut task_pool_settings = TaskPoolOptions::default();
    task_pool_settings.async_compute.percent = 1.0f32;
    task_pool_settings.compute.percent = 0.0f32;
    task_pool_settings.io.percent = 0.0f32; // always use 1

    App::new()
        .insert_resource(task_pool_settings)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.9f32, 0.9f32, 0.9f32))) // Background color
        .add_event::<CellStatesChangedEvent>()
        .add_plugin(RotatingCameraPlugin)
        .add_plugin(CustomMaterialPlugin)
        .add_plugin(cells::SimsPlugin)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .run();
}

// fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut sims: ResMut<Sims>) {
//     sims.add_sim(
//         "Simple Cell".into(),
//         Box::new(cells::simple_cell::SingleThreaded::new()),
//     );
//
//     // sims.add_sim(
//     //     "(Unsafe) - Atomic Cell".into(),
//     //     Box::new(cells::simple_cell::LeddooAtomic::new()),
//     // );
//
//     sims.add_example(Example {
//         name: "Builder".into(),
//         rule: Rule {
//             survival_rule: Value::new(&[2, 6, 9]),
//             birth: Value::new(&[4, 6, 8, 9, 10]),
//             states: 10,
//             neighbourhood: Moore,
//         },
//         colour_method: ColorMethod::DistToCenter,
//         colour1: Color::YELLOW,
//         colour2: Color::RED,
//     });
//
//     sims.add_example(Example {
//         name: "Pyramid".into(),
//         rule: Rule {
//             survival_rule: Value::from_range(0..=6),
//             birth: Value::new(&[1, 3]),
//             states: 2,
//             neighbourhood: VonNeumann,
//         },
//
//         colour_method: ColorMethod::DistToCenter,
//         colour1: Color::GREEN,
//         colour2: Color::BLUE,
//     });
//
//     sims.set_example(0);
//
//     commands.spawn((
//         meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//         Transform::from_xyz(0.0, 0.0, 15.0),
//         GlobalTransform::default(),
//         InstanceMaterialData(Vec::new()),
//         // InstanceMaterialData
//         //     (
//         //     (1..=10)
//         //         .flat_map(|x| (1..=100).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
//         //         .map(|(x, y)| InstanceData {
//         //             position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
//         //             scale: 1.0,
//         //             color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
//         //         })
//         //         .collect(),
//         // ),
//         Visibility::default(),
//         ComputedVisibility::default(),
//         NoFrustumCulling,
//     ));
//
//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     })
//         .insert(RotatingCamera::default());
// }