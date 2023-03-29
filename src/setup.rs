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
use crate::render::InstanceMaterialData;
use crate::rotating_camera::{
    RotatingCamera,
};
use crate::rule::{ColorMethod, Rule, Value};

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut sims: ResMut<Sims>) {
    sims.add_sim(
        "Simple Cell".into(),
        Box::new(cells::simple_cell::SingleThreaded::new()),
    );

    // sims.add_sim(
    //     "(Unsafe) - Atomic Cell".into(),
    //     Box::new(cells::simple_cell::LeddooAtomic::new()),
    // );

    sims.add_example(Example {
        name: "Builder".into(),
        rule: Rule {
            survival_rule: Value::new(&[2, 6, 9]),
            birth: Value::new(&[4, 6, 8, 9, 10]),
            states: 10,
            neighbourhood: Moore,
        },
        colour_method: ColorMethod::DistToCenter,
        colour1: Color::YELLOW,
        colour2: Color::RED,
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
        colour1: Color::GREEN,
        colour2: Color::BLUE,
    });

    sims.set_example(0);

    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.0, 15.0),
        GlobalTransform::default(),
        InstanceMaterialData(Vec::new()),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));

    // Spawn Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(RotatingCamera::default());
}