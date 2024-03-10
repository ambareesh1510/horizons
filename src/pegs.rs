use crate::camera::MainCamera;
use crate::ui::ui;
use bevy::{asset, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_egui::EguiContexts;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct PegPlugin;

impl Plugin for PegPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SceneObjects { objects: HashMap::new(), object_count: 0 })
            .add_event::<SpawnObject>()
            .add_systems(Update, spawn_object)
            .add_systems(Update, spawn_peg.after(ui))
            .add_systems(Update, spawn_ball.after(ui))
            .add_systems(Update, spawn_ball_spawner.after(ui))
            .add_systems(Update, cleanup_sounds)
            .add_systems(Startup, setup_sound)
            .add_systems(Update, display_events)
            .add_systems(Update, delete_all_pegs_and_balls);
    
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct SceneObjects {
    pub objects: HashMap<u32, Object>,
    object_count: u32
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub enum Object {
    Peg(f32, f32),
    Ball(f32, f32),
    BallSpawner(f32, f32),
}

#[derive(Event)]
pub struct SpawnObject(pub Object);

#[derive(Component)]
struct Peg;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Debug, PartialEq, Eq)]
enum Notes {
    C3,
    CS3,
    D3,
    DS3,
    E3,
    F3,
    FS3,
    G3,
    GS3,
    A3,
    AS3,
    B3,
    C4,
    CS4,
    D4,
    DS4,
    E4,
    F4,
    FS4,
    G4,
    GS4,
    A4,
    AS4,
    B4,
    C5,
}

impl Notes {
    fn to_file_path(&self) -> String {
        match self {
            Notes::C3 => "sounds/c3.ogg",
            Notes::CS3 => "sounds/cs3.ogg",
            Notes::D3 => "sounds/d3.ogg",
            Notes::DS3 => "sounds/ds3.ogg",
            Notes::E3 => "sounds/e3.ogg",
            Notes::F3 => "sounds/f3.ogg",
            Notes::FS3 => "sounds/fs3.ogg",
            Notes::G3 => "sounds/g3.ogg",
            Notes::GS3 => "sounds/gs3.ogg",
            Notes::A3 => "sounds/a3.ogg",
            Notes::AS3 => "sounds/as3.ogg",
            Notes::B3 => "sounds/b3.ogg",
            Notes::C4 => "sounds/c4.ogg",
            Notes::CS4 => "sounds/cs4.ogg",
            Notes::D4 => "sounds/d4.ogg",
            Notes::DS4 => "sounds/ds4.ogg",
            Notes::E4 => "sounds/e4.ogg",
            Notes::F4 => "sounds/f4.ogg",
            Notes::FS4 => "sounds/fs4.ogg",
            Notes::G4 => "sounds/g4.ogg",
            Notes::GS4 => "sounds/gs4.ogg",
            Notes::A4 => "sounds/a4.ogg",
            Notes::AS4 => "sounds/as4.ogg",
            Notes::B4 => "sounds/b4.ogg",
            Notes::C5 => "sounds/c5.ogg",
        }
        .into()
    }
}
fn setup_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/c3.ogg"),
        ..default()
    });
}

fn cleanup_sounds(mut commands: Commands, controller: Query<(Entity, &AudioSink)>) {
    for (id, sink) in controller.iter() {
        if sink.empty() {
            commands.entity(id).despawn();
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
    peg_query: Query<&Notes>,
    asset_server: Res<AssetServer>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flags) => {
                println!("asdf");
                let note;
                match peg_query.get(*e1) {
                    Ok(n) => note = n,
                    Err(_e) => match peg_query.get(*e2) {
                        Ok(n) => note = n,
                        Err(e) => {
                            println!("note not played {:?}", e);
                            return;
                        }
                    },
                }
                commands.spawn(AudioBundle {
                    source: asset_server.load(note.to_file_path()),
                    ..default()
                });
            }
            CollisionEvent::Stopped(e1, e2, flags) => {}
        }
        // if let Ok(Notes::C3) = peg_query.get_component::<Notes>(collision_event.collider1_entity) {
        //     commands.spawn(
        //         AudioBundle {
        //             source: asset_server.load("sounds/c3.ogg"),
        //             ..default()
        //         }
        //     );
        // }
    }
    

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
#[derive(Component)]
pub struct BallSpawner;

#[derive(Component)]
struct ObjectId(u32);

fn spawn_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_objects: ResMut<SceneObjects>,
    mut spawn_events: EventReader<SpawnObject>,
) {
    for ev in spawn_events.read() {
        match ev.0 {
            Object::Peg(x, y) => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("peg.png"),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Peg)
                    .insert(ObjectId(scene_objects.object_count))
                    .insert(RigidBody::Fixed)
                    .insert(Collider::ball(25.))
                    .insert(Notes::C3)
                    .insert(Restitution {
                        coefficient: 0.7,
                        combine_rule: CoefficientCombineRule::Max,
                    });
                scene_objects.object_count += 1;
                let object_count = scene_objects.object_count;
                scene_objects.objects.insert(object_count, ev.0.clone());
            }
            Object::Ball(x, y) => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("peg.png"),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Ball)
                    .insert(GravityScale(2.0))
                    .insert(RigidBody::Dynamic)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Collider::ball(25.));
            }
            Object::BallSpawner(x, y) => {
                commands
                    .spawn(SpriteBundle {
                        texture: asset_server.load("spawner.png"),
                        transform: Transform {
                            translation: Vec3::new(x, y, 0.),
                            scale: Vec3::new(0.3, 0.3, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(BallSpawner)
                    .insert(ObjectId(scene_objects.object_count));
                scene_objects.object_count += 1;
                let object_count = scene_objects.object_count;
                scene_objects.objects.insert(object_count, ev.0.clone());
            }
        }
    }
}

fn spawn_peg(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Left) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_event_writer.send(SpawnObject(Object::Peg(position.x, position.y)));
        }
    }
}

fn spawn_ball(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Right) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_event_writer.send(SpawnObject(Object::Ball(position.x, position.y)));
        }
    }
}

fn spawn_ball_spawner(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut spawn_event_writer: EventWriter<SpawnObject>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Middle) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_event_writer.send(SpawnObject(Object::BallSpawner(position.x, position.y)));
        }
    }
}

fn delete_all_pegs_and_balls(
    input: Res<ButtonInput<KeyCode>>,
    query_pegs_and_balls: Query<Entity, Or<(With<Peg>, With<Ball>, With<BallSpawner>)>>,
    mut scene_objects: ResMut<SceneObjects>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for e in query_pegs_and_balls.iter() {
            commands.entity(e).despawn();
        }
        scene_objects.objects.clear();
        scene_objects.object_count = 0;
    }
}



