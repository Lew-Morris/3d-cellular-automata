// First-Party
// Modules
pub mod cell_event;
mod cell_render;
mod helper;
mod neighbours;
mod rotating_camera;
mod rule;
mod cells;
// mod setup;

// Imports
use cell_render::*;
use neighbours::Neighbourhood::*;
use rule::*;
use cell_event::CellStatesChangedEvent;
use cells::sims::Example;
use crate::cells::Sims;
// use setup::*;

// Third-Party
// Imports
use rotating_camera::{RotatingCamera, RotatingCameraPlugin}; //todo! Implement rotating camera
use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

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
        .add_plugin(CellMaterialPlugin)
        .add_plugin(cells::SimsPlugin)
        .add_startup_system(configure_visuals_system)
        .add_startup_system( setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut sims: ResMut<Sims>) {
    sims.add_sim(
        "Simple Cell".into(),
        Box::new(cells::simple_cell::SingleThreaded::new()),
    );

    sims.add_example(Example {
        name: "Builder".into(),
        rule: Rule {
            survival_rule: Value::new(&[2, 6, 9]),
            birth: Value::new(&[4, 6, 8, 9, 10]),
            states: 10,
            neighbourhood: VonNeumann,
        },
        color_method: ColorMethod::DistToCenter,
        color1: Color::YELLOW,
        color2: Color::RED,
    });

    sims.add_example(Example {
        name: "Pyramid".into(),
        rule: Rule {
            survival_rule: Value::from_range(0..=6),
            birth: Value::new(&[1, 3]),
            states: 2,
            neighbourhood: VonNeumann,
        },

        color_method: ColorMethod::DistToCenter,
        color1: Color::GREEN,
        color2: Color::BLUE,
    });

    sims.set_example(0);

    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        // SpatialBundle::INHERITED_IDENTITY,
        Transform::from_xyz(0.0, 0.0, 15.0),
        GlobalTransform::default(),
        InstanceMaterialData (
            (1..=10)
                .flat_map(|x| (1..=100).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
                    scale: 1.0,
                    color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
                })
                .collect(),
        ),
        Visibility::Visible,
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(RotatingCamera::default());
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}
