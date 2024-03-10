use crate::camera::MainCamera;
use crate::ui::ui;
use bevy::{prelude::*, window::PrimaryWindow};
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
