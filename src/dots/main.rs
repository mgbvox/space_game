mod movement;
mod trails;
mod constants;
mod rng;

use crate::movement;

use std::ops::Neg;
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use bevy_rand::prelude::*;
use bevy_rand::prelude::{WyRand};
use rand::RngCore;
use crate::constants::{DESIRED_DOT_DISTANCE, DOT_DECAY_DESPAWN_THRESHOLD, DOT_DECAY_RATE, DOT_SPAWN_DELTA, PLAYER_COLOR, PLAYER_INITIAL_POSITION, PLAYER_INITIAL_VELOCITY};
use crate::movement::MovementState;

// a flag to indicate that an object is the player
#[derive(Component)]
struct Player;


#[derive(Component)]
struct Dot;

#[derive(Component)]
struct Age(f32);


#[derive(Resource)]
struct DotTimer(Timer);

#[derive(Component)]
struct HUD;

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());


    commands.spawn(
        (
            // the entity
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 25. })),
                material: materials.add(PLAYER_COLOR),
                transform: Transform::from_xyz(PLAYER_INITIAL_POSITION.x, PLAYER_INITIAL_POSITION.y, PLAYER_INITIAL_POSITION.z),
                ..default()
            },
            // attach components here
            Player,
            movement::MovementState {
                position: PLAYER_INITIAL_POSITION,
                velocity: PLAYER_INITIAL_VELOCITY,
                acceleration: Vec3::ZERO,
            }
        ),
    );

    // repeating timers
    // dot spawning
    commands.insert_resource(DotTimer(Timer::from_seconds(DOT_SPAWN_DELTA, TimerMode::Repeating)));

    // track dot position for even spawn drawing
    commands.insert_resource(LastDotPos(PLAYER_INITIAL_POSITION))
}


#[derive(Resource)]
struct LastDotPos(Vec3);


fn random(rng: &mut ResMut<GlobalEntropy<WyRand>>, min: f32, max: f32) -> f32 {
    min + (rng.next_u32() as f32) / (u32::MAX as f32 + 1.) * (max - min)
}


fn dot_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_movement_state: Query<&MovementState, With<Player>>,
    mut last_dot: ResMut<LastDotPos>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    for movement_state in &player_movement_state {
        if movement_state.acceleration.norm() > 0.0 {
            let player_pos = movement_state.position;

            let player_acceleration = movement_state.acceleration;
            let player_velocity = movement_state.velocity;

            // todo: for x/y accel, deploy dots in that direction
            //  and subtract particle mass from player mass

            let distance = player_pos.distance(last_dot.0);
            if distance > DESIRED_DOT_DISTANCE {
                let shape = Mesh2dHandle(meshes.add(
                    Circle { radius: random(&mut rng, 3.0, 10.0) }
                ));

                let color = Color::srgba(255., 255., 255., 0.5);
                const TRAIL_DELTA_MAG_X: f32 = 7.0;
                const TRAIL_DELTA_MAG_Y: f32 = 7.0;

                let dot_transform = Transform::from_xyz(
                    // spawn trail dot at player's position
                    // add slight offset noise
                    player_pos.x + random(&mut rng, -TRAIL_DELTA_MAG_X, TRAIL_DELTA_MAG_X),
                    player_pos.y + random(&mut rng, -TRAIL_DELTA_MAG_Y, TRAIL_DELTA_MAG_Y),
                    player_pos.z - 1.0,
                );

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: shape,
                        material: materials.add(
                            color,
                        ),
                        // always spawn beneath the player z
                        transform: dot_transform,
                        ..default()
                    },
                    Dot,
                    Age(0.0),
                    MovementState {
                        // todo: change velocity to point slightly off-axis, e.g. cone-shaped exhaust
                        // reverse the velocity - dot is moving backwards!
                        velocity: (player_velocity.neg() + player_acceleration),
                        position: dot_transform.translation,
                        acceleration: Vec3::ZERO,
                    }
                ));

                last_dot.0 = dot_transform.translation
            }
        }
    }
}


// fn dot_age_system(
//     mut commands: Commands,
//     mut query: Query<(
//         Entity,
//         &mut Age,
//         &mut Handle<ColorMaterial>,
//     ), With<Dot>>,
//     time: Res<Time>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     for (entity, age, material_handle) in query.iter_mut() {
//         if let Some(material) = materials.get_mut(material_handle.id()) {
//             let [r, g, b, a] = material.color.to_srgba().to_f32_array();
//             if a < DOT_DECAY_DESPAWN_THRESHOLD {
//                 // fully remove this dot entity
//                 commands.entity(entity).despawn()
//             } else {
//                 // todo: increment age by delta_seconds and use this to make lighter
//                 // age.0 += time.delta_seconds();
//                 let new_color = Color::srgba(r, g, b, a * DOT_DECAY_RATE);
//                 material.color = new_color;
//             }
//         }
//     }
// }

fn dot_decay_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Handle<ColorMaterial>,
    ), With<Dot>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(material_handle.id()) {
            let [r, g, b, a] = material.color.to_srgba().to_f32_array();
            if a < DOT_DECAY_DESPAWN_THRESHOLD {
                // fully remove this dot entity
                commands.entity(entity).despawn()
            } else {
                let new_color = Color::srgba(r, g, b, a * DOT_DECAY_RATE);
                material.color = new_color;
            }
        }
    }
}


fn create_scoreboard(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     window: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(win) = window.get_single() {
        let font = asset_server.load("embedded://fonts/HackNerdFont-Regular.ttf");
        let top_left = Transform {
            translation: Vec3::new(-(win.width() / 2.0) + 10.0, (win.height() / 2.0) - 30.0, 0.0),
            ..default()
        };

        let text_style = TextStyle {
            font: font.clone(),
            font_size: 20.,
            ..default()
        };


        let mut hud_bundle = TextBundle::from_sections((0..6).map(|e| TextSection::new(format!("{}\n", e), text_style.clone())))
            .with_text_justify(JustifyText::Left);


        hud_bundle.transform = top_left;

        commands.spawn(
            (
                hud_bundle,
                HUD,
            )
        );
    };
}

fn update_scoreboard(mut hud_query: Query<&mut Text, With<HUD>>,
                     player_query: Query<(&Transform, &movement::MovementState), With<Player>>) {
    if let Ok((player_tf, player_v)) = player_query.get_single() {
        for mut hud_element in hud_query.iter_mut() {
            hud_element.sections[0].value = format!("dx: {}\n", player_v.velocity.x.to_string());
            hud_element.sections[1].value = format!("dy: {}\n", player_v.velocity.y.to_string());
            hud_element.sections[2].value = format!("x: {}\n", player_tf.translation.x.to_string());
            hud_element.sections[3].value = format!("y: {}\n", player_tf.translation.y.to_string());
            hud_element.sections[4].value = format!("ax: {}\n", player_v.acceleration.x.to_string());
            hud_element.sections[5].value = format!("ay: {}\n", player_v.acceleration.y.to_string());
        }
    }
}


pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // embed assets directly into the binary for max portability
            EmbeddedAssetPlugin::default(),
            // rng for the game
            EntropyPlugin::<WyRand>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, create_scoreboard)
        .add_systems(FixedUpdate, movement::position_update_system)
        .add_systems(FixedUpdate, movement::input_system)
        .add_systems(Update, movement::transform_movement_extrapolate_velocity)
        .add_systems(FixedUpdate, dot_spawn_system)
        .add_systems(Update, dot_decay_system)
        .add_systems(Update, update_scoreboard)
        // .add_systems(Update, spawn_target)
        .run();
}
