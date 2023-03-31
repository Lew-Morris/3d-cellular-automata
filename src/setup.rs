use bevy::{
    prelude::*,
    render::view::NoFrustumCulling,
    // diagnostic::{
    //     FrameTimeDiagnosticsPlugin,
    //     LogDiagnosticsPlugin
    // }
};

use crate::cells::{Example, Sims};
use crate::cells;
use crate::neighbours::Neighbourhood::*;
use crate::render::{InstanceData, InstanceMaterialData};
use crate::rotating_camera::RotatingCamera;
use crate::rule::{ColorMethod, Rule, Value};

// use bevy_fly_camera::FlyCamera;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut sims: ResMut<Sims>) {
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
            neighbourhood: Moore,
        },
        colour_method: ColorMethod::DistToCenter,
        colour1: Color::GRAY,
        colour2: Color::CYAN,
    });

    sims.add_example(Example {
        name: "Pyramid".into(),
        rule: Rule {
            survival_rule: Value::from_range(0..=6),
            birth: Value::new(&[1, 3]),
            states: 2,
            neighbourhood: VonNeumann,
        },

        colour_method: ColorMethod::DistToCenter,
        colour1: Color::WHITE,
        colour2: Color::GREEN,
    });

    sims.set_example(0);

    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        // InstanceMaterialData(Vec::new()),
        InstanceMaterialData
            (
            (1..=10)
                .flat_map(|x| (1..=100).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
                    scale: 0.9,
                    color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
                })
                .collect(),
        ),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        // transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        // .insert(FlyCamera::default()); // Todo! Add movable camera
        // .insert(Camera::default());
        .insert(RotatingCamera::default());

    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_translation(Vec3::ONE * 3.0),
    //     ..default()
    // });
}