use bevy::{
    prelude::{
        Assets,
        Camera3dBundle,
        Color,
        Commands,
        default,
        GlobalTransform,
        Mesh,
        ResMut,
        shape,
        Transform,
        Vec3,
    },
    render::view::{
        ComputedVisibility,
        NoFrustumCulling,
        Visibility
    },
};

use crate::{
    cells::{
        Example,
        single_threaded,
        Sims,
    },
    neighbours::Neighbourhood::{
        Moore,
        VonNeumann,
    },
    render::{
        InstanceData,
        InstanceMaterialData,
    },
    rotating_camera::{
        RotatingCamera,
    },
    rule::{
        ColourMethod,
        Rule,
        Value,
    },
};

// DIAGNOSTICS - Framerate information
// use bevy::{
//     diagnostic::{
//         FrameTimeDiagnosticsPlugin,
//         LogDiagnosticsPlugin
//     },
// };

// use bevy_fly_camera::FlyCamera;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut sims: ResMut<Sims>) {
    sims.add_sim(
        "Simple Cell".into(),
        Box::new(single_threaded::SingleThreaded::new()),
    );

    sims.add_example(Example {
        name: "Builder".into(),
        rule: Rule {
            survival_rule: Value::new(&[2, 6, 9]),
            birth: Value::new(&[4, 6, 8, 9, 10]),
            states: 10,
            neighbourhood: Moore,
        },
        colour_method: ColourMethod::DistToCenter,
        colour1: Color::RED,
        colour2: Color::GREEN,
    });

    sims.add_example(Example {
        name: "Pyramid".into(),
        rule: Rule {
            survival_rule: Value::from_range(0..=6),
            birth: Value::new(&[1, 3]),
            states: 2,
            neighbourhood: VonNeumann,
        },

        colour_method: ColourMethod::State,
        colour1: Color::BLACK,
        colour2: Color::PINK,
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
        // .insert(FlyCamera::default());

    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_translation(Vec3::ONE * 3.0),
    //     ..default()
    // }).insert(PointLight::default());
}