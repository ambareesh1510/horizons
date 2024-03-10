use crate::camera::MainCamera;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use bevy_egui::EguiContexts;

pub struct PegPlugin;

impl Plugin for PegPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_peg)
            .add_systems(Update, spawn_ball)
            .add_systems(Update, delete_all_pegs_and_balls);
    }
}


#[derive(Component)]
struct Peg;

#[derive(Component)]
struct Ball;


fn spawn_peg(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Left) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("peg.png"),
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y, 0.),
                        scale: Vec3::new(0.3, 0.3, 1.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Peg)
                .insert(RigidBody::Fixed)
                .insert(Collider::ball(25.))
                .insert(Restitution {
                    coefficient: 0.7,
                    combine_rule: CoefficientCombineRule::Max,
                });
        }
    }
}

fn spawn_ball(
    input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    primary_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (camera, camera_transform) = primary_camera.single();
    if input.just_pressed(MouseButton::Right) && !contexts.ctx_mut().wants_pointer_input() {
        if let Some(position) = primary_window
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            commands
                .spawn(SpriteBundle {
                    texture: asset_server.load("peg.png"),
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y, 0.),
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
    }
}

fn delete_all_pegs_and_balls(
    input: Res<ButtonInput<KeyCode>>,
    query_pegs_and_balls: Query<Entity, Or<(With<Peg>, With<Ball>)>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for e in query_pegs_and_balls.iter() {
            commands.entity(e).despawn();
        }
    }
}
