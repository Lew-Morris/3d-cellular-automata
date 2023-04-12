use bevy::{
    prelude::{
        default, shape, Assets, Camera3dBundle, Color, Commands, GlobalTransform, Mesh, ResMut,
        Transform, Vec3,
    },
    render::view::{ComputedVisibility, NoFrustumCulling, Visibility},
};

use bevy_flycam::prelude::*;

// use crate::cells::multi_threaded;
use crate::color_method::ColourMethod::*;
use crate::{
    cells::{
        multi_dimensional,
        single_threaded,
        Example,
        // multi_threaded,
        Sims,
    },
    neighbours::Neighbourhood::{Moore, VonNeumann},
    render::{InstanceData, InstanceMaterialData},
    rule::{Rule, Value},
};

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut sims: ResMut<Sims>) {
    sims.add_sim(
        "Simple Cell".into(),
        Box::new(single_threaded::SingleThreaded::new()),
    );

    sims.add_sim(
        "Multi-Dimensional Cell".into(),
        Box::new(multi_dimensional::MultiDimensional::new()),
    );

    // sims.add_sim(
    //     "Multi-Threaded Cell".into(),
    //     Box::new(multi_threaded::MultiThreaded::new()),
    // );

    // todo! Add to its own system
    sims.add_example(Example {
        name: "Chaos".into(),
        rule: Rule {
            survival: Value::new(&[2, 6, 9]),
            birth: Value::new(&[4, 6, 8, 9, 10]),
            states: 10,
            neighbourhood: Moore,
        },
        colour_method: State,
        colour1: Color::RED,
        colour2: Color::GREEN,
    });

    sims.add_example(Example {
        name: "Expanding Pyramid".into(),
        rule: Rule {
            survival: Value::from_range(0..=6),
            birth: Value::new(&[1, 3]),
            states: 2,
            neighbourhood: VonNeumann,
        },
        colour_method: Neighbour,
        colour1: Color::BLACK,
        colour2: Color::PINK,
    });

    sims.add_example(Example {
        name: "Morphing Pathways".into(),
        rule: Rule {
            survival: Value::new(&[2, 6, 7, 8, 9, 10, 11, 14, 15]),
            birth: Value::new(&[4]),
            states: 50,
            neighbourhood: Moore,
        },
        colour_method: State,
        colour1: Color::RED,
        colour2: Color::CYAN,
    });

    sims.add_example(Example {
        name: "Crazy Patterns".into(),
        rule: Rule {
            survival: Value::new(&[2, 7, 10, 16, 19, 22, 25]),
            birth: Value::new(&[4]),
            states: 25,
            neighbourhood: Moore,
        },
        colour_method: State,
        colour1: Color::LIME_GREEN,
        colour2: Color::rgb(47.0, 0.0, 255.0),
    });

    sims.add_example(Example {
        name: "Pathways".into(),
        rule: Rule {
            survival: Value::new(&[2, 6, 7, 8, 9, 10, 11, 12]),
            birth: Value::new(&[4]),
            states: 50,
            neighbourhood: Moore,
        },
        colour_method: Index,
        colour1: Color::WHITE,
        colour2: Color::BLACK,
    });

    sims.add_example(Example {
        name: "Cycle States (SLOW)".into(),
        rule: Rule {
            survival: Value::new(&[2, 6, 7, 8, 9, 10, 11, 12]),
            birth: Value::new(&[1, 4]),
            states: 50,
            neighbourhood: Moore,
        },
        colour_method: State,
        colour1: Color::LIME_GREEN,
        colour2: Color::rgb(47.0, 0.0, 255.0),
    });

    // todo! Use RNG to select a random example
    sims.set_example(2);

    // todo! Have a look into transparent cells for demo
    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        InstanceMaterialData(
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
    // https://bevy-cheatbook.github.io/window/clear-color.html
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(50.0, 25.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        // .insert(RotatingCamera::default());
        .insert(FlyCam);
}
